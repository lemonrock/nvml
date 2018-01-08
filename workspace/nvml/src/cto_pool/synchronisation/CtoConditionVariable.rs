// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Condition Variabel, similar to Condvar Rust, but lacking the concepts of Poison and verification.
pub struct CtoConditionVariable
{
	persistent_memory_pointer: *mut CtoConditionVariableInner,
}

impl CtoSafe for CtoConditionVariable
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.persistent_memory_mut().reinitialize(cto_pool_inner)
	}
}

impl PersistentMemoryWrapper for CtoConditionVariable
{
	type PersistentMemory = CtoConditionVariableInner;
	
	type Value = CtoConditionVariableInner;
	
	#[inline(always)]
	fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(&mut Self::Value) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_inner: &Arc<CtoPoolInner>, _initializer: Initializer) -> Result<Self, InitializationError>
	{
		let inner = unsafe { &mut * persistent_memory_pointer };
		inner.reinitialize(cto_pool_inner);
		Ok
		(
			Self
			{
				persistent_memory_pointer,
			}
		)
	}
}

impl Drop for CtoConditionVariable
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let cto_pool_inner = self.persistent_memory().cto_pool_inner.clone();
		CtoPoolInner::free_persistent_memory(&cto_pool_inner, self.persistent_memory_pointer)
	}
}

impl Debug for CtoConditionVariable
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.pad("CtoConditionVariable { .. }")
	}
}

impl CtoConditionVariable
{
	/// Blocks the current thread until this condition variable receives a notification.
	///
	/// This function will atomically unlock the mutex specified (represented by `guard`) and block the current thread.
	/// This means that any calls to [`notify_one`] or [`notify_all`] which happen logically after the mutex is unlocked are candidates to wake this thread up.
	/// When this function call returns, the lock specified will have been re-acquired.
	///
	/// Note that this function is susceptible to spurious wakeups.
	/// Condition variables normally have a boolean predicate associated with them, and the predicate must always be checked each time this function returns to protect against spurious wakeups.
    ///
    /// [`notify_one`]: #method.notify_one
    /// [`notify_all`]: #method.notify_all
	///
	///
	/// # Warning
	///
	/// This implementation, unlike that supplied by Rust, ***does not*** check that one, and only one, mutex `guard` is being used with this condition variable.
    ///
	#[inline(always)]
	pub fn wait<'mutex, T: CtoSafe>(&'mutex self, guard: CtoMutexLockGuard<'mutex, T>) -> CtoMutexLockGuard<'mutex, T>
	{
		self.persistent_memory().wait(guard.0.mutex.get());
		guard
	}
	
	/// Waits on this condition variable for a notification, timing out after a specified duration.
    ///
    /// The semantics of this function are equivalent to [`wait`] except that the thread will be blocked for roughly no longer than `duration`.
    /// This method should not be used for precise timing due to anomalies such as preemption or platform differences that may not cause the maximum amount of time waited to be precisely `duration`.
    ///
    /// Note that the best effort is made to ensure that the time waited is measured with a monotonic clock, and not affected by the changes made to the system time.
    ///
    /// The returned [`TimedOut`] value indicates if the timeout is known to have elapsed.
    ///
    /// Like [`wait`], the lock specified will be re-acquired when this function returns, regardless of whether the timeout elapsed or not.
    ///
    /// [`wait`]: #method.wait
    /// [`TimedOut`]: struct.TimedOut.html
    ///
	#[inline(always)]
	pub fn wait_timeout<'mutex, T: CtoSafe>(&self, guard: CtoMutexLockGuard<'mutex, T>, duration: Duration) -> (CtoMutexLockGuard<'mutex, T>, TimedOut)
	{
		let timed_out = self.persistent_memory().wait_timeout(guard.0.mutex.get(), duration);
		(guard, timed_out)
	}
	
	/// Wakes up one blocked thread on this condition variable.
    ///
    /// If there is a blocked thread on this condition variable, then it will
    /// be woken up from its call to [`wait`] or [`wait_timeout`]. Calls to
    /// `notify_one` are not buffered in any way.
    ///
    /// To wake up all threads, see [`notify_all`].
    ///
	#[inline(always)]
	pub fn notify_one(&self)
	{
		self.persistent_memory().notify_one()
	}
	
	/// Wakes up all blocked threads on this condition variable.
    ///
    /// This method will ensure that any current waiters on the condition
    /// variable are awoken. Calls to `notify_all()` are not buffered in any
    /// way.
    ///
    /// To wake up only one thread, see [`notify_one`].
    ///
    /// [`notify_one`]: #method.notify_one
    ///
	#[inline(always)]
	pub fn notify_all(&self)
	{
		self.persistent_memory().notify_all()
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoConditionVariableInner
	{
		unsafe { &*self.persistent_memory_pointer }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&self) -> &mut CtoConditionVariableInner
	{
		unsafe { &mut *self.persistent_memory_pointer }
	}
}
