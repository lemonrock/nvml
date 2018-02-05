// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Volatile Memory
#[derive(Debug)]
pub(crate) struct Chain<B: Block>
{
	memory_base_pointer: NonNull<u8>,
	block_pointer: BlockPointer<B>,
	block_meta_data: Option<NonNull<BlockMetaData<B>>>,
}

impl<B: Block> Chain<B>
{
	#[inline(always)]
	pub(crate) fn next_chain(&mut self, block_meta_data_items: &BlockMetaDataItems<B>)
	{
		let next_chain = self.get_next_chain();
		self.block_pointer = next_chain;
		self.block_meta_data = next_chain.expand_to_pointer_to_meta_data_raw(block_meta_data_items);
	}
	
	#[inline(always)]
	pub(crate) fn remaining_capacity(&self, offset: usize) -> usize
	{
		let capacity = self.capacity();
		debug_assert!(offset <= capacity, "offset exceeds capacity");
		capacity - offset
	}
	
	#[inline(always)]
	pub(crate) fn capacity(&self) -> usize
	{
		self.chain_length().as_capacity::<B>()
	}
	
	#[inline(always)]
	pub(crate) fn data_ptr_offset(&self, offset: usize) -> NonNull<u8>
	{
		debug_assert!(offset <= ::std::isize::MAX as usize, "offset exceeds isize::MAX");
		debug_assert!(offset <= self.capacity(), "offset exceeds capacity"); // Note <=, not <, so we can get the exclusive end address.
		
		self.data_ptr().offset(offset)
	}
	
	#[inline(always)]
	pub(crate) fn data_ptr(&self) -> NonNull<u8>
	{
		self.block_pointer.expand_to_pointer_to_memory_unchecked(self.memory_base_pointer)
	}
	
	#[inline(always)]
	fn get_next_chain(&self) -> BlockPointer<B>
	{
		self.block_meta_data().get_next_chain()
	}
	
	#[inline(always)]
	fn chain_length(&self) -> ChainLength
	{
		self.block_meta_data().chain_length()
	}
	
	#[inline(always)]
	fn block_meta_data(&self) -> &BlockMetaData<B>
	{
		let block_meta_data = self.block_meta_data.expect("No block meta data implies a null BlockPointer for this chain, which means we've exceeded the available memory");
		block_meta_data.longer_as_ref()
	}
}
