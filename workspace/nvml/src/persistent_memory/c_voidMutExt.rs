// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension trait to *mut c_void pointers to make it easier to work with persistent memory.
#[allow(non_camel_case_types)]
pub trait c_voidMutExt
{
	/// Unmap persistent memory.
	/// self can not be null.
	/// length can be zero (but then nothing happens).
	#[inline(always)]
	fn unmap(self, length: usize);
}

macro_rules! debug_assert_self_is_not_null
{
	($self: ident) =>
	{
		debug_assert!(!$self.is_null(), "self (address) can not be null");
	}
}

impl c_voidMutExt for *mut c_void
{
	#[inline(always)]
	fn unmap(self, length: usize)
	{
		debug_assert_self_is_not_null!(self);
		
		if length == 0
		{
			return;
		}
		
		let result = unsafe { pmem_unmap(self, length) };
		if likely(result == 0)
		{
			return;
		}
		else if likely(result == -1)
		{
			match errno().0
			{
				EINVAL => panic!("EINVAL for pmem_unmap() implies a bad address or length"),
				
				illegal @ _ => panic!("Error number '{}' should not occur for pmem_unmap()", illegal),
			}
		}
		else
		{
			panic!("pmem_unmap() returned value '{}', not 0 or -1", result);
		}
	}
}
