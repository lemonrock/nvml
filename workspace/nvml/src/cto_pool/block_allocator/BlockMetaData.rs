// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
#[repr(C)]
pub(crate) struct BlockMetaData<B: Block>
{
	chain_length_and_bag_stripe_index: AtomicChainLengthAndBagStripeIndex,
	next: AtomicBlockPointer<B>,
	previous: AtomicBlockPointer<B>,
	
	next_chain: Cell<BlockPointer<B>>,
}

impl<B: Block> BlockMetaData<B>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			chain_length_and_bag_stripe_index: AtomicChainLengthAndBagStripeIndex::default(),
			next: AtomicBlockPointer::default(),
			previous: AtomicBlockPointer::default(),
			
			next_chain: Cell::new(BlockPointer::default()),
		}
	}
}

impl<B: Block> BlockMetaData<B>
{
	// Part of Drop logic for Chains struct.
	#[inline(always)]
	pub(crate) fn recycle_chains_into_block_allocator(&self, block_allocator: &BlockAllocator<B>, our_block_pointer: BlockPointer<B>)
	{
		let next_chain = self.get_next_chain();
		if next_chain.is_not_null()
		{
			let next_chain_block_meta_data = block_allocator.block_meta_data_unchecked(next_chain);
			next_chain_block_meta_data.recycle_chains_into_block_allocator(block_allocator, next_chain);
		}
		block_allocator.receive_solitary_chain_back(our_block_pointer)
	}
	
	// IDEA: Think of this chain as a chocolate bar made out of chunks.
	// The chunks are 'blocks'.
	// We don't need the whole bar, so we're going to 'snap off' one or more chunks (blocks) from the end of the bar (looking sideways on, the right hand side of the bar).
	// This turns one chain into two chains. We return the chain containing our unwanted blocks to the block allocator. This function then returns the new, shortened chain.
	// For the snapped off chain, we know it (inclusive) start address and exclusive end address. The exclusive end address might be the start of another chain.
	// NOTE: On entry, this chain MUST have been 'taken', ie the memory must be in use, ie we have 'locked' it for use by us.
	#[inline(always)]
	pub(crate) fn snap_off_back_if_longer_than_required_capacity_and_recycle_into_block_allocator(&self, our_block_pointer: BlockPointer<B>, memory_base_pointer: NonNull<u8>, our_shorter_chain_length: ChainLength, block_allocator: &BlockAllocator<B>)
	{
		let our_chain_length = self.chain_length();
		
		debug_assert!(&our_chain_length > &our_shorter_chain_length, "our_shorter_chain_length '{:?}' is equal to or greater than self.chain_length() '{:?}'", our_shorter_chain_length, our_chain_length);
		
		let snapped_off_chain_length = our_chain_length.subtract(our_shorter_chain_length);
		self.acquire(our_shorter_chain_length);
		
		let snapped_off_chain_block_pointer = BlockPointer::block_address_to_block_pointer(memory_base_pointer, our_block_pointer.subsequent_chain_start_address(memory_base_pointer, our_shorter_chain_length));
		let snapped_off_chain_block_meta_data = block_allocator.block_meta_data_unchecked(snapped_off_chain_block_pointer);
		snapped_off_chain_block_meta_data.acquire(snapped_off_chain_length);
		block_allocator.receive_solitary_chain_back(snapped_off_chain_block_pointer);
	}
	
	#[inline(always)]
	fn reset_before_add_to_bag(&self)
	{
		debug_assert!(self.chain_length_and_bag_stripe_index().bag_stripe_index().is_none(), "can not ask for reset_before_add_to_bag when in a bag");
		
		self.next.set_relaxed(BlockPointer::Null);
		self.previous.set_relaxed(BlockPointer::Null);
		self.next_chain.set(BlockPointer::Null);
	}
	
	// Valid only if not in a bag.
	// Is permitted to return a null BlockPointer.
	#[inline(always)]
	fn get_next_chain(&self) -> BlockPointer<B>
	{
		debug_assert!(self.chain_length_and_bag_stripe_index().bag_stripe_index().is_none(), "can not ask for next_chain when in a bag");
		self.next_chain.get()
	}
	
	// Valid only if not in a bag.
	// Is permitted to accept a null BlockPointer.
	#[inline(always)]
	fn set_next_chain(&self, new_next_chain: BlockPointer<B>)
	{
		debug_assert!(self.chain_length_and_bag_stripe_index().bag_stripe_index().is_none(), "can not ask for next_chain when in a bag");
		self.next_chain.set(new_next_chain);
		self.persist()
	}
	
	// Valid only if not in a bag.
	#[inline(always)]
	fn chain_length(&self) -> ChainLength
	{
		let chain_length_and_bag_stripe_index = self.chain_length_and_bag_stripe_index();
		
		debug_assert!(chain_length_and_bag_stripe_index.bag_stripe_index().is_none(), "can not ask for chain_length when in a bag");
		
		chain_length_and_bag_stripe_index.chain_length()
	}
	
	#[inline(always)]
	fn chain_length_and_bag_stripe_index(&self) -> ChainLengthAndBagStripeIndex
	{
		self.chain_length_and_bag_stripe_index.get()
	}
	
	#[inline(always)]
	fn release(&self, chain_length: ChainLength, next_bag_stripe_index: BagStripeIndex)
	{
		self.chain_length_and_bag_stripe_index.set(ChainLengthAndBagStripeIndex::new(chain_length, Some(next_bag_stripe_index)));
		self.persist()
	}
	
	#[inline(always)]
	fn acquire(&self, chain_length: ChainLength)
	{
		self.chain_length_and_bag_stripe_index.set(ChainLengthAndBagStripeIndex::new(chain_length, None));
		self.persist()
	}
	
	#[inline(always)]
	fn get_next(&self) -> BlockPointer<B>
	{
		self.next.get()
	}
	
	#[inline(always)]
	fn set_next(&self, new_next: BlockPointer<B>)
	{
		self.next.set(new_next);
	}
	
	#[inline(always)]
	fn get_previous(&self) -> BlockPointer<B>
	{
		self.previous.get()
	}
	
	#[inline(always)]
	fn set_previous(&self, new_previous: BlockPointer<B>)
	{
		self.previous.set(new_previous);
	}
	
	#[inline(always)]
	fn persist(&self)
	{
		B::P::flush_struct(self);
	}
}
