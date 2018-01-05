// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PMEMobjpoolEx
{
	#[inline(always)]
	fn close(self);
	
	#[inline(always)]
	fn persist(self, address: *const c_void, length: usize);
	
	/// aka 'memcpy' in C
	#[inline(always)]
	fn copy_nonoverlapping_then_persist(self, address: *mut c_void, length: usize, from: *const c_void);
	
	/// aka 'memset' in C
	#[inline(always)]
	fn write_bytes_then_persist(self, address: *mut c_void, count: usize, value: u8);
}

impl PMEMobjpoolEx for *mut PMEMobjpool
{
	#[inline(always)]
	fn close(self)
	{
		unsafe { pmemobj_close(self) }
	}
	
	#[inline(always)]
	fn persist(self, address: *const c_void, length: usize)
	{
		unsafe { pmemobj_persist(self, address, length) }
	}
	
	#[inline(always)]
	fn copy_nonoverlapping_then_persist(self, address: *mut c_void, length: usize, from: *const c_void)
	{
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmemobj_memcpy_persist(self, address, from, length) };
	}
	
	#[inline(always)]
	fn write_bytes_then_persist(self, address: *mut c_void, count: usize, value: u8)
	{
		unsafe { pmemobj_memset_persist(self, address, value as i32, count) };
	}
}
