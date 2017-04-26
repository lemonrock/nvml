// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait FileBackedMemory : Sized + Send + Sync
{
	// Look in bits/limits.h in musl for #define PAGE_SIZE
	// Is 4096 for all musl architectures, except or1k, where it is 8192 (statement true as of April 25th 2017)
	const Alignment: usize;
	
	const IsPersistent: bool;
	
	#[inline(always)]
	fn open(persistentMemoryFilePath: &Path, exclusive: bool) -> Result<Option<Self>, GenericError>
	{
		const length: usize = 0;
		
		let flags = Self::_openFlags(exclusive);
		
		const IrrelevantMode: mode_t = 0;
		
		Self::_map(persistentMemoryFilePath, length, flags, IrrelevantMode)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _map(persistentMemoryFilePath: &Path, length: usize, flags: PersistentMemoryFileFlags, mode: mode_t) -> Result<Option<Self>, GenericError>
	{
		let result = persistentMemoryFilePath.mapMemoryFile(length, flags, mode);
		
		if unlikely(result.is_err())
		{
			return Err(result.unwrap_err());
		}
		
		let (address, mappedLength, isPersistentMemory) = result.unwrap();
		
		if likely(Self::_finishMappingIfMemoryIsOfCorrectType(isPersistentMemory, address.isPersistentMemoryThatSupportsFlushingWithPersist(mappedLength)))
		{
			Ok(Some(Self::_new(address, mappedLength)))
		}
		else
		{
			address.unmap(mappedLength);
			Ok(None)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _openFlags(exclusive: bool) -> PersistentMemoryFileFlags;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _finishMappingIfMemoryIsOfCorrectType(isPersistentMemory: bool, isPersistentMemoryThatSupportsFlushingWithPersist: bool) -> bool;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(address: *mut c_void, mappedLength: usize) -> Self;
}
