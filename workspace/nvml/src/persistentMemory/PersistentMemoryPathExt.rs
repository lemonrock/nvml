// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PersistentMemoryPathExt
{
	#[inline(always)]
	fn mapMemoryFile(&self, length: usize, flags: PersistentMemoryFileFlags, mode: mode_t) -> Result<(*mut c_void, usize, bool), GenericError>;
}

impl PersistentMemoryPathExt for Path
{
	#[inline(always)]
	fn mapMemoryFile(&self, length: usize, flags: PersistentMemoryFileFlags, mode: mode_t) -> Result<(*mut c_void, usize, bool), GenericError>
	{
		let mut mappedLength: usize = unsafe { uninitialized() };
		let mappedLengthPointer = &mut mappedLength as *mut _;
		
		let mut isPersistentMemory: i32 = unsafe { uninitialized() };
		let isPersistentMemoryPointer = &mut isPersistentMemory as *mut _;
		
		let result = usePath!(self, pmem_map_file, length, flags.bits(), mode, mappedLengthPointer, isPersistentMemoryPointer);
		
		if unlikely(result.is_null())
		{
			let osErrorNumber = errno().0;
			Err(GenericError::new(osErrorNumber, pmem_errormsg, "pmem_map_file"))
		}
		else
		{
			let isPersistentMemory = if likely(isPersistentMemory == 1)
			{
				true
			}
			else if likely(isPersistentMemory == 0)
			{
				false
			}
			else
			{
				panic!("pmem_map_file() returned a value of is_pmemp which is not 1 or 0");
			};
			
			Ok((result, mappedLength, isPersistentMemory))
		}
	}
}
