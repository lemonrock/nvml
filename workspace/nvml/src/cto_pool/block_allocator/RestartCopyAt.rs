// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RestartCopyAt<'chains, B: 'chains + Block>
{
	chain: Option<&'chains mut Chain<B>>,
	offset: usize,
}

impl<'chains, B: Block> RestartCopyAt<'chains, B>
{
	#[inline(always)]
	pub fn copy_bytes_into_chains_start(head_of_chains_linked_list: &'chains mut Chain<B>, copy_from_address: *mut u8, copy_from_length: usize) -> Self
	{
		if copy_from_length == 0
		{
			return Some(Self
			{
				chain: head_of_chains_linked_list,
				offset: 0,
			});
		}
		
		head_of_chains_linked_list.copy_bytes_into_chains_offset_is_zero(copy_from_address, copy_from_length)
	}
	
	#[inline(always)]
	pub fn copy_bytes_into_chains(self, copy_from_address: *mut u8, copy_from_length: usize) -> Self
	{
		debug_assert_ne!(self.offset, self.chain.unwrap().length(), "offset should never be the chain length");
		debug_assert!(!copy_from_address.is_null(), "copy_from_address should never be null");
		
		if copy_from_length == 0
		{
			return self;
		}
		
		let chain = self.next_chain_mut_or_panic();
		
		if self.offset == 0
		{
			chain.copy_bytes_into_chains_offset_is_zero(copy_from_address, copy_from_length)
		}
		else
		{
			chain.copy_bytes_into_chains_offset::<'chains>(copy_from_address, copy_from_length, self.offset)
		}
	}
	
	#[inline(always)]
	pub fn copy_bytes_from_chains_start(head_of_chains_linked_list: &'chains mut Chain<B>, copy_into_address: *mut c_void, copy_into_length: usize) -> Self
	{
		if copy_from_length == 0
		{
			return Some(Self
			{
				chain: head_of_chains_linked_list,
				offset: 0,
			});
		}
		
		head_of_chains_linked_list.copy_bytes_from_chains_offset_is_zero(copy_into_address, copy_into_length)
	}
	
	#[inline(always)]
	pub fn copy_bytes_from_chains(self, copy_into_address: *mut c_void, copy_into_length: usize) -> Self
	{
		debug_assert_ne!(self.offset, self.chain.unwrap().length(), "offset should never be the chain length");
		debug_assert!(!copy_into_address.is_null(), "copy_into_address should never be null");
		
		if copy_into_length == 0
		{
			return self;
		}
		
		let chain = self.next_chain_mut_or_panic();
		
		if self.offset == 0
		{
			chain.copy_bytes_from_chains_offset_is_zero(copy_into_address, copy_into_length)
		}
		else
		{
			chain.copy_bytes_from_chains_offset::<'chains>(copy_into_address, copy_into_length, self.offset)
		}
	}
}
