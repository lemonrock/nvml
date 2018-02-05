// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Volatile Memory
#[derive(Debug)]
pub struct RestartCopyIntoAt<'block_meta_data, B: 'block_meta_data + Block>
{
	chain: Chain<B>,
	offset: usize,
	block_meta_data_items: &'block_meta_data BlockMetaDataItems<B>,
}

impl<'block_meta_data, B: Block> RestartCopyIntoAt<'block_meta_data, B>
{
	/// head_of_chains_linked_list can be null; any copy must then only be for zero bytes.
	#[inline(always)]
	fn new(memory_base_pointer: NonNull<u8>, head_of_chains_linked_list: BlockPointer<B>, block_meta_data_items: &'block_meta_data BlockMetaDataItems<B>) -> Self
	{
		Self
		{
			chain: Chain
			{
				memory_base_pointer,
				block_pointer: head_of_chains_linked_list,
				block_meta_data: head_of_chains_linked_list.expand_to_pointer_to_meta_data_raw(block_meta_data_items),
			},
			offset: 0,
			block_meta_data_items,
		}
	}
	
	/// Copy bytes into chains from a source, `copy_from_address`.
	#[inline(always)]
	pub fn copy_bytes_into_chains(&mut self, copy_from_address: NonNull<u8>, copy_from_length: usize)
	{
		if copy_from_length == 0
		{
			return;
		}
		
		let offset = self.offset;
		if offset == 0
		{
			self.copy_bytes_into_chains_offset_is_zero(copy_from_address, copy_from_length)
		}
		else
		{
			self.copy_bytes_into_chains_offset(copy_from_address, copy_from_length, offset)
		}
	}
	
	#[inline(always)]
	fn copy_bytes_into_chains_offset_is_zero(&mut self, copy_from_address: NonNull<u8>, copy_from_capacity: usize)
	{
		let copy_into_chain_address = self.chain.data_ptr();
		let remaining_capacity = self.chain.capacity();
		
		self._copy_bytes_into_chains_inner(copy_from_address, copy_from_capacity, copy_into_chain_address, remaining_capacity, 0)
	}
	
	#[inline(always)]
	fn copy_bytes_into_chains_offset(&mut self, copy_from_address: NonNull<u8>, copy_from_capacity: usize, offset: usize)
	{
		let copy_into_chain_address = self.chain.data_ptr_offset(offset);
		let remaining_capacity = self.chain.remaining_capacity(offset);
		
		self._copy_bytes_into_chains_inner(copy_from_address, copy_from_capacity, copy_into_chain_address, remaining_capacity, offset)
	}
	
	#[doc(hidden)]
	fn _copy_bytes_into_chains_inner(&mut self, copy_from_address: NonNull<u8>, copy_from_capacity: usize, copy_into_chain_address: NonNull<u8>, remaining_capacity: usize, offset: usize)
	{
		#[inline(always)]
		fn copy_and_flush_persistent_memory<B: Block>(copy_from_address: NonNull<u8>, copy_into_chain_address: NonNull<u8>, capacity: usize)
		{
			unsafe { copy_nonoverlapping(copy_from_address.as_ptr() as *const _, copy_into_chain_address.as_ptr(), capacity) };
			B::P::flush_memory(copy_into_chain_address.as_ptr() as *mut c_void, capacity);
		}
		
		copy_and_flush_persistent_memory::<B>(copy_from_address, copy_into_chain_address, copy_from_capacity);
		
		if copy_from_capacity <= remaining_capacity
		{
			self.restart_at_as_nothing_more_to_copy(copy_from_capacity, remaining_capacity, offset)
		}
		else
		{
			self.next_chain();
			
			let (new_copy_from_address, new_copy_from_capacity) = Self::adjust_copy_from_address_and_capacity(copy_from_address, copy_from_capacity, remaining_capacity);
			self.copy_bytes_into_chains_offset_is_zero(new_copy_from_address, new_copy_from_capacity)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn adjust_copy_from_address_and_capacity(copy_from_address: NonNull<u8>, copy_from_capacity: usize, remaining_capacity: usize) -> (NonNull<u8>, usize)
	{
		debug_assert!(copy_from_capacity >= remaining_capacity, "copy_from_capacity is less than remaining_capacity");
		
		(copy_from_address.offset(remaining_capacity), copy_from_capacity - remaining_capacity)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn restart_at_as_nothing_more_to_copy(&mut self, copy_from_capacity: usize, remaining_capacity: usize, offset: usize)
	{
		if copy_from_capacity == remaining_capacity
		{
			self.next_chain()
		}
		else
		{
			self.offset = offset + copy_from_capacity;
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn next_chain(&mut self)
	{
		self.chain.next_chain(self.block_meta_data_items);
		self.offset = 0;
	}
}
