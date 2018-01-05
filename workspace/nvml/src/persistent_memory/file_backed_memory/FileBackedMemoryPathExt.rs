// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait to make it easier to use Path to map in file-backed memory, irrespective of whether it is directly accessible or mmap'd.
pub trait FileBackedMemoryPathExt
{
	/// Maps a memory file.
	#[inline(always)]
	fn map_memory_file(&self, length: usize, open_flags: FileBackedMemoryOpenFlags, mode: mode_t) -> Result<(*mut c_void, usize, bool), PmdkError>;
}

impl FileBackedMemoryPathExt for Path
{
	#[inline(always)]
	fn map_memory_file(&self, length: usize, open_flags: FileBackedMemoryOpenFlags, mode: mode_t) -> Result<(*mut c_void, usize, bool), PmdkError>
	{
		let mut mapped_length: usize = unsafe { uninitialized() };
		let mapped_length_pointer = &mut mapped_length as *mut _;
		
		let mut is_persistent_memory: i32 = unsafe { uninitialized() };
		let is_persistent_memory_pointer = &mut is_persistent_memory as *mut _;
		
		let result = use_path!(self, pmem_map_file, length, open_flags.bits(), mode, mapped_length_pointer, is_persistent_memory_pointer);
		
		if unlikely(result.is_null())
		{
			PmdkError::pmem("pmem_map_file")
		}
		else
		{
			let is_persistent_memory = if likely(is_persistent_memory == 1)
			{
				true
			}
			else if likely(is_persistent_memory == 0)
			{
				false
			}
			else
			{
				panic!("pmem_map_file() returned a value of is_pmemp which is not 1 or 0");
			};
			
			Ok((result, mapped_length, is_persistent_memory))
		}
	}
}
