// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Ensures the object pool is persisted when dropped.
#[derive(Debug)]
pub struct ObjectPoolPersistOnDrop<'a>(*mut PMEMobjpool, *mut c_void, PhantomData<&'a ObjectPool>);

impl<'a> Drop for ObjectPoolPersistOnDrop<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { pmemobj_drain(self.0) }
	}
}

impl<'a> ObjectPoolPersistOnDrop<'a>
{
	/// Creates a new instance of this struct.
	#[inline(always)]
	pub fn new(object_pool: *mut PMEMobjpool, address: *mut c_void) -> Self
	{
		ObjectPoolPersistOnDrop(object_pool, address, PhantomData)
	}
	
	/// Moves offset forward.
	#[inline(always)]
	pub fn offset(mut self, offset: usize) -> Self
	{
		self.1 = unsafe { self.1.offset(offset as isize) };
		self
	}
	
	/// Flushes data without draining.
	#[inline(always)]
	pub fn flush(&self, length: usize)
	{
		unsafe { pmemobj_flush(self.0, self.1, length); }
	}
}
