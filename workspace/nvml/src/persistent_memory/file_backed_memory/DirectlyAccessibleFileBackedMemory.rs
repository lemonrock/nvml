// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Directly accessible (`DAX`) persistent memory, eg non-volatile DIMMs.
#[derive(Debug, Clone)]
pub struct DirectlyAccessibleFileBackedMemory
{
	address: *mut c_void,
	file_backed_memory_drop_wrapper: Arc<FileBackedMemoryDropWrapper>,
}

unsafe impl Send for DirectlyAccessibleFileBackedMemory
{
}

unsafe impl Sync for DirectlyAccessibleFileBackedMemory
{
}

impl FileBackedMemory for DirectlyAccessibleFileBackedMemory
{
	// For x86-64 (4Kb for msync).
	const Alignment: usize = 64;
	
	const IsPersistent: bool = true;
	
	const SupportsExclusiveOpen: bool = false;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _open_flags(_exclusive: bool) -> FileBackedMemoryOpenFlags
	{
		FileBackedMemoryOpenFlags::None
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _finish_mapping_if_memory_is_of_correct_type(is_persistent_memory: bool, is_persistent_memory_that_supports_flushing_with_persist: bool) -> bool
	{
		is_persistent_memory && is_persistent_memory_that_supports_flushing_with_persist
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(address: *mut c_void, mapped_length: usize) -> Self
	{
		Self
		{
			address,
			file_backed_memory_drop_wrapper: FileBackedMemoryDropWrapper::new(address, mapped_length)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _address(&self) -> *mut c_void
	{
		self.address
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _mapped_length(&self) -> usize
	{
		self.file_backed_memory_drop_wrapper.mapped_length
	}
}

impl DirectlyAccessibleFileBackedMemory
{
	/// offset and length will be adjusted to cache line size granularity
	#[inline(always)]
	pub fn persist_quickly_at_cache_line_granularity(&self, offset: usize, length: usize)
	{
		debug_assert!(offset + length <= self._mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mapped_length());
		
		self._offset(offset).persist(length);
	}
	
	/// First 'half' of persist_quickly_at_cache_line_granularity
	#[inline(always)]
	pub fn flush(&self, offset: usize, length: usize)
	{
		debug_assert!(offset + length <= self._mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mapped_length());
		
		self._offset(offset).flush(length);
	}
	
	/// Second 'half' of persist_quickly_at_cache_line_granularity
	#[inline(always)]
	pub fn drain_after_flush()
	{
		unsafe { pmem_drain() }
	}
	
	/// Returns a `PersistOnDrop` based on a specific `offset` into this memory.
	#[inline(always)]
	pub fn persist_on_drop_from<'a>(&'a self, offset: usize) -> PersistOnDrop<'a>
	{
		PersistOnDrop(self._offset(offset), PhantomData)
	}
	
	/// Returns a `PersistOnDrop` based on an `offset` of zero this memory.
	#[inline(always)]
	pub fn persist_on_drop<'a>(&'a self) -> PersistOnDrop<'a>
	{
		PersistOnDrop(self.address, PhantomData)
	}
	
	/// aka 'memmove' in C.
	#[inline(always)]
	pub fn copy_then_persist_quickly_at_cache_line_granularity(&self, offset: usize, length: usize, from: *const c_void)
	{
		debug_assert!(offset + length <= self._mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mapped_length());
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memmove_persist(self._offset(offset), from, length) };
	}
	
	/// aka 'memcpy' in C.
	#[inline(always)]
	pub fn copy_nonoverlapping_then_persist_quickly_at_cache_line_granularity(&self, offset: usize, length: usize, from: *const c_void)
	{
		debug_assert!(offset + length <= self._mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mapped_length());
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memcpy_persist(self._offset(offset), from, length) };
	}
	
	/// aka 'memset' in C.
	#[inline(always)]
	pub fn write_bytes_then_persist_quickly_at_cache_line_granularity(&self, offset: usize, count: usize, value: u8)
	{
		debug_assert!(offset + count <= self._mapped_length(), "offset '{}' + count '{}' is greater than mapped length '{}'", offset, count, self._mapped_length());
		
		unsafe { pmem_memset_persist(self._offset(offset), value as i32, count) };
	}
}
