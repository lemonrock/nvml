// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A structure that represents a Mutex lock.
pub struct MutexLock<'a, T: Persistable + 'a>
{
	objectPool: *mut PMEMobjpool,
	mutex: *mut PMEMmutex,
	object: &'a mut T
}

impl<'a, T: Persistable> MutexLock<'a, T>
{
	#[inline(always)]
	fn new(objectPool: *mut PMEMobjpool, mutex: *mut PMEMmutex, object: &'a mut T) -> Self
	{
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		debug_assert!(!mutex.is_null(), "mutex is null");
		
		Self
		{
			objectPool,
			mutex,
			object,
		}
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn lockInTransaction(self, transaction: Transaction)
	{
		let result = unsafe { pmemobj_tx_lock(pobj_tx_param::TX_PARAM_MUTEX, self.mutex as *mut c_void) };
		if likely(result == 0)
		{
			return;
		}
		Self::lockErrorHandling(result);
	}
	
	#[inline(always)]
	pub fn lock(self) -> MutexUnlock<'a, T>
	{
		let result = unsafe { pmemobj_mutex_lock(self.objectPool, self.mutex) };
		if likely(result == 0)
		{
			return MutexUnlock(self);
		}
		Self::lockErrorHandling(result)
	}
	
	#[inline(always)]
	fn lockErrorHandling(result: c_int) -> MutexUnlock<'a, T>
	{
		match result
		{
			E::EDEADLK => panic!("Deadlock"),
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EOWNERDEAD => panic!("This should only occur with a Robust mutex, which it is believed libpmemobj is not using"),
			E::ENOTRECOVERABLE => panic!("This does not occur on Linux or Mac OS X"),
			E::EINVAL => panic!("objectPool or mutex was null or mutex was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn tryLock(self) -> Option<MutexUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_mutex_trylock(self.objectPool, self.mutex) };
		if likely(result == 0)
		{
			return Some(MutexUnlock(self));
		}
		
		match result
		{
			E::EBUSY => None,
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EOWNERDEAD => panic!("This should only occur with a Robust mutex, which it is believed libpmemobj is not using"),
			E::ENOTRECOVERABLE => panic!("This does not occur on Linux or Mac OS X"),
			E::EINVAL => panic!("objectPool or mutex was null or mutex was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn timedLock(self, absoluteTimeOut: &timespec) -> Option<MutexUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_mutex_timedlock(self.objectPool, self.mutex, absoluteTimeOut) };
		if likely(result == 0)
		{
			return Some(MutexUnlock(self));
		}
		
		match result
		{
			E::ETIMEDOUT => None,
			
			E::EDEADLK => panic!("Deadlock"),
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or mutex was null or mutex was invalid or absoluteTimeOut is out-of-range (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	fn unlock(&self)
	{
		let result = unsafe { pmemobj_mutex_unlock(self.objectPool, self.mutex) };
		if likely(result == 0)
		{
			return;
		}
		
		match result
		{
			E::EINVAL => panic!("objectPool or mutex was null or mutex was invalid (none of these things should occur)"),
			E::EPERM => panic!("Current thread does not hold this mutex lock"),
			E::EAGAIN => panic!("EAGAIN is no longer part of POSIX for pthread_mutex_unlock"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
}
