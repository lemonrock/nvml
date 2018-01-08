// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO mutex lock guard; the result of locking a CtoMutexLock.
/// When dropped (ie goes out of scope) the lock is released.
#[must_use]
pub struct CtoMutexLockGuard<'mutex_lock, T: 'mutex_lock + CtoSafe>(&'mutex_lock CtoMutexLockInner<T>);

impl<'mutex_lock, T: CtoSafe> !Send for CtoMutexLockGuard<'mutex_lock, T>
{
}

unsafe impl<'mutex_lock, T: CtoSafe + Sync> Sync for CtoMutexLockGuard<'mutex_lock, T>
{
}

impl<'mutex_lock, T: CtoSafe> Drop for CtoMutexLockGuard<'mutex_lock, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.0.unlock_mutex(); }
	}
}

impl<'mutex_lock, T: CtoSafe> Deref for CtoMutexLockGuard<'mutex_lock, T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'mutex_lock, T: CtoSafe> DerefMut for CtoMutexLockGuard<'mutex_lock, T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut * self.0.value.get() }
	}
}

impl<'mutex_lock, T: CtoSafe> Borrow<T> for CtoMutexLockGuard<'mutex_lock, T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<'mutex_lock, T: CtoSafe> BorrowMut<T> for CtoMutexLockGuard<'mutex_lock, T>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<'mutex_lock, T: CtoSafe> AsRef<T> for CtoMutexLockGuard<'mutex_lock, T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<'mutex_lock, T: CtoSafe> AsMut<T> for CtoMutexLockGuard<'mutex_lock, T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}
