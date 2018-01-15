// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
pub struct Chains<'chains, B: 'chains + Block>
{
	block_allocator: CtoArc<BlockAllocator>,
	linked_list_of_chains: &'chains mut Chain<B>,
}

impl<'chains, B: Block> Drop for Chains<'chains, B>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.linked_list_of_chains.recycle_chains_into_block_allocator(self.block_allocator.as_ref());
	}
}

impl<'chains, B: Block> CtoSafe for Chains<'chains, B>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, _cto_pool_arc: &CtoPoolArc)
	{
	}
}

impl<'chains, B: Block> Chains<'chains, B>
{
	#[inline(always)]
	fn new(block_allocator: &CtoArc<BlockAllocator>, linked_list_of_chains: &'chains Chain<B>) -> Self
	{
		Self
		{
			block_allocator: block_allocator.clone(),
			linked_list_of_chains,
		}
	}
	
	#[inline(always)]
	pub fn copy_bytes_into_chains_start(&mut self, copy_from_address: *mut u8, copy_from_length: usize) -> RestartCopyAt<'chains, B>
	{
		RestartCopyAt::copy_bytes_into_chains_start(self.linked_list_of_chains, copy_from_address, copy_from_length)
	}
	
	#[inline(always)]
	pub fn copy_bytes_from_chains_start(&mut self, copy_into_address: *mut c_void, copy_into_length: usize) -> RestartCopyAt<'chains, B>
	{
		RestartCopyAt::copy_bytes_from_chains_start(self.linked_list_of_chains, copy_into_address, copy_into_length)
	}
}
