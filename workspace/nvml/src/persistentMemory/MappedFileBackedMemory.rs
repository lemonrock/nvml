// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct MappedFileBackedMemory
{
	address: *mut c_void,
	fileBackedMemoryDropWrapper: Arc<FileBackedMemoryDropWrapper>,
}

unsafe impl Send for MappedFileBackedMemory
{
}

unsafe impl Sync for MappedFileBackedMemory
{
}

impl FileBackedMemory for MappedFileBackedMemory
{
	const Alignment: usize = 4096;
	
	const IsPersistent: bool = true;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _openFlags(exclusive: bool) -> PersistentMemoryFileFlags
	{
		if exclusive
		{
			PersistentMemoryFileFlags::Exclusive
		}
		else
		{
			PersistentMemoryFileFlags::None
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _finishMappingIfMemoryIsOfCorrectType(isPersistentMemory: bool, isPersistentMemoryThatSupportsFlushingWithPersist: bool) -> bool
	{
		!isPersistentMemory || !isPersistentMemoryThatSupportsFlushingWithPersist
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

impl MappedFileBackedMemory
{
	#[inline(always)]
	pub fn createAndOpen(persistentMemoryFilePath: &Path, length: usize, mode: mode_t, sparse: bool, temporaryFile: bool, exclusive: bool) -> Result<Option<Self>, PmdkError>
	{
		assert!(length != 0, "length can not be zero when creating");
		
		let mut flags = PersistentMemoryFileFlags::Create;
		
		if sparse
		{
			flags |= PersistentMemoryFileFlags::Sparse;
		}
		
		if temporaryFile
		{
			flags |= PersistentMemoryFileFlags::TmpFile;
		}
		
		if exclusive
		{
			flags |= PersistentMemoryFileFlags::Exclusive;
		}
		
		Self::_map(persistentMemoryFilePath, length, flags, mode)
	}
}
