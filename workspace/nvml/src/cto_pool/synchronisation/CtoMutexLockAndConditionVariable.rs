// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Simple wrapper type to make it easier to work correctly with condition variables and mutexes in CtoSafe structures.
#[derive(Debug)]
pub struct CtoMutexLockAndConditionVariable<Value: CtoSafe>
{
	cto_mutex_lock: CtoMutexLock<Value>,
	cto_condition_variable: CtoConditionVariable,
}

unsafe impl<Value: CtoSafe> Send for CtoMutexLockAndConditionVariable<Value>
{
}

unsafe impl<Value: CtoSafe> Sync for CtoMutexLockAndConditionVariable<Value>
{
}

impl<Value: CtoSafe> UnwindSafe for CtoMutexLockAndConditionVariable<Value>
{
}

impl<Value: CtoSafe> RefUnwindSafe for CtoMutexLockAndConditionVariable<Value>
{
}

impl<Value: CtoSafe> CtoSafe for CtoMutexLockAndConditionVariable<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.cto_mutex_lock.cto_pool_opened(cto_pool_arc);
		self.cto_condition_variable.cto_pool_opened(cto_pool_arc)
	}
}

impl<Value: CtoSafe> CtoMutexLockAndConditionVariable<Value>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(cto_mutex_lock: CtoMutexLock<Value>) -> Self
	{
		Self
		{
			cto_mutex_lock,
			cto_condition_variable: CtoConditionVariable::new(),
		}
	}
	
	/// Locks the mutex.
	/// Use the resultant object to access wait / notify behaviour of the condition variable.
	#[inline(always)]
	pub fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuardWithConditionVariable<'mutex, Value>
	{
		CtoMutexLockGuardWithConditionVariable
		{
			cto_mutex_lock_guard: self.cto_mutex_lock.lock(),
			cto_condition_variable: &self.cto_condition_variable,
		}
	}
	
	/// Returns Some(lock_guard) if could be locked.
	/// Returns None if the lock is held by another.
	/// Use the resultant object to access wait / notify behaviour of the condition variable.
	#[inline(always)]
	pub fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuardWithConditionVariable<'mutex, Value>>
	{
		self.cto_mutex_lock.try_lock().map(|cto_mutex_lock_guard| CtoMutexLockGuardWithConditionVariable
		{
			cto_mutex_lock_guard,
			cto_condition_variable: &self.cto_condition_variable,
		})
	}
}
