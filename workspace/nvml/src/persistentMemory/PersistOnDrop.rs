// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct PersistOnDrop<'a>(*mut c_void, PhantomData<&'a DirectlyAccessibleFileBackedMemory>);

impl<'a> Drop for PersistOnDrop<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		DirectlyAccessibleFileBackedMemory::drainAfterFlush()
	}
}

impl<'a> PersistOnDrop<'a>
{
	#[inline(always)]
	pub fn offset(&mut self, offset: usize)
	{
		self.0 = unsafe { self.0.offset(offset as isize) };
	}
	
	#[inline(always)]
	pub fn flush(&self, length: usize)
	{
		self.0.flush(length);
	}
	
	// aka 'memmove' in C
	#[inline(always)]
	pub fn copy_then_flush(&self, length: usize, from: *const c_void)
	{
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memmove_nodrain(self.0, from, length) };
	}
	
	// aka 'memcpy' in C
	#[inline(always)]
	pub fn copy_nonoverlapping_then_flush(&self, length: usize, from: *const c_void)
	{
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memcpy_nodrain(self.0, from, length) };
	}
	
	// aka 'memset' in C
	#[inline(always)]
	pub fn write_bytes_then_flush(&self, count: usize, value: u8)
	{
		unsafe { pmem_memset_nodrain(self.0, value as i32, count) };
	}
}
