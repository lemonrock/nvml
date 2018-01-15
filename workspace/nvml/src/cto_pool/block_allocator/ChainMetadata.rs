// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct ChainMetadata<B: Block>
{
	pointer_to_next_chain_or_lock: PointerToNextChainOrLock<B>,
	number_of_blocks: usize,
}

impl<B: Block> ChainMetadata<B>
{
	#[inline(always)]
	fn new(number_of_blocks: usize) -> Self
	{
		Self
		{
			pointer_to_next_chain_or_lock: PointerToNextChainOrLock::Empty,
			number_of_blocks,
		}
	}
	
	#[inline(always)]
	fn next_chain<'chains>(&'chains self) -> Option<&'chains Chain<B>>
	{
		let pointer = unsafe { self.pointer_to_next_chain_or_lock.next_chain_pointer_might_be_null() };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { &*pointer })
		}
	}
	
	#[inline(always)]
	fn next_chain_mut<'chains>(&'chains mut self) -> Option<&'chains mut Chain<B>>
	{
		let pointer = unsafe { self.pointer_to_next_chain_or_lock.next_chain_pointer_might_be_null() };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { &mut *pointer })
		}
	}
	
	#[inline(always)]
	fn next_chain_pointer_might_be_null(&mut self) -> *mut Chain<B>
	{
		unsafe { self.pointer_to_next_chain_or_lock.next_chain_pointer_might_be_null() }
	}
	
	#[inline(always)]
	fn memory_for_this_chain_is_in_use(&self) -> bool
	{
		self.pointer_to_next_chain_or_lock.memory_for_this_chain_is_in_use()
	}
	
	#[inline(always)]
	fn memory_for_this_chain_is_available(&self) -> bool
	{
		self.pointer_to_next_chain_or_lock.memory_for_this_chain_is_available()
	}
	
	#[inline(always)]
	fn try_to_take(&self) -> bool
	{
		unsafe { self.pointer_to_next_chain_or_lock.try_to_take() }
	}
	
	#[inline(always)]
	fn make_available(&self)
	{
		unsafe { self.pointer_to_next_chain_or_lock.make_available() }
	}
	
	#[inline(always)]
	fn set_number_of_blocks(&mut self, number_of_blocks: usize)
	{
		self.number_of_blocks = number_of_blocks
	}
	
	#[inline(always)]
	fn number_of_blocks(&self) -> usize
	{
		self.number_of_blocks
	}
	
	#[inline(always)]
	fn capacity(&self) -> usize
	{
		self.number_of_blocks * B::BlockSizeInBytes
	}
	
	#[inline(always)]
	fn length(&self) -> usize
	{
		self.capacity() - size_of::<Self>()
	}
}
