// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// This struct is a simple wrapper that allows precise control over persistence.
/// Use one of `copy_then_flush`, `copy_nonoverlapping_then_flush` or `write_bytes_then_flush`.
/// Flushing does not occur with this kind of memory.
/// When dropped, persistence (draining) occurs.
#[derive(Debug)]
pub struct MappedPersistOnDrop<'memory>(pub(crate) *mut c_void, pub(crate) PhantomData<&'memory MappedFileBackedMemory>, pub(crate) Cell<usize>);

impl<'memory> Drop for MappedPersistOnDrop<'memory>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		#[allow(deprecated)]
		{
			self.0.persist_or_msync_regardless_of_whether_self_is_persistent_or_regular_memory(self.2.get());
		}
	}
}

impl<'memory> PersistOnDrop<'memory> for MappedPersistOnDrop<'memory>
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
	
	/// flush memory.
	/// Useful to call after using `ptr::write()` or some other memory write.
	/// Does nothing for mapped file-backed memory.
	#[inline(always)]
	fn flush(&self, length: usize)
	{
		if length > self.2.get()
		{
			self.2.set(length);
		}
	}
	
	/// copy memory then flush (flush does nothing for this kind of memory).
	/// Similar to 'memmove' in C.
	#[inline(always)]
	fn copy_then_flush(&self, length: usize, from: *const c_void)
	{
		debug_assert!(from.is_not_null(), "from must not be null");
		
		unsafe { copy(from, self.0, length) }
		self.flush(length)
	}
	
	/// copy memory then flush (flush does nothing for this kind of memory).
	/// Similar to 'memcpy' in C.
	#[inline(always)]
	fn copy_nonoverlapping_then_flush(&self, length: usize, from: *const c_void)
	{
		debug_assert!(from.is_not_null(), "from must not be null");
		
		unsafe { copy_nonoverlapping(from, self.0, length) }
		self.flush(length)
	}
	
	/// set memory then flush (flush does nothing for this kind of memory).
	/// Similar to 'memset' in C.
	#[inline(always)]
	fn write_bytes_then_flush(&self, count: usize, value: u8)
	{
		unsafe { write_bytes(self.0, value, count) }
		self.flush(count)
	}
}
