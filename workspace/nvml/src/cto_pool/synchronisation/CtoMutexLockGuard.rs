// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO mutex lock guard; the result of locking a CtoMutex.
/// When dropped (ie goes out of scope) the lock is released.
#[must_use]
pub struct CtoMutexLockGuard<'mutex, T: 'mutex + CtoSafe>
{
	cto_mutex_lock: &'mutex CtoMutexLock<T>,
}

impl<'mutex, T: CtoSafe> !Send for CtoMutexLockGuard<'mutex, T>
{
}

unsafe impl<'mutex, T: CtoSafe + Sync> Sync for CtoMutexLockGuard<'mutex, T>
{
}

impl<'mutex, T: CtoSafe + Debug> Debug for CtoMutexLockGuard<'mutex, T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.debug_struct("CtoMutexGuard").field("cto_mutex", &self.cto_mutex_lock).finish()
	}
}

impl<'mutex, T: CtoSafe + Display> Display for CtoMutexLockGuard<'mutex, T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		(**self).fmt(f)
	}
}

impl<'mutex, T: CtoSafe> Drop for CtoMutexLockGuard<'mutex, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.cto_mutex_lock.unlock_mutex(); }
	}
}

impl<'mutex, T: CtoSafe> Deref for CtoMutexLockGuard<'mutex, T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { &*self.cto_mutex_lock.value.get() }
	}
}

impl<'mutex, T: CtoSafe> DerefMut for CtoMutexLockGuard<'mutex, T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut *self.cto_mutex_lock.value.get() }
	}
}

impl<'mutex, T: CtoSafe> Borrow<T> for CtoMutexLockGuard<'mutex, T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<'mutex, T: CtoSafe> BorrowMut<T> for CtoMutexLockGuard<'mutex, T>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<'mutex, T: CtoSafe> AsRef<T> for CtoMutexLockGuard<'mutex, T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<'mutex, T: CtoSafe> AsMut<T> for CtoMutexLockGuard<'mutex, T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}
