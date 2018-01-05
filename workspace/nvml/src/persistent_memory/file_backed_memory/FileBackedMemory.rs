// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents the operations commonly possible across different kinds of file-backed memory.
pub trait FileBackedMemory : Sized + Send + Sync
{
	/// The (power-of-two, non-zero) alignment of this memory.
	/// Look in bits/limits.h in musl for #define PAGE_SIZE
	/// Is 4096 for all musl architectures, except or1k, where it is 8192 (statement true as of April 25th 2017)
	const Alignment: usize;
	
	/// Is this persistent memory?
	const IsPersistent: bool;
	
	/// Does this memory support exclusive open?
	const SupportsExclusiveOpen: bool;
	
	/// Open file-backed memory from a file.
	#[inline(always)]
	fn open(persistent_memory_file_path: &Path, exclusive: bool) -> Result<Option<Self>, PmdkError>
	{
		if exclusive && !Self::SupportsExclusiveOpen
		{
			panic!("This memory kind does not support exclusive open");
		}
		
		const length: usize = 0;
		
		let flags = Self::_open_flags(exclusive);
		
		const IrrelevantMode: mode_t = 0;
		
		Self::_map(persistent_memory_file_path, length, flags, IrrelevantMode)
	}
	
	/// offset and length will be adjusted to page size granularity
	#[allow(deprecated)]
	#[inline(always)]
	fn persist_slowly_at_page_size_granularity(&self, offset: usize, length: usize)
	{
		debug_assert!(offset + length <= self._mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self._mapped_length());
		
		self._offset(offset).persist_or_msync_regardless_of_whether_self_is_persistent_or_regular_memory(length)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _map(persistent_memory_file_path: &Path, length: usize, open_flags: FileBackedMemoryOpenFlags, mode: mode_t) -> Result<Option<Self>, PmdkError>
	{
		let result = persistent_memory_file_path.map_memory_file(length, open_flags, mode);
		
		if unlikely(result.is_err())
		{
			return Err(result.unwrap_err());
		}
		
		let (address, mapped_length, is_persistent_memory) = result.unwrap();
		
		if unlikely(address.is_null())
		{
			panic!("Mapping returned a null address");
		}
		
		if likely(Self::_finish_mapping_if_memory_is_of_correct_type(is_persistent_memory, address.is_persistent_memory_that_supports_flushing_with_persist(mapped_length)))
		{
			Ok(Some(Self::_new(address, mapped_length)))
		}
		else
		{
			address.unmap(mapped_length);
			Ok(None)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _open_flags(exclusive: bool) -> FileBackedMemoryOpenFlags;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _finish_mapping_if_memory_is_of_correct_type(is_persistent_memory: bool, is_persistent_memory_that_supports_flushing_with_persist: bool) -> bool;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _new(address: *mut c_void, mapped_length: usize) -> Self;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _address(&self) -> *mut c_void;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _mapped_length(&self) -> usize;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _offset(&self, offset: usize) -> *mut c_void
	{
		debug_assert!(offset <= self._mapped_length(), "offset '{}' is greater than mapped length '{}'", offset, self._mapped_length());
		
		unsafe { self._address().offset(offset as isize) }
	}
}
