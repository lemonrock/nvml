// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Persistent Memory.
pub struct BlockAllocator<B: Block>
{
	memory_base_pointer: NonNull<u8>,
	exclusive_end_address: NonNull<u8>,
	cto_pool_arc: CtoPoolArc,
	
	// A free list.
	bags: Bags<B>,
	
	// MUST be last item as it is variable-length.
	block_meta_data_items: BlockMetaDataItems<B>,
}

unsafe impl<B: Block> Send for BlockAllocator<B>
{
}

unsafe impl<B: Block> Sync for BlockAllocator<B>
{
}

impl<B: Block> Drop for BlockAllocator<B>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.cto_pool_arc.pool_pointer().free(self.memory_base_pointer.as_ptr());
		
		self.cto_pool_arc.pool_pointer().free(self);
	}
}

impl<B: Block> CtoSafe for BlockAllocator<B>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.replace(&mut self.cto_pool_arc);
	}
}

impl<B: Block> BlockAllocator<B>
{
	const CacheLineSize: usize = 64;
	
	/// block_size is a minimum of 256 and could be 512 for systems with AVX512 CPU instructions.
	pub fn new(number_of_blocks: usize, cto_pool_arc: &CtoPoolArc) -> NonNull<Self>
	{
		assert!(B::BlockSizeInBytes.is_power_of_two(), "BlockSizeInBytes must be a power of two");
		assert!(B::BlockSizeInBytes >= Self::CacheLineSize, "BlockSizeInBytes must be be equal to or greater than cache-line size");
		assert_ne!(number_of_blocks, 0, "number_of_blocks must not be zero");
		
		let maximum_block_pointer_index = number_of_blocks - 1;
		assert!(maximum_block_pointer_index < BlockPointer::ExclusiveMaximumBlockPointer, "maximum_block_pointer_index must be less than ExclusiveMaximumBlockPointer '{}'", BlockPointer::ExclusiveMaximumBlockPointer);
		
		let capacity = number_of_blocks * B::BlockSizeInBytes;
		
		let memory_base_pointer = cto_pool_arc.aligned_allocate_or_panic(B::BlockSizeInBytes, capacity);
		
		let mut this = unsafe { NonNull::new_unchecked(cto_pool_arc.aligned_allocate_or_panic(8, size_of::<Self>() + BlockMetaDataItems::size_of(number_of_blocks)).as_ptr() as *mut Self) };
		
		unsafe
		{
			write(&mut this.as_mut().memory_base_pointer, memory_base_pointer);
			write(&mut this.as_mut().exclusive_end_address, memory_base_pointer.offset(capacity));
			write(&mut this.as_mut().cto_pool_arc, cto_pool_arc.clone());
			write(&mut this.as_mut().bags, Bags::default());
			
			this.as_mut().block_meta_data_items.initialize(number_of_blocks);
		}
		
		// TODO: add blocks to bags and bag stripes.
		
		this
	}
	
	#[inline(always)]
	fn block_meta_data_unchecked(&self, block_pointer: BlockPointer<B>) -> &BlockMetaData<B>
	{
		block_pointer.expand_to_pointer_to_meta_data_unchecked(&self.block_meta_data_items)
	}
	
	#[inline(always)]
	pub(crate) fn receive_solitary_chain_back(&self, solitary_chain_block_pointer: BlockPointer<B>)
	{
		debug_assert!(solitary_chain_block_pointer.is_not_null(), "solitary_chain_block_pointer should not be null");
		let solitary_chain_block_meta_data = self.block_meta_data_unchecked(solitary_chain_block_pointer);
		
		// This loop attempts to repeatedly merge more chains onto the end of solitary_chain_block_pointer.
		// Longer chains are better.
		let mut solitary_chain_length = solitary_chain_block_meta_data.chain_length();
		while solitary_chain_length.is_less_than_inclusive_maximum()
		{
			let subsequent_chain_start_address = solitary_chain_block_pointer.subsequent_chain_start_address(self.memory_base_pointer, solitary_chain_length);
			
			if subsequent_chain_start_address.as_ptr() == self.exclusive_end_address.as_ptr()
			{
				break
			}
			
			let cut_chain_block_pointer = BlockPointer::block_address_to_block_pointer(self.memory_base_pointer, subsequent_chain_start_address);
			if self.bags.try_to_cut(&self.block_meta_data_items, cut_chain_block_pointer)
			{
				let cut_chain_block_meta_data = self.block_meta_data_unchecked(cut_chain_block_pointer);
				
				let cut_chain_length = cut_chain_block_meta_data.chain_length();
				match solitary_chain_length.add_if_maximum_length_not_exceeded(cut_chain_length)
				{
					// The newly merged combined chain length may too long.
					// Add the now unwanted cut_chain back to the bags free list.
					None =>
					{
						cut_chain_block_meta_data.reset_before_add_to_bag();
						self.bags.add(&self.block_meta_data_items, cut_chain_length, cut_chain_block_pointer);
						break
					},
					
					Some(combined_chain_length) => solitary_chain_length = combined_chain_length,
				}
				
				solitary_chain_block_meta_data.acquire(solitary_chain_length);
			}
			else
			{
				// Wasn't in the bag, or was stolen by another thread; give up trying to merge chains.
				break
			}
		}
		
		self.nothing_to_merge_with_so_add_to_free_list(solitary_chain_block_pointer, solitary_chain_block_meta_data, solitary_chain_length);
	}
	
	/// Allocate
	pub fn allocate(block_allocator: &CtoArc<Self>, requested_size: usize) -> Result<NonNull<Chains<B>>, ()>
	{
		let mut chains = Chains::new(block_allocator)?;
		
		let (number_of_blocks_required, _capacity_in_use_of_last_chain) = B::number_of_blocks_required_and_capacity_in_use_of_last_chain(requested_size);
		if number_of_blocks_required == 0
		{
			return Ok(chains)
		}
		
		// TODO: Estimate if there is enough memory left before allocating, as it makes failure faster.
		
		let mut number_of_blocks_remaining_to_find = number_of_blocks_required;
		
		let (mut head_of_chains_linked_list, chain_length) = block_allocator.grab_a_chain(number_of_blocks_remaining_to_find);
		if head_of_chains_linked_list.is_null()
		{
			drop_in_place(chains.as_ptr());
			return Err(())
		}
		unsafe { chains.as_mut().head_of_chains_linked_list = head_of_chains_linked_list };
		
		let mut previous_chain = head_of_chains_linked_list;
		number_of_blocks_remaining_to_find -= chain_length;
		while number_of_blocks_remaining_to_find != 0
		{
			let (mut next_chain, chain_length) = block_allocator.grab_a_chain(number_of_blocks_remaining_to_find);
			let mut previous_chain_block_meta_data = block_allocator.block_meta_data_unchecked(previous_chain);
			if next_chain.is_null()
			{
				// If this isn't done, then who knows what we might free in `drop()`.
				previous_chain_block_meta_data.set_next_chain(BlockPointer::Null);
				drop_in_place(chains.as_ptr());
				
				return Err(())
			}
			previous_chain_block_meta_data.set_next_chain(next_chain);
			
			previous_chain = next_chain;
			number_of_blocks_remaining_to_find -= chain_length;
		}
		
		block_allocator.block_meta_data_unchecked(previous_chain).set_next_chain(BlockPointer::Null);
		
		Ok(chains)
	}
	
	#[inline(always)]
	fn nothing_to_merge_with_so_add_to_free_list(&self, solitary_chain_block_pointer: BlockPointer<B>, solitary_chain_block_meta_data: &BlockMetaData<B>, solitary_chain_length: ChainLength)
	{
		solitary_chain_block_meta_data.reset_before_add_to_bag();
		self.bags.add(&self.block_meta_data_items, solitary_chain_length, solitary_chain_block_pointer)
	}
	
	#[inline(always)]
	fn grab_a_chain(&self, ideal_number_of_blocks: usize) -> (BlockPointer<B>, usize)
	{
		let capped_chain_length = min(ideal_number_of_blocks, InclusiveMaximumChainLength);
		
		// (1) Try to get an exactly right chain or a longer chain.
		let mut longer_chain_length = capped_chain_length;
		while longer_chain_length <= InclusiveMaximumChainLength
		{
			let chain = self.bags.remove(&self.block_meta_data_items, ChainLength::from_length(capped_chain_length));
			if chain.is_not_null()
			{
				return (chain, longer_chain_length)
			}
			
			longer_chain_length += 1;
		}
		
		// (2) Try to get a smaller exactly right chain or a smaller chain.
		let mut smaller_chain_length = capped_chain_length;
		while smaller_chain_length > 0
		{
			let chain = self.bags.remove(&self.block_meta_data_items, ChainLength::from_length(capped_chain_length));
			if chain.is_not_null()
			{
				return (chain, smaller_chain_length)
			}
			
			smaller_chain_length -=1;
		}
		
		(BlockPointer::Null, 0)
	}
}
