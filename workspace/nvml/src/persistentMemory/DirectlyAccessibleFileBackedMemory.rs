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
}
