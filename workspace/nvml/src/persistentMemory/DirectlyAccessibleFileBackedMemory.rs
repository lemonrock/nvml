// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct DirectlyAccessibleFileBackedMemory
{
	address: *mut c_void,
	mappedLength: usize,
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
			mappedLength: mappedLength,
		}
	}
}
