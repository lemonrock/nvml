// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO mutex lock guard; the result of locking a CtoMutexLock.
/// When dropped (ie goes out of scope) the lock is released.
#[must_use]
pub struct CtoMutexLockGuard<'mutex_lock, Value: 'mutex_lock + CtoSafe>(&'mutex_lock CtoMutexLockInner<Value>);

impl<'mutex_lock, Value: CtoSafe> !Send for CtoMutexLockGuard<'mutex_lock, Value>
{
}

unsafe impl<'mutex_lock, Value: CtoSafe + Sync> Sync for CtoMutexLockGuard<'mutex_lock, Value>
{
}

impl<'mutex_lock, Value: CtoSafe> Drop for CtoMutexLockGuard<'mutex_lock, Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.0.unlock_mutex(); }
	}
}

impl<'mutex_lock, Value: CtoSafe> Deref for CtoMutexLockGuard<'mutex_lock, Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'mutex_lock, Value: CtoSafe> DerefMut for CtoMutexLockGuard<'mutex_lock, Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut * self.0.value.get() }
	}
}

impl<'mutex_lock, Value: CtoSafe> Borrow<Value> for CtoMutexLockGuard<'mutex_lock, Value>
{
	#[inline(always)]
	fn borrow(&self) -> &Value
	{
		self.deref()
	}
}

impl<'mutex_lock, Value: CtoSafe> BorrowMut<Value> for CtoMutexLockGuard<'mutex_lock, Value>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut Value
	{
		self.deref_mut()
	}
}

impl<'mutex_lock, Value: CtoSafe> AsRef<Value> for CtoMutexLockGuard<'mutex_lock, Value>
{
	#[inline(always)]
	fn as_ref(&self) -> &Value
	{
		self.deref()
	}
}

impl<'mutex_lock, Value: CtoSafe> AsMut<Value> for CtoMutexLockGuard<'mutex_lock, Value>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Value
	{
		self.deref_mut()
	}
}
