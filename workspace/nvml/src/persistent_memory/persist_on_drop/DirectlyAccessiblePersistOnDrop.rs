// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This struct is a simple wrapper that allows precise control over persistence.
/// Use one of `copy_then_flush`, `copy_nonoverlapping_then_flush` or `write_bytes_then_flush`.
/// These flush to the cache, but do not drain (persist) the cache.
/// When dropped, persistence (draining) occurs.
/// Use `flush()` to force a flush after some other memory operation (eg a pointer read or write).
#[derive(Debug)]
pub struct DirectlyAccessiblePersistOnDrop<'memory>(pub(crate) *mut c_void, pub(crate) PhantomData<&'memory DirectlyAccessibleFileBackedMemory>);

impl<'memory> Drop for DirectlyAccessiblePersistOnDrop<'memory>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		DirectlyAccessibleFileBackedMemory::drain_after_flush()
	}
}

impl<'memory> PersistOnDrop<'memory> for DirectlyAccessiblePersistOnDrop<'memory>
{
	#[inline(always)]
	fn as_ptr(&self) -> *mut c_void
	{
		self.0
	}
	
	#[inline(always)]
	fn offset(mut self, offset: usize) -> Self
	{
		self.0 = unsafe { self.0.offset(offset as isize) };
		self
	}
	
	#[inline(always)]
	fn flush(&self, length: usize)
	{
		self.0.flush(length)
	}
	
	/// copy memory then flush (performing a flush, but not a drain, afterwards).
	/// Similar to 'memmove' in C.
	#[inline(always)]
	fn copy_then_flush(&self, length: usize, from: *const c_void)
	{
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memmove_nodrain(self.0, from, length) };
	}
	
	/// copy memory then flush (performing a flush, but not a drain, afterwards).
	/// Similar to 'memcpy' in C.
	#[inline(always)]
	fn copy_nonoverlapping_then_flush(&self, length: usize, from: *const c_void)
	{
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memcpy_nodrain(self.0, from, length) };
	}
	
	/// set memory (performing a flush, but not a drain, afterwards).
	/// Similar to 'memset' in C.
	#[inline(always)]
	fn write_bytes_then_flush(&self, count: usize, value: u8)
	{
		unsafe { pmem_memset_nodrain(self.0, value as i32, count) };
	}
}
