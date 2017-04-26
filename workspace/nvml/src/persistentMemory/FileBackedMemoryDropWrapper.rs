// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
struct FileBackedMemoryDropWrapper
{
	address: *mut c_void,
	mappedLength: usize,
}

impl Drop for FileBackedMemoryDropWrapper
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.address.unmap(self.mappedLength)
	}
}

impl FileBackedMemoryDropWrapper
{
	#[inline(always)]
	fn new(address: *mut c_void, mappedLength: usize) -> Arc<Self>
	{
		Arc::new
		(
			Self
			{
				address: address,
				mappedLength: mappedLength,
			}
		)
	}
}
