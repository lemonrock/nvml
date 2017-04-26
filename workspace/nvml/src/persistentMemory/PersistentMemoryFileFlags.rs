// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


bitflags!
{
	pub flags PersistentMemoryFileFlags: i32
	{
		const Create = PMEM_FILE_CREATE,
		const Exclusive = PMEM_FILE_EXCL,
		const Sparse = PMEM_FILE_SPARSE,
		const TmpFile = PMEM_FILE_TMPFILE,
	}
}
