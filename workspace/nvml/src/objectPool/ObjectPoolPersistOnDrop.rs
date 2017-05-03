// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct ObjectPoolPersistOnDrop(*mut PMEMobjpool, *mut c_void);

impl Drop for ObjectPoolPersistOnDrop
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { pmemobj_drain(self.0) }
	}
}

impl ObjectPoolPersistOnDrop
{
	#[inline(always)]
	pub fn new(objectPool: *mut PMEMobjpool, address: *mut c_void) -> Self
	{
		ObjectPoolPersistOnDrop(objectPool, address)
	}
	
	#[inline(always)]
	pub fn offset(mut self, offset: usize) -> Self
	{
		self.1 = unsafe { self.1.offset(offset as isize) };
		self
	}
	
	#[inline(always)]
	pub fn flush(&self, length: usize)
	{
		unsafe { pmemobj_flush(self.0, self.1, length); }
	}
}
