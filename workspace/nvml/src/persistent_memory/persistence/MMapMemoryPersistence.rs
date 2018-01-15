// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// mmap'd memory.
#[derive(Debug)]
pub struct MMapMemoryPersistence;

impl Persistence for MMapMemoryPersistence
{
	#[inline(always)]
	fn flush_memory(address: *mut c_void, length: usize)
	{
		unsafe { pmem_msync(address, length) };
	}
	
	#[inline(always)]
	fn drain_memory()
	{
		unsafe { pmem_drain() };
	}
}
