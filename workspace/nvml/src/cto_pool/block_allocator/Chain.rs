// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Chain<B: Block>
{
	chain_metadata: ChainMetadata<B>,
	data: VariableLengthArray,
}

impl<B: Block> Chain<B>
{
	#[inline(always)]
	pub(crate) fn subsequent_chain_start_address(&mut self) -> *mut u8
	{
		self.data_mut_ptr_offset(self.length())
	}
	
	#[inline(always)]
	pub(crate) fn recycle_chains_into_block_allocator(&mut self, block_allocator: &BlockAllocator)
	{
		if let Some(next_chain) = self.next_chain_mut()
		{
			unsafe { write(&mut self.chain_metadata.pointer_to_next_chain_or_lock, PointerToNextChainOrLock::default()) };
			next_chain.recycle_chains_into_block_allocator(block_allocator);
		}
		block_allocator.receive_solitary_chain_back(self as *mut Self)
	}
	
	// IDEA: Think of this chain as a chocolate bar made out of chunks.
	// The chunks are 'blocks'.
	// We don't need the whole bar, so we're going to 'snap off' one or more chunks (blocks) from the end of the bar (looking sideways on, the right hand side of the bar).
	// This turns one chain into two chains. We return the chain containing our unwanted blocks to the block allocator. This function then returns the new, shortened chain.
	// For the snapped off chain, we know it (inclusive) start address and exclusive end address. The exclusive end address might be the start of another chain.
	// NOTE: On entry, this chain MUST have been 'taken', ie the memory must be in use, ie we have 'locked' it for use by us.
	#[inline(always)]
	pub(crate) fn snap_off_back_if_longer_than_required_capacity_and_recycle_into_block_allocator(&mut self, number_of_blocks_to_retain: usize, block_allocator: &BlockAllocator)
	{
		let start_address_of_snapped_off_chain = Self::snap_off_back_if_longer_than_required_and_return_snapped_off_chain_and_shortened_chain(self as *mut Self, number_of_blocks_to_retain);
		block_allocator.receive_solitary_chain_back(start_address_of_snapped_off_chain);
	}
	
	#[inline(always)]
	fn snap_off_back_if_longer_than_required_and_return_snapped_off_chain_and_shortened_chain(this: *mut Self, number_of_blocks_to_retain: usize) -> *mut Self
	{
		// Any references to method calls on self will become invalid once the first overwrite_metadata() statement is encountered.
		let number_of_blocks =
		{
			let this = unsafe { &mut * this };
			debug_assert!(this.memory_for_this_chain_is_in_use(), "We must have already taken this chain");
			debug_assert!(this.next_chain_pointer_is_null(), "Chains which are being snapped should be solitary");
			
			let number_of_blocks = this.number_of_blocks();
			
			debug_assert_ne!(number_of_blocks, 0, "self.number_of_blocks) can not be zero");
			debug_assert!(number_of_blocks <= MaximumNumberOfBlocksInAChain, "self.number_of_blocks() must be less than or equal to MaximumNumberOfBlocksInAChain");

			debug_assert_ne!(number_of_blocks_to_retain, 0, "must retain as least one block, ie number_of_blocks_to_retain can not be zero");
			debug_assert!(number_of_blocks_to_retain < number_of_blocks, "must retain less than the current self.number_of_blocks(), ie number_of_blocks_to_retain < self.number_of_blocks()");
			
			this.set_number_of_blocks(number_of_blocks_to_retain);
			
			number_of_blocks
		};
		
		let number_of_blocks_to_snap_off = number_of_blocks - number_of_blocks_to_retain;
		
		let offset_of_number_of_blocks_to_snap_off =
		{
			let length_of_number_of_blocks_to_snap_off = number_of_blocks_to_snap_off * B::BlockSize;
			debug_assert!(length_of_number_of_blocks_to_snap_off <= ::std::isize::MAX as usize, "length_of_number_of_blocks_to_snap_off exceeds maximum pointer offset");
			length_of_number_of_blocks_to_snap_off as isize
		};
		
		let start_address_of_snapped_off_chain = unsafe { (this as *mut u8).offset(offset_of_number_of_blocks_to_snap_off) as *mut Self };
		
		Self::overwrite_metadata(start_address_of_snapped_off_chain, number_of_blocks_to_snap_off);
		
		start_address_of_snapped_off_chain
	}
	
	// aka 'unsnap'
	#[inline(always)]
	pub(crate) fn merge_subsequent_chain_onto_end(&mut self, subsequent_chain: &Self)
	{
		let existing_number_of_blocks = self.number_of_blocks();
		let additional_blocks = subsequent_chain.number_of_blocks();
		self.set_number_of_blocks(existing_number_of_blocks + additional_blocks);
	}
	
	#[inline(always)]
	fn overwrite_metadata(this: *mut Self, number_of_blocks: usize) -> Self
	{
		unsafe { write(this, Self::new(number_of_blocks)) }
	}
	
	#[inline(always)]
	fn new(number_of_blocks: usize) -> Self
	{
		Self
		{
			chain_metadata: ChainMetadata::new(number_of_blocks),
			data: VariableLengthArray::default(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn copy_bytes_into_chains_offset<'chains>(&mut self, copy_from_address: *mut u8, copy_from_length: usize, offset: usize) -> RestartCopyAt<'chains, B>
	{
		let copy_into_chain_address = self.data_mut_ptr_offset(offset);
		let remaining_length = self.remaining_length(offset);
		self._copy_bytes_into_chains_inner::<'chains>(copy_from_address, copy_from_length, copy_into_chain_address, remaining_length, offset)
	}
	
	#[inline(always)]
	pub(crate) fn copy_bytes_into_chains_offset_is_zero<'chains>(&mut self, copy_from_address: *mut u8, copy_from_length: usize) -> RestartCopyAt<'chains, B>
	{
		let copy_into_chain_address = self.data_mut_ptr();
		let remaining_length = self.length();
		self._copy_bytes_into_chains_inner::<'chains>(copy_from_address, copy_from_length, copy_into_chain_address, remaining_length, 0)
	}
	
	fn _copy_bytes_into_chains_inner<'chains>(&mut self, copy_from_address: *mut u8, copy_from_length: usize, copy_into_chain_address: *mut u8, remaining_length: usize, offset: usize) -> RestartCopyAt<'chains, B>
	{
		#[inline(always)]
		fn copy_and_flush(copy_from_address: *mut u8, copy_into_chain_address: *mut u8, length: usize)
		{
			unsafe { copy_nonoverlapping(copy_from_address, copy_into_chain_address, length) };
			B::P::flush(copy_into_chain_address, length);
		}
		
		if copy_from_length <= remaining_length
		{
			copy_and_flush(copy_from_address, copy_into_chain_address, copy_from_length);
			
			self.restart_copy_at_if_nothing_more_to_copy::<'chains>(copy_from_length, remaining_length, offset)
		}
		else
		{
			copy_and_flush(copy_from_address, copy_into_chain_address, remaining_length);
			
			let (new_copy_from_address, new_copy_from_length) = Self::adjust_copy_address_and_length(copy_from_address, copy_from_length, remaining_length);
			self.next_chain_mut_or_panic().copy_bytes_into_chains_offset_is_zero::<'chains>(new_copy_from_address, new_copy_from_length)
		}
	}
	
	#[inline(always)]
	pub(crate) fn copy_bytes_from_chains_offset<'chains>(&mut self, copy_into_address: *mut u8, copy_into_length: usize, offset: usize) -> RestartCopyAt<'chains, B>
	{
		let copy_from_chain_address = self.data_mut_ptr_offset(offset);
		let remaining_length = self.remaining_length(offset);
		self._copy_bytes_from_chains_inner::<'chains>(copy_into_address, copy_into_length, copy_from_chain_address, remaining_length, offset)
	}
	
	#[inline(always)]
	pub(crate) fn copy_bytes_from_chains_offset_is_zero<'chains>(&mut self, copy_into_address: *mut u8, copy_into_length: usize) -> RestartCopyAt<'chains, B>
	{
		let copy_from_chain_address = self.data_mut_ptr();
		let remaining_length = self.length();
		self._copy_bytes_from_chains_inner::<'chains>(copy_into_address, copy_into_length, copy_from_chain_address, remaining_length, 0)
	}
	
	fn _copy_bytes_from_chains_inner<'chains>(&mut self, copy_into_address: *mut c_void, copy_into_length: usize, copy_from_chain_address: *mut u8, remaining_length: usize, offset: usize) -> RestartCopyAt<'chains, B>
	{
		#[inline(always)]
		fn copy(copy_from_chain_address: *mut u8, copy_into_address: *mut u8, length: usize)
		{
			unsafe { copy_nonoverlapping(copy_from_chain_address, copy_into_address, length) };
		}
		
		if copy_into_length <= remaining_length
		{
			copy(copy_from_chain_address, copy_into_address, copy_into_length);
			
			self.restart_copy_at_if_nothing_more_to_copy::<'chains>(copy_into_length, remaining_length, offset)
		}
		else
		{
			copy(copy_from_chain_address, copy_into_address, remaining_length);
			
			let (new_copy_into_address, new_copy_into_length) = Self::adjust_copy_address_and_length(copy_into_address, copy_into_length, remaining_length);
			self.next_chain_mut_or_panic().copy_bytes_from_chains_offset_is_zero::<'chains>(new_copy_into_address, new_copy_into_length)
		}
	}
	
	#[inline(always)]
	fn adjust_copy_address_and_length(copy_address: *mut u8, copy_length: usize, remaining_length: usize) -> (*mut u8, usize)
	{
		(copy_address + remaining_length, copy_length - remaining_length)
	}
	
	#[inline(always)]
	fn next_chain_mut_or_panic<'chains>(&'chains mut self) -> &'chains mut Self
	{
		self.next_chain_mut::<'chains>().expect("Exceeded length of chains")
	}
	
	#[inline(always)]
	fn next_chain<'chains>(&'chains self) -> Option<&'chains Self>
	{
		self.chain_metadata.next_chain()
	}
	
	#[inline(always)]
	fn next_chain_mut<'chains>(&'chains mut self) -> Option<&'chains mut Self>
	{
		self.chain_metadata.next_chain_mut()
	}
	
	#[inline(always)]
	fn next_chain_pointer_might_be_null(&mut self) -> *mut Self
	{
		self.chain_metadata.next_chain_ptr()
	}
	
	#[inline(always)]
	fn next_chain_pointer_is_null(&mut self) -> *mut Self
	{
		self.next_chain_pointer_might_be_null().is_null()
	}
	
	#[inline(always)]
	fn memory_for_this_chain_is_in_use(&self) -> bool
	{
		self.chain_metadata.memory_for_this_chain_is_in_use()
	}
	
	#[inline(always)]
	fn memory_for_this_chain_is_available(&self) -> bool
	{
		self.chain_metadata.memory_for_this_chain_is_available()
	}
	
	#[inline(always)]
	fn try_to_take(&self) -> bool
	{
		self.chain_metadata.try_to_take()
	}
	
	#[inline(always)]
	fn make_available(&self)
	{
		self.chain_metadata.make_available()
	}
	
	#[inline(always)]
	fn set_number_of_blocks(&mut self, number_of_blocks: usize)
	{
		self.chain_metadata.set_number_of_blocks(number_of_blocks)
	}
	
	#[inline(always)]
	fn number_of_blocks(&self) -> usize
	{
		self.chain_metadata.number_of_blocks()
	}
	
	#[inline(always)]
	fn capacity(&self) -> usize
	{
		self.chain_metadata.capacity()
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.chain_metadata.length()
	}
	
	#[inline(always)]
	fn remaining_length(&self, offset: usize) -> usize
	{
		let length = self.length();
		debug_assert!(offset <= length, "offset exceeds length");
		length - offset
	}
	
	#[inline(always)]
	fn data_ptr(&self) -> *const u8
	{
		unsafe { self.data.as_ptr() }
	}
	
	#[inline(always)]
	fn data_mut_ptr(&self) -> *mut u8
	{
		unsafe { self.data.as_mut_ptr() }
	}
	
	#[inline(always)]
	fn data_ptr_offset(&self, offset: usize) -> *const u8
	{
		debug_assert!(offset <= ::std::isize::MAX as usize, "offset exceeds isize::MAX");
		debug_assert!(offset <= self.length(), "offset exceeds length"); // Note <=, not <, so we can get the exclusive end address.
		
		unsafe { self.data_ptr().offset(offset as isize) }
	}
	
	#[inline(always)]
	fn data_mut_ptr_offset(&self, offset: usize) -> *mut u8
	{
		debug_assert!(offset <= ::std::isize::MAX as usize, "offset exceeds isize::MAX");
		debug_assert!(offset <= self.length(), "offset exceeds length"); // Note <=, not <, so we can get the exclusive end address.
		
		unsafe { self.data_mut_ptr().offset(offset as isize) }
	}
	
	#[inline(always)]
	fn data_as_slice(&self) -> &[u8]
	{
		unsafe { self.data.as_slice(self.length()) }
	}
	
	#[inline(always)]
	fn data_as_mut_slice(&self, length: usize) -> &mut [u8]
	{
		unsafe { self.data.as_mut_slice(self.length()) }
	}
	
	#[inline(always)]
	fn restart_copy_at_if_nothing_more_to_copy<'chains>(&mut self, copy_length: usize, remaining_length: usize, offset: usize) -> RestartCopyAt<'chains, B>
	{
		if copy_length == remaining_length
		{
			RestartCopyAt
			{
				chain: self.next_chain_mut::<'chains>(),
				offset: 0,
			}
		}
		else
		{
			RestartCopyAt
			{
				chain: Some(self),
				offset: offset + copy_length,
			}
		}
	}
}

