// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A structure that represents a Condition Variable.
pub struct ConditionVariable<'a, T: Persistable + 'a>
{
	object_pool: *mut PMEMobjpool,
	condition_variable: *mut PMEMcond,
	phantom_data: PhantomData<&'a mut T>,
}

impl<'a, T: Persistable> ConditionVariable<'a, T>
{
	#[inline(always)]
	fn new(object_pool: *mut PMEMobjpool, condition_variable: *mut PMEMcond) -> Self
	{
		debug_assert!(!object_pool.is_null(), "object_pool is null");
		debug_assert!(!condition_variable.is_null(), "condition_variable is null");
		
		Self
		{
			object_pool,
			condition_variable,
			phantom_data: PhantomData,
		}
	}
	
	/// Always recheck whatever predicate we were waiting on after this function returns due to spurious wake ups.
	#[inline(always)]
	pub fn wait(&self, locked_mutex: MutexUnlock<'a, T>) -> MutexUnlock<'a, T>
	{
		let result = unsafe { pmemobj_cond_wait(self.object_pool, self.condition_variable, locked_mutex.0.mutex) };
		if likely(result == 0)
		{
			return locked_mutex;
		}
		
		match result
		{
			EINVAL => panic!("object_pool or condition_variable was null or conditionVariable was invalid (none of these things should occur)"),
			EPERM => panic!("Mutex was not owned by calling thread"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Always recheck whatever predicate we were waiting on after this function returns due to spurious wake ups and time out expiry being coincidental with signalOne() or signalAll().
	#[inline(always)]
	pub fn timed_wait(&self, locked_mutex: MutexUnlock<'a, T>, absolute_time_out: &timespec) -> (MutexUnlock<'a, T>, bool)
	{
		let result = unsafe { pmemobj_cond_timedwait(self.object_pool, self.condition_variable, locked_mutex.0.mutex, absolute_time_out) };
		if likely(result == 0)
		{
			return (locked_mutex, true);
		}
		
		match result
		{
			ETIMEDOUT => (locked_mutex, false),
			
			EINVAL => panic!("object_pool or condition_variable was null or conditionVariable was invalid or absolute_time_out was out of range (none of these things should occur)"),
			EPERM => panic!("Mutex was not owned by calling thread"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// signal all waiting threads.
	#[inline(always)]
	pub fn signal_all(&self, locked_mutex: MutexUnlock<'a, T>)
	{
		Self::signal(unsafe { pmemobj_cond_broadcast(self.object_pool, self.condition_variable) }, locked_mutex);
	}
	
	/// signal one of the waiting threads.
	#[inline(always)]
	pub fn signal_one(&self, locked_mutex: MutexUnlock<'a, T>)
	{
		Self::signal(unsafe { pmemobj_cond_signal(self.object_pool, self.condition_variable) }, locked_mutex);
	}
	
	#[inline(always)]
	fn signal(result: c_int, locked_mutex: MutexUnlock<'a, T>)
	{
		if likely(result == 0)
		{
			drop(locked_mutex);
			return;
		}
		
		match result
		{
			EINVAL => panic!("object_pool or condition_variable was null or conditionVariable was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
}
