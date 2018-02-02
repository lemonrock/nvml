// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub const MaximumNumberOfBlocksInAChain: usize = 2048;

/// Build a single-threaded, non-persistent example first.
pub struct BlockAllocator<B: Block>
{
	address: *mut u8,
	number_of_blocks: usize,
	capacity: usize,
	exclusive_end_address: *mut u8,
	cto_pool_arc: CtoPoolArc,
	
	// Each of those 2048 entries can have a locking, or similar, structure, eg a simplified mutex (set bit to lock, unset bit to unlock, use spinlocks as critical section is very small).
	// We can walk up or down if we don't find a chain that matches. Probably better to walk up, and 'snap-off' what we don't need. It is probably better to 'snap-off' from the front, not the end, and then look to return these to the allocator by finding ends that match them, ie the bit we don't take is from the front. When returned to the pool, this is more likely to attach to an end.
	// index in array free_list is equivalent to number of blocks
	// RcuLock allows us to read() to see if there's actually a chain. If so, try to take the RcuLock.write() (takes a mutex).
	// There are some lock-free set algorithms.
		// We need remove_any() (or remove_first())
		// And we need remove_specific() - use by the single thread that merges chains together
			// Since remove_specific() needs the Set behaviour, it can handle bad set states, potentially.
			// It's worth nothing that a Chain entry currently can fit in 128-bits - the size of an AtomicU128.
	free_list: [(Set<Chain>, RcuLock); MaximumNumberOfBlocksInAChain],
}

impl<B: Block> Drop for BlockAllocator<B>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.cto_pool_arc.pool_pointer().free(self.address);
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
	
	// The 'space' referred to is in the value of the address pointer. With blocks of size 256 bytes (or a power of two greater), there are 8 least significant bits of the pointer that are always 0 (unused).
	const MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock: usize = 256;
	
	/// New instance
	/// block_size is a minimum of 256 and should be 512 for systems with AVX512 CPU instructions.
	pub fn new(number_of_blocks: usize, cto_pool_arc: &CtoPoolArc) -> Self
	{
		#[inline(always)]
		fn is_power_of_two(value: usize) -> bool
		{
			(value != 0) && ((value & (value - 1)) == 0)
		}
		
		assert!(is_power_of_two(B::BlockSize), "block_size must be a power of two");
		assert!(B::BlockSize >= Self::CacheLineSize, "block_size must be be equal to or greater than cache-line size");
		assert!(B::BlockSize >= Self::MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock, "block_size must be be equal to or greater than MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock, {}", MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock);
		assert!(is_power_of_two(number_of_blocks), "number_of_blocks must be a power of two");
		
		let capacity = number_of_blocks * B::BlockSize;
		assert!(capacity <= ::std::isize::MAX as usize, "capacity exceeds isize::MAX, making it impossible to use pointer offsets");
		
		let address = cto_pool_arc.pool_pointer().aligned_alloc(B::BlockSize, capacity).unwrap();
		
		Self
		{
			address,
			number_of_blocks,
			capacity,
			exclusive_end_address: unsafe { address.offset(capacity as isize) },
			cto_pool_arc: cto_pool_arc.clone(),
			free_list: XXXXX,
		}
	}
	
	#[inline(always)]
	pub(crate) fn receive_solitary_chain_back(&self, solitary_chain: *mut Chain<B>)
	{
		let solitary_chain = unsafe { &mut * solitary_chain };
		
		debug_assert!(solitary_chain.next_chain_pointer_is_null(), "solitary chain has chains");
		solitary_chain.make_available();
		
		let number_of_blocks = solitary_chain.number_of_blocks();
		debug_assert_ne!(number_of_blocks, 0, "There can not be zero blocks in a chain");
		debug_assert!(number_of_blocks <= MaximumNumberOfBlocksInAChain);
		
		// This loop attempts to repeatedly merge more chains onto this one.
		// Longer chains are better.
		// We could put a limit on this loop by using a constant such as MaximumNumberOfBlocksInAChain
		loop
		{
			let subsequent_chain_start_address = solitary_chain.subsequent_chain_start_address();
			if subsequent_chain_start_address == self.exclusive_end_address
			{
				self.nothing_to_merge_with_so_add_to_free_list(solitary_chain);
				return;
			}
			else
			{
				let subsequent_chain = unsafe { &mut * (subsequent_chain_start_address as *mut Chain<B>) };
				
				if subsequent_chain.try_to_take()
				{
					solitary_chain.merge_subsequent_chain_onto_end(subsequent_chain);
				}
				else
				{
					// Happens because either it is already taken, or it was available and is now taken by another thread.
					self.nothing_to_merge_with_so_add_to_free_list(solitary_chain);
					return;
				}
			}
		}
	}
	
	#[inline(always)]
	fn nothing_to_merge_with_so_add_to_free_list(&self, solitary_chain: &mut Chain<B>)
	{
		debug_assert!(solitary_chain.memory_for_this_chain_is_available(), "This method should only be called with a chain that is available");
		
		
		
		
		xxxx;
	}
	
	
	/// Allocate
	// this: &CtoArc could be reduced to a Strong-only Arc, as there will never be weak references.
	pub fn allocate<'chains>(this: &CtoArc<Self>, requested_size: usize) -> Chains<'chains, B>
	{
		struct FreeList([SomeChains; MaximumNumberOfBlocksInAChain]);
		
		impl FreeList
		{
			#[inline(always)]
			fn entry(&self, number_of_blocks_required: usize)
			{
				debug_assert_ne!(number_of_blocks_required, 0, "can not ask for zero blocks");
				debug_assert!(number_of_blocks_required <= MaximumNumberOfBlocksInAChain, "can not ask for more than MaximumNumberOfBlocksInAChain");
				
				let free_list_index = number_of_blocks_required - 1;
				
				// An alternative to an always_increasing_sentinel is probably a spin-lock 0x00 / 0x1, or even-odd counts.
				let (number_of_chains, always_increasing_sentinel) = self.0[number_of_blocks_required];
				
				// Let's say we've locked this entry somehow.
				if yep_ok
				{
					// Get the first chain in the queue of chains.
					let locked_chain;
					
				}
			}
		}
		
		
		
		let required_size = size_of::<ChainMetadata<B>> + requested_size;
		let remainder = required_size % this.block_size;
		
		let (number_of_blocks_required, capacity_in_use_of_last_chain) = if remainder == 0
		{
			(required_size / B::BlockSize, B::BlockSize)
		}
		else
		{
			((required_size / B::BlockSize) + 1, remainder)
		};
		
		let mut linked_list_of_chains: *mut Chain<B> = null_mut();
		
		let free_list = &this.free_list;
		
		// TODO: We have to cap number_of_blocks to find to the size of the free_list
		// TODO: When out of memory this will loop forever!
		let mut number_of_blocks_remaining_to_find = number_of_blocks_required;
		while number_of_blocks_remaining_to_find != 0
		{
			let free_list_index = number_of_blocks_remaining_to_find - 1;
			
			if number_of_blocks_remaining_to_find > MaximumNumberOfBlocksInAChain
			{
				// An alternative to an always_increasing_sentinel is probably a spin-lock 0x00 / 0x1, or even-odd counts.
				let (number_of_chains, always_increasing_sentinel) = free_list[MaximumNumberOfBlocksInAChain - 1];
				
				// Let's say we've locked this entry somehow.
				if yep_ok
				{
				
				}
			}
			else
			{
			
			}
			
			
			
			let mut incrementing_free_list_index = free_list_index;
			while incrementing_free_list_index < free_list.len()
			{
				// An alternative to an always_increasing_sentinel is probably a spin-lock 0x00 / 0x1, or even-odd counts.
				let (number_of_chains, always_increasing_sentinel) = free_list[incrementing_free_list_index];
				
				// Let's say we've locked this entry somehow.
				if yep_ok
				{
					// Get the first chain in the queue of chains.
					let locked_chain;
					
					let excess_blocks = incrementing_free_list_index - free_list_index;
					if excess_blocks != 0
					{
						let number_of_blocks_to_retain = number_of_blocks_remaining_to_find;
						// FIXME: potential problem: We have a lock (above) on this entry, and we need to re-enter the lock on this entry with the snapped off chain (if the snapped off chain no of blocks == number_of_blocks_to_retain, ie exactly double the size required), ie we may need to relinquish this lock.
						locked_chain.snap_off_back_if_longer_than_required_capacity_and_recycle_into_block_allocator(number_of_blocks_to_retain, self);
					}
					
					if linked_list_of_chains.is_null()
					{
						return Chains::new(this, locked_chain);
					}
					else
					{
						unsafe { &*linked_list_of_chains }.add_chain(locked_chain);
						linked_list_of_chains =	locked_chain;
						
						// TODO: Something we should consider doing at this point is sorting the chains by address and seeing if we can merge them at all.
						
						
						return Chains::new(this, unsafe { &*linked_list_of_chains });
					}
				}
				
				incrementing_free_list_index += 1;
			}
			
			let mut decrementing_free_list_index = free_list_index;
			while decrementing_free_list_index >= 0
			{
				let (number_of_chains, always_increasing_sentinel) = self.free_list[decrementing_free_list_index];
				
				// Let's say we've locked this entry somehow.
				if yep_ok
				{
					// Get the first chain in the queue of chains.
					let locked_chain;
					
					if linked_list_of_chains.is_null()
					{
						linked_list_of_chains = locked_chain;
					}
					else
					{
						unsafe { &*linked_list_of_chains }.add_chain(locked_chain);
						linked_list_of_chains =	locked_chain;
					}
					
					break;
				}
				
				decrementing_free_list_index -= 1;
			}
		}
	}
}
