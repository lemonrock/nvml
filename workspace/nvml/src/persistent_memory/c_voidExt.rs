// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension trait to *const c_void (and *mut c_void) pointers to make it easier to work with persistent memory.
#[allow(non_camel_case_types)]
pub trait c_voidExt
{
	/// On Linux, only returns true if memory is mapped directly from a device file of the kind `/dev/daxX.Y` (aka a Device DAX) without an intervening filesystem
	/// Only returns true if ALL memory in the range is persistent.
	/// If false, use msync() instead on this memory.
	/// This is non-trivial function; do not call it repeatedly.
	/// self and length do not need to be aligned.
	/// self can not be null.
	/// It is not known if length can be zero.
	#[inline(always)]
	fn is_persistent_memory_that_supports_flushing_with_persist(self, length: usize) -> bool;
	
	/// Only use this on memory for which `is_persistent_memory_that_supports_flushing_with_persist()` is true.
	/// Similar to traditional `msync()` of a memory-mapped file but without kernel overhead.
	/// Stores may already have been persisted before this call (eg due to cache eviction).
	/// This method is neither transactional nor atomic.
	/// self and length do not need to be aligned, however, they will be adjusted to Cache Line Size Alignment (64 bytes on x86-64).
	/// self can not be null.
	/// It is not known if length can be zero.
	#[inline(always)]
	fn persist(self, length: usize);
	
	/// Use this on any kind of file-backed memory, either persistent-memory or memory-mapped-file.
	/// It is quite slow for persistent memory as it calls into the kernel using `msync(MS_SYNC)`.
	/// Stores may already have been persisted before this call (eg due to cache eviction).
	/// This method is neither transactional nor atomic.
	/// self and length do not need to be aligned, however, they will be adjusted to Page Size (4Kb on x86-64).
	/// self can not be null.
	/// length can be zero.
	#[deprecated(note = "Avoid this as it adds substantial overhead for persistent memory")]
	#[inline(always)]
	fn persist_or_msync_regardless_of_whether_self_is_persistent_or_regular_memory(self, length: usize);
	
	/// Only use this on memory for which `is_persistent_memory_that_supports_flushing_with_persist()` is true.
	/// Similar to traditional `msync()` of a memory-mapped file but without kernel overhead.
	/// Stores may already have been persisted before this call (eg due to cache eviction).
	/// This method is neither transactional nor atomic.
	/// self and length do not need to be aligned, however, they will be adjusted to Cache Line Size Alignment (64 bytes on x86-64).
	/// First 'half' of pmem_persist.
	/// Call pmem_drain() after this function.
	/// Use this for discontinuous ranges, eg those in `iovec` buffers.
	/// self can not be null.
	/// It is not known if length can be zero.
	#[inline(always)]
	fn flush(self, length: usize);
}

macro_rules! debug_assert_self_is_not_null
{
	($self: ident) =>
	{
		debug_assert!(!$self.is_null(), "self (address) can not be null");
	}
}

impl c_voidExt for *const c_void
{
	#[inline(always)]
	fn is_persistent_memory_that_supports_flushing_with_persist(self, length: usize) -> bool
	{
		debug_assert_self_is_not_null!(self);
		
		let result = unsafe { pmem_is_pmem(self, length) };
		if likely(result == 1)
		{
			true
		}
		else if likely(result == 0)
		{
			false
		}
		else
		{
			panic!("pmem_is_pmem() returned value '{}', not 1 or 0", result);
		}
	}
	
	#[inline(always)]
	fn persist(self, length: usize)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmem_persist(self, length) }
	}
	
	#[inline(always)]
	fn persist_or_msync_regardless_of_whether_self_is_persistent_or_regular_memory(self, length: usize)
	{
		debug_assert_self_is_not_null!(self);
		
		if length == 0
		{
			return;
		}
		
		let result = unsafe { pmem_msync(self, length) };
		if likely(result == 0)
		{
			return;
		}
		else if likely(result == -1)
		{
			match errno().0
			{
				ENOMEM => panic!("Address range and length is not fully-backed by either persistent memory or a memory-mapped file"),
				
				EBUSY => panic!("EBUSY should be impossible for pmem_sync()"),
				EINVAL => panic!("EINVAL should be impossible for pmem_sync()"),
				
				illegal @ _ => panic!("Error number '{}' should not occur for pmem_sync()", illegal),
			}
		}
		else
		{
			panic!("pmem_msync() returned value '{}', not 0 or -1", result);
		}
	}
	
	#[inline(always)]
	fn flush(self, length: usize)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmem_flush(self, length) }
	}
}

impl c_voidExt for *mut c_void
{
	#[inline(always)]
	fn is_persistent_memory_that_supports_flushing_with_persist(self, length: usize) -> bool
	{
		(self as *const _).is_persistent_memory_that_supports_flushing_with_persist(length)
	}
	
	#[inline(always)]
	fn persist(self, length: usize)
	{
		(self as *const _).persist(length)
	}
	
	#[allow(deprecated)]
	#[inline(always)]
	fn persist_or_msync_regardless_of_whether_self_is_persistent_or_regular_memory(self, length: usize)
	{
		(self as *const _).persist_or_msync_regardless_of_whether_self_is_persistent_or_regular_memory(length)
	}
	
	#[inline(always)]
	fn flush(self, length: usize)
	{
		(self as *const _).flush(length)
	}
}
