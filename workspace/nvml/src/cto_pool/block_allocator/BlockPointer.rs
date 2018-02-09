// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A compressed pointer, representing an index.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct BlockPointer<B: Block>(u32, PhantomData<B>);

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
	
	const ExclusiveMaximumBlockPointer: usize = Self::NullSentinel as usize;
	
	#[inline(always)]
	pub(crate) fn subsequent_chain_start_address(&self, memory_base_pointer: NonNull<u8>, chain_length: ChainLength) -> NonNull<u8>
	{
		self.expand_to_pointer_to_memory_unchecked(memory_base_pointer).offset(chain_length.bytes::<B>())
	}
	
	#[inline(always)]
	pub(crate) fn block_address_to_block_pointer(memory_base_pointer: NonNull<u8>, block_address: NonNull<u8>) -> Self
	{
		debug_assert!(block_address.as_ptr() >= memory_base_pointer.as_ptr(), "block_address can not be less than memory_base_pointer");
		
		let difference = memory_base_pointer.difference(block_address);
		debug_assert_eq!(difference % B::BlockSizeInBytes, 0, "difference must be a multiple of BlockSizeInBytes");
		
		debug_assert_ne!(difference, Self::NullSentinel as usize, "difference can not be the NullSentinel");
		
		let index = difference / B::BlockSizeInBytes;
		debug_assert!(index < Self::ExclusiveMaximumBlockPointer, "index must be less than the ExclusiveMaximumBlockPointer, {}", Self::ExclusiveMaximumBlockPointer);
		
		BlockPointer::new(index as u32)
	}
	
	#[inline(always)]
	pub(crate) fn expand_to_pointer_to_memory_unchecked(self, memory_base_pointer: NonNull<u8>) -> NonNull<u8>
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		
		memory_base_pointer.offset(B::BlockSizeInBytes * (self.0 as usize))
	}
	
	#[inline(always)]
	pub(crate) fn expand_to_pointer_to_meta_data(self, block_meta_data_items: &BlockMetaDataItems<B>) -> Option<&BlockMetaData<B>>
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
	pub(crate) fn expand_to_pointer_to_meta_data_unchecked(self, block_meta_data_items: &BlockMetaDataItems<B>) -> &BlockMetaData<B>
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		
		block_meta_data_items.get_unchecked(self.0 as usize)
	}
	
	#[inline(always)]
	pub(crate) fn expand_to_pointer_to_meta_data_raw(self, block_meta_data_items: &BlockMetaDataItems<B>) -> Option<NonNull<BlockMetaData<B>>>
	{
		if self.is_null()
		{
			None
		}
		else
		{
			Some(self.expand_to_pointer_to_meta_data_raw_unchecked(block_meta_data_items))
		}
	}
	
	#[inline(always)]
	pub(crate) fn expand_to_pointer_to_meta_data_raw_unchecked(self, block_meta_data_items: &BlockMetaDataItems<B>) -> NonNull<BlockMetaData<B>>
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		
		block_meta_data_items.get_unchecked_raw(self.0 as usize)
	}
	
	#[inline(always)]
	pub(crate) fn equals(self, other: Self) -> bool
	{
		self.0 == other.0
	}
	
	#[inline(always)]
	pub(crate) fn does_not_equal(self, other: Self) -> bool
	{
		self.0 != other.0
	}
	
	#[inline(always)]
	pub(crate) fn is_not_null(self) -> bool
	{
		self.does_not_equal(Self::Null)
	}
	
	#[inline(always)]
	pub(crate) fn is_null(self) -> bool
	{
		self.equals(Self::Null)
	}
	
	#[inline(always)]
	fn new(value: u32) -> Self
	{
		BlockPointer(value, PhantomData)
	}
}
