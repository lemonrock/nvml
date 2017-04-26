// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct DirectlyAccessibleFileBackedMemory
{
	address: *mut c_void,
	fileBackedMemoryDropWrapper: Arc<FileBackedMemoryDropWrapper>,
}

unsafe impl Send for DirectlyAccessibleFileBackedMemory
{
}

unsafe impl Sync for DirectlyAccessibleFileBackedMemory
{
}

impl FileBackedMemory for DirectlyAccessibleFileBackedMemory
{
	// For x86-64 (4Kb for msync)
	const Alignment: usize = 64;
	
	const IsPersistent: bool = true;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _openFlags(exclusive: bool) -> PersistentMemoryFileFlags
	{
		assert!(!exclusive, "Directly Accessible Memory (Device DaX) does not support exclusive");
		
		PersistentMemoryFileFlags::empty()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _finishMappingIfMemoryIsOfCorrectType(isPersistentMemory: bool, isPersistentMemoryThatSupportsFlushingWithPersist: bool) -> bool
	{
		isPersistentMemory && isPersistentMemoryThatSupportsFlushingWithPersist
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(address: *mut c_void, mappedLength: usize) -> Self
	{
		Self
		{
			address: address,
			fileBackedMemoryDropWrapper: FileBackedMemoryDropWrapper::new(address, mappedLength)
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
	fn _mappedLength(&self) -> usize
	{
		self.fileBackedMemoryDropWrapper.mappedLength
	}
}

impl DirectlyAccessibleFileBackedMemory
{
	/// offset and length will be adjusted to cache line size granularity
	#[inline(always)]
	pub fn persistQuicklyAtCacheLineGranularity(&self, offset: usize, length: usize)
	{
		debug_assert!(offset + length <= self._mappedLength(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mappedLength());
		
		self._offset(offset).persist(length);
	}
	
	/// First 'half' of persistQuicklyAtCacheLineGranularity
	#[inline(always)]
	pub fn flush(&self, offset: usize, length: usize)
	{
		debug_assert!(offset + length <= self._mappedLength(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mappedLength());
		
		self._offset(offset).flush(length);
	}
	
	/// Second 'half' of persistQuicklyAtCacheLineGranularity
	#[inline(always)]
	pub fn drainAfterFlush()
	{
		unsafe { pmem_drain() }
	}
	
	#[inline(always)]
	pub fn persistOnDropFrom<'a>(&'a self, offset: usize) -> PersistOnDrop<'a>
	{
		PersistOnDrop(self._offset(offset), PhantomData)
	}
	
	#[inline(always)]
	pub fn persistOnDrop<'a>(&'a self) -> PersistOnDrop<'a>
	{
		PersistOnDrop(self.address, PhantomData)
	}
	
	// aka 'memmove' in C
	#[inline(always)]
	pub fn copy_then_persistQuicklyAtCacheLineGranularity(&self, offset: usize, length: usize, from: *const c_void)
	{
		debug_assert!(offset + length <= self._mappedLength(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mappedLength());
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memmove_persist(self._offset(offset), from, length) };
	}
	
	// aka 'memcpy' in C
	#[inline(always)]
	pub fn copy_nonoverlapping_then_persistQuicklyAtCacheLineGranularity(&self, offset: usize, length: usize, from: *const c_void)
	{
		debug_assert!(offset + length <= self._mappedLength(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mappedLength());
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memcpy_persist(self._offset(offset), from, length) };
	}
	
	// aka 'memset' in C
	#[inline(always)]
	pub fn write_bytes_then_persistQuicklyAtCacheLineGranularity(&self, offset: usize, count: usize, value: u8)
	{
		debug_assert!(offset + count <= self._mappedLength(), "offset '{}' + count '{}' is greater than mapped length '{}'", offset, count, self._mappedLength());
		
		unsafe { pmem_memset_persist(self._offset(offset), value as i32, count) };
	}
}
