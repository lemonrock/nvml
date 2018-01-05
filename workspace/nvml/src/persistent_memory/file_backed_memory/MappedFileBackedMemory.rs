// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// MMap'd persistent memory.
#[derive(Debug, Clone)]
pub struct MappedFileBackedMemory
{
	address: *mut c_void,
	file_backed_memory_drop_wrapper: Arc<FileBackedMemoryDropWrapper>,
}

unsafe impl Send for MappedFileBackedMemory
{
}

unsafe impl Sync for MappedFileBackedMemory
{
}

impl<'memory> FileBackedMemory<'memory> for MappedFileBackedMemory
{
	type PersistOnDropT = MappedPersistOnDrop<'memory>;
	
	const Alignment: usize = 4096;
	
	const IsPersistent: bool = true;
	
	const SupportsExclusiveOpen: bool = true;
	
	#[inline(always)]
	fn address(&self) -> *mut c_void
	{
		self.address
	}
	
	#[inline(always)]
	fn mapped_length(&self) -> usize
	{
		self.file_backed_memory_drop_wrapper.mapped_length
	}
	
	#[inline(always)]
	fn persist_on_drop_from(&'memory self, offset: usize) -> Self::PersistOnDropT
	{
		MappedPersistOnDrop(self.offset(offset), PhantomData, Cell::new(0))
	}
	
	#[inline(always)]
	fn persist_on_drop(&'memory self) -> Self::PersistOnDropT
	{
		MappedPersistOnDrop(self.address(), PhantomData, Cell::new(0))
	}
	
	#[inline(always)]
	fn copy_then_persist_at_alignment_granularity(&self, offset: usize, length: usize, from: *const c_void)
	{
		debug_assert!(offset + length <= self.mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self.mapped_length());
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { pmem_memmove_persist(self.offset(offset), from, length) };
		self.persist_slowly_at_page_size_granularity(offset, length)
	}
	
	#[inline(always)]
	fn copy_nonoverlapping_then_persist_at_alignment_granularity(&self, offset: usize, length: usize, from: *const c_void)
	{
		debug_assert!(offset + length <= self.mapped_length(), "offset '{}' + length '{}' is greater than mapped length '{}'", offset, length, self.mapped_length());
		debug_assert!(!from.is_null(), "from must not be null");
		
		unsafe { copy_nonoverlapping(from, self.offset(offset), length) }
		self.persist_slowly_at_page_size_granularity(offset, length)
	}
	
	#[inline(always)]
	fn write_bytes_then_persist_at_alignment_granularity(&self, offset: usize, count: usize, value: u8)
	{
		debug_assert!(offset + count <= self.mapped_length(), "offset '{}' + count '{}' is greater than mapped length '{}'", offset, count, self.mapped_length());
		
		unsafe { write_bytes(self.offset(offset), value, count) }
		self.persist_slowly_at_page_size_granularity(offset, count)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _open_flags(exclusive: bool) -> FileBackedMemoryOpenFlags
	{
		if exclusive
		{
			FileBackedMemoryOpenFlags::Exclusive
		}
		else
		{
			FileBackedMemoryOpenFlags::None
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _finish_mapping_if_memory_is_of_correct_type(is_persistent_memory: bool, is_persistent_memory_that_supports_flushing_with_persist: bool) -> bool
	{
		!is_persistent_memory || !is_persistent_memory_that_supports_flushing_with_persist
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
}

impl MappedFileBackedMemory
{
	/// Create (if required) and then open a mapped-file file-backed persistent memory.
	#[inline(always)]
	pub fn create_and_open(persistent_memory_file_path: &Path, length: usize, mode: mode_t, sparse: bool, temporary_file: bool, exclusive: bool) -> Result<Option<Self>, PmdkError>
	{
		assert_ne!(length, 0, "length can not be zero when creating");
		
		let mut flags = FileBackedMemoryOpenFlags::Create;
		
		if sparse
		{
			flags |= FileBackedMemoryOpenFlags::Sparse;
		}
		
		if temporary_file
		{
			flags |= FileBackedMemoryOpenFlags::TmpFile;
		}
		
		if exclusive
		{
			flags |= FileBackedMemoryOpenFlags::Exclusive;
		}
		
		Self::_map(persistent_memory_file_path, length, flags, mode)
	}
}
