// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BlockMetaDataItems<B: Block>(PhantomData<B>);

impl<B: Block> BlockMetaDataItems<B>
{
	#[inline(always)]
	fn size_of(number_of_blocks: usize) -> usize
	{
		number_of_blocks * size_of::<BlockMetaData<B>>()
	}
	
	#[inline(always)]
	unsafe fn initialize(&mut self, number_of_blocks: usize)
	{
		let mut index = 0;
		while index < number_of_blocks
		{
			let block_meta_data = self.get(index).as_mut();
			
			write(block_meta_data, BlockMetaData::default());
			
			index += 1;
		}
	}
	
	#[inline(always)]
	fn get_unchecked(&self, block_pointer: usize) -> &BlockMetaData<B>
	{
		unsafe { self.get(block_pointer).as_ref() }
	}
	
	#[inline(always)]
	unsafe fn get(&self, block_pointer: usize) -> NonNull<BlockMetaData<B>>
	{
		NonNull::new_unchecked(self as *const Self as *const u8 as *mut u8).offset(block_pointer * size_of::<BlockMetaData<B>>)
	}
}