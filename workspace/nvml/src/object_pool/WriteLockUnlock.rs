// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A structure that represents an owned write lock. When it goes out of scope (dropped) the write lock is released.
/// Use `deref()` (ie &x) to get to the object wrapped in the lock.
pub struct WriteLockUnlock<'a, T: Persistable + 'a>(ReadWriteLock<'a, T>);

impl<'a, T: Persistable + 'a> Drop for WriteLockUnlock<'a, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.unlock()
	}
}

impl<'a, T: Persistable + 'a> Deref for WriteLockUnlock<'a, T>
{
	type Target = T;
	
	fn deref(&self) -> &T
	{
		self.0.object
	}
}

impl<'a, T: Persistable + 'a> DerefMut for WriteLockUnlock<'a, T>
{
	fn deref_mut(&mut self) -> &mut T
	{
		self.0.object
	}
}
