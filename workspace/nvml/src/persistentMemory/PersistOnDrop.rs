// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct PersistOnDrop<'a>(*mut c_void, PhantomData<&'a DirectlyAccessibleFileBackedMemory>);

impl<'a> Drop for PersistOnDrop<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		DirectlyAccessibleFileBackedMemory::drainAfterFlush()
	}
}

impl<'a> PersistOnDrop<'a>
{
	#[inline(always)]
	pub fn offset(&mut self, offset: usize)
	{
		self.0 = unsafe { self.0.offset(offset as isize) };
	}
	
	#[inline(always)]
	pub fn flush(&self, length: usize)
	{
		self.0.flush(length);
	}
}
