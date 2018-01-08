// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO Read-Write lock read guard; the result of read-locking a CtoReadWriteLock.
/// When dropped (ie goes out of scope) the read lock is released.
#[must_use]
pub struct CtoReadWriteLockReadGuard<'read_write_lock, T: 'read_write_lock + CtoSafe>(&'read_write_lock CtoReadWriteLockInner<T>);

impl<'read_write_lock, T: CtoSafe> !Send for CtoReadWriteLockReadGuard<'read_write_lock, T>
{
}

unsafe impl<'read_write_lock, T: CtoSafe + Sync> Sync for CtoReadWriteLockReadGuard<'read_write_lock, T>
{
}

impl<'read_write_lock, T: CtoSafe> Drop for CtoReadWriteLockReadGuard<'read_write_lock, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.0.read_unlock(); }
	}
}

impl<'read_write_lock, T: CtoSafe> Deref for CtoReadWriteLockReadGuard<'read_write_lock, T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'read_write_lock, T: CtoSafe> Borrow<T> for CtoReadWriteLockReadGuard<'read_write_lock, T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<'read_write_lock, T: CtoSafe> AsRef<T> for CtoReadWriteLockReadGuard<'read_write_lock, T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}
