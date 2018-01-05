// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A structure that represents a Mutex lock.
pub struct MutexLock<'a, T: Persistable + 'a>
{
	object_pool: *mut PMEMobjpool,
	mutex: *mut PMEMmutex,
	object: &'a mut T
}

impl<'a, T: Persistable> MutexLock<'a, T>
{
	#[inline(always)]
	fn new(object_pool: *mut PMEMobjpool, mutex: *mut PMEMmutex, object: &'a mut T) -> Self
	{
		debug_assert!(!object_pool.is_null(), "object_pool is null");
		debug_assert!(!mutex.is_null(), "mutex is null");
		
		Self
		{
			object_pool,
			mutex,
			object,
		}
	}
	
	/// Obtain a mutex lock within a transaction.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn mutex_in_transaction(self, transaction: Transaction)
	{
		let result = unsafe { pmemobj_tx_lock(pobj_tx_param::TX_PARAM_MUTEX, self.mutex as *mut c_void) };
		if likely(result == 0)
		{
			return;
		}
		Self::lock_error_handling(result);
	}
	
	/// Obtain a mutex lock.
	#[inline(always)]
	pub fn mutex(self) -> MutexUnlock<'a, T>
	{
		let result = unsafe { pmemobj_mutex_lock(self.object_pool, self.mutex) };
		if likely(result == 0)
		{
			return MutexUnlock(self);
		}
		Self::lock_error_handling(result)
	}
	
	/// Try to obtain a mutex lock.
	#[inline(always)]
	pub fn try_mutex(self) -> Option<MutexUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_mutex_trylock(self.object_pool, self.mutex) };
		if likely(result == 0)
		{
			return Some(MutexUnlock(self));
		}
		
		match result
		{
			EBUSY => None,
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EOWNERDEAD => panic!("This should only occur with a Robust mutex, which it is believed libpmemobj is not using"),
			ENOTRECOVERABLE => panic!("This does not occur on Linux or Mac OS X"),
			EINVAL => panic!("object_pool or mutex was null or mutex was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Obtain a mutex lock. Time out if the mutex lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_mutex(self, absolute_time_out: &timespec) -> Option<MutexUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_mutex_timedlock(self.object_pool, self.mutex, absolute_time_out) };
		if likely(result == 0)
		{
			return Some(MutexUnlock(self));
		}
		
		match result
		{
			ETIMEDOUT => None,
			
			EDEADLK => panic!("Deadlock"),
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or mutex was null or mutex was invalid or absolute_time_out is out-of-range (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	fn unlock(&self)
	{
		let result = unsafe { pmemobj_mutex_unlock(self.object_pool, self.mutex) };
		if likely(result == 0)
		{
			return;
		}
		
		match result
		{
			EINVAL => panic!("object_pool or mutex was null or mutex was invalid (none of these things should occur)"),
			EPERM => panic!("Current thread does not hold this mutex lock"),
			EAGAIN => panic!("EAGAIN is no longer part of POSIX for pthread_mutex_unlock"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	fn lock_error_handling(result: c_int) -> MutexUnlock<'a, T>
	{
		match result
		{
			EDEADLK => panic!("Deadlock"),
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EOWNERDEAD => panic!("This should only occur with a Robust mutex, which it is believed libpmemobj is not using"),
			ENOTRECOVERABLE => panic!("This does not occur on Linux or Mac OS X"),
			EINVAL => panic!("object_pool or mutex was null or mutex was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
}
