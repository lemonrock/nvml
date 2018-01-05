// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait FileBackedMemory : Sized + Send + Sync
{
	// Look in bits/limits.h in musl for #define PAGE_SIZE
	// Is 4096 for all musl architectures, except or1k, where it is 8192 (statement true as of April 25th 2017)
	const Alignment: usize;
	
	const IsPersistent: bool;
	
	#[inline(always)]
	fn open(persistentMemoryFilePath: &Path, exclusive: bool) -> Result<Option<Self>, PmdkError>
	{
		const length: usize = 0;
		
		let flags = Self::_openFlags(exclusive);
		
		const IrrelevantMode: mode_t = 0;
		
		Self::_map(persistentMemoryFilePath, length, flags, IrrelevantMode)
	}
	
	/// offset and length will be adjusted to page size granularity
	#[allow(deprecated)]
	#[inline(always)]
	fn persistSlowlyAtPageSizeGranularity(&self, offset: usize, length: usize)
	{
		debug_assert!(offset + length <= self._mappedLength(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mappedLength());
		
		self._offset(offset).persistOrMsyncRegardlessOfWhetherSelfIsPersistentOrNonPersistentMemory(length)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _map(persistentMemoryFilePath: &Path, length: usize, flags: PersistentMemoryFileFlags, mode: mode_t) -> Result<Option<Self>, PmdkError>
	{
		let result = persistentMemoryFilePath.mapMemoryFile(length, flags, mode);
		
		if unlikely(result.is_err())
		{
			return Err(result.unwrap_err());
		}
		
		let (address, mappedLength, isPersistentMemory) = result.unwrap();
		
		if unlikely(address.is_null())
		{
			panic!("Mapping returned a null address");
		}
		
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
	
	#[doc(hidden)]
	#[inline(always)]
	fn _address(&self) -> *mut c_void;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _mappedLength(&self) -> usize;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _offset(&self, offset: usize) -> *mut c_void
	{
		debug_assert!(offset <= self._mappedLength(), "offset '{}' is greater than mapped length '{}'", offset, self._mappedLength());
		
		unsafe { self._address().offset(offset as isize) }
	}
}
