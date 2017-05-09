// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub mod PersistentMemoryFileFlags
{
	use ::nvml_sys::*;
	
	bitflags!
	{
		#[derive(Default)]
		pub flags Flags: i32
		{
			const None = 0,
			
			const Create = PMEM_FILE_CREATE as i32,
			
			const Exclusive = PMEM_FILE_EXCL as i32,
			
			const Sparse = PMEM_FILE_SPARSE as i32,
			
			const TmpFile = PMEM_FILE_TMPFILE as i32,
		}
	}
}
