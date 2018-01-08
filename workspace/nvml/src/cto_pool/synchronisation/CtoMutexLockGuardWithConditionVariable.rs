// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A variant of a mutex lock guard that also encapsulates use of a condition variable.
pub struct CtoMutexLockGuardWithConditionVariable<'mutex_lock, T: 'mutex_lock + CtoSafe>
{
	cto_mutex_lock_guard: CtoMutexLockGuard<'mutex_lock, T>,
	cto_condition_variable: &'mutex_lock CtoConditionVariable,
}

impl<'mutex_lock, T: CtoSafe> Deref for CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.cto_mutex_lock_guard.deref()
	}
}

impl<'mutex_lock, T: CtoSafe> DerefMut for CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.cto_mutex_lock_guard.deref_mut()
	}
}

impl<'mutex_lock, T: CtoSafe> Borrow<T> for CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<'mutex_lock, T: CtoSafe> BorrowMut<T> for CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<'mutex_lock, T: CtoSafe> AsRef<T> for CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<'mutex_lock, T: CtoSafe> AsMut<T> for CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<'mutex_lock, T: CtoSafe> CtoMutexLockGuardWithConditionVariable<'mutex_lock, T>
{
	/// Blocks the current thread until this condition variable receives a notification.
	#[inline(always)]
	pub fn wait(self) -> Self
	{
		Self
		{
			cto_mutex_lock_guard: self.cto_condition_variable.wait(self.cto_mutex_lock_guard),
			cto_condition_variable: self.cto_condition_variable,
		}
	}
	
	/// Waits on this condition variable for a notification, timing out after a specified duration.
	#[inline(always)]
	pub fn wait_timeout(self, duration: Duration) -> (Self, TimedOut)
	{
		let (cto_mutex_lock_guard, timed_out) = self.cto_condition_variable.wait_timeout(self.cto_mutex_lock_guard, duration);
		
		(
			Self
			{
				cto_mutex_lock_guard,
				cto_condition_variable: self.cto_condition_variable,
			},
			timed_out
		)
	}
	
	/// Wakes up one blocked thread on this condition variable.
	/// Unlocks the mutex immediately AFTER signalling, as recommended in <http://www.domaigne.com/blog/computing/condvars-signal-with-mutex-locked-or-not/>
	#[inline(always)]
	pub fn unlock_then_notify_one(self)
	{
		drop(self.cto_mutex_lock_guard);
		self.cto_condition_variable.notify_one()
	}
	
	/// Wakes up all blocked threads on this condition variable.
	/// Unlocks the mutex immediately AFTER signalling, as recommended in <http://www.domaigne.com/blog/computing/condvars-signal-with-mutex-locked-or-not/>
	#[inline(always)]
	pub fn unlock_then_notify_all(self)
	{
		drop(self.cto_mutex_lock_guard);
		self.cto_condition_variable.notify_all()
	}
	
	/// Wakes up one blocked thread on this condition variable.
	/// Unlocks the mutex immediately BEFORE signalling
	#[inline(always)]
	pub fn notify_one_then_unlock(self)
	{
		self.cto_condition_variable.notify_one();
		drop(self.cto_mutex_lock_guard)
	}
	
	/// Wakes up all blocked threads on this condition variable.
	/// Unlocks the mutex immediately BEFORE signalling
	#[inline(always)]
	pub fn notify_all_then_unlock(self)
	{
		self.cto_condition_variable.notify_all();
		drop(self.cto_mutex_lock_guard)
	}
}
