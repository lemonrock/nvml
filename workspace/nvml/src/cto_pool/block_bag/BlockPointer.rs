// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A compressed pointer, representing an index.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BlockPointer<B: Block>(u32, PhantomData<B>);

impl<B: Block> Default for BlockPointer<B>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Null
	}
}

impl<B: Block> BlockPointer<B>
{
	const NullSentinel: u32 = ::std::u32::MAX;
	
	const Null: Self = BlockPointer(Self::NullSentinel, PhantomData);
	
	/// x
	#[inline(always)]
	pub fn expand_to_pointer_to_memory(self, memory_base_pointer: *mut u8) -> *mut u8
	{
		if self.is_null()
		{
			null_mut()
		}
		else
		{
			self.expand_to_pointer_to_memory_unchecked(memory_base_pointer)
		}
	}
	
	/// x
	#[inline(always)]
	pub fn expand_to_pointer_to_memory_unchecked(self, memory_base_pointer: *mut u8) -> *mut u8
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		
		unsafe { memory_base_pointer.offset(B::BlockSizeInBytes as isize * self.0 as isize) }
	}
	
	#[inline(always)]
	fn new(value: u32) -> Self
	{
		BlockPointer(value, PhantomData)
	}
	
	#[inline(always)]
	fn equals(self, other: Self) -> bool
	{
		self.0 == other.0
	}
	
	#[inline(always)]
	fn does_not_equal(self, other: Self) -> bool
	{
		self.0 != other.0
	}
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		self.does_not_equal(Self::Null)
	}
	
	#[inline(always)]
	fn is_null(self) -> bool
	{
		self.equals(Self::Null)
	}
	
	#[inline(always)]
	fn expand_to_pointer_to_meta_data(self, block_meta_data_items: &[BlockMetaData<B>]) -> Option<&BlockMetaData<B>>
	{
		if self.is_null()
		{
			None
		}
		else
		{
			Some(self.expand_to_pointer_to_meta_data_unchecked(block_meta_data_items))
		}
	}
	
	#[inline(always)]
	fn expand_to_pointer_to_meta_data_unchecked(self, block_meta_data_items: &[BlockMetaData<B>]) -> &BlockMetaData<B>
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		debug_assert!((self.0 as usize) < block_meta_data_items.len(), "block_meta_data_items is too short");
		
		unsafe { block_meta_data_items.get_unchecked(self.0 as usize) }
	}
}
