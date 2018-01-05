// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// This trait allows one to do memory writes with control over flushes.
/// When dropped, the memory is drained and so fully persisted.
/// This is better than using `persist()`, which forces a flush and drain every time.
/// Some types of memory (eg mmap, heap) do not allow flushes, and so flushes for these memory types do nothing.
pub trait PersistOnDrop<'memory>
{
	/// Returns a pointer to underlying memory.
	#[inline(always)]
	fn as_ptr(&self) -> *mut c_void;
	
	/// persist, then move forward. Can offset beyond the persistable memory region.
	#[inline(always)]
	fn offset(self, offset: usize) -> Self;
	
	/// flush memory.
	/// Useful to call after using `ptr::write()` or some other memory write.
	#[inline(always)]
	fn flush(&self, length: usize);
	
	/// copy memory then flush.
	/// Similar to 'memmove' in C.
	#[inline(always)]
	fn copy_then_flush(&self, length: usize, from: *const c_void);
	
	/// copy memory then flush.
	/// Similar to 'memcpy' in C.
	#[inline(always)]
	fn copy_nonoverlapping_then_flush(&self, length: usize, from: *const c_void);
	
	/// set memory then flush.
	/// Similar to 'memset' in C.
	#[inline(always)]
	fn write_bytes_then_flush(&self, count: usize, value: u8);
}
