// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	/// Various flags for opening persistent memory file mappings.
	#[derive(Default)]
	pub struct FileBackedMemoryOpenFlags: i32
	{
		/// No flags
		const None = 0;
		
		/// Create.
		const Create = PMEM_FILE_CREATE as i32;
		
		/// Only allow this process access to the memory.
		const Exclusive = PMEM_FILE_EXCL as i32;
		
		/// Allow sparse mappings (ie large area of zero bytes are not actually mapped).
		const Sparse = PMEM_FILE_SPARSE as i32;
		
		/// Use a temporary file mapping.
		const TmpFile = PMEM_FILE_TMPFILE as i32;
	}
}
