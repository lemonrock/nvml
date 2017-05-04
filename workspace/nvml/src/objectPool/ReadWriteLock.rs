// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ReadWriteLock<'a, T: Persistable + 'a>
{
	objectPool: *mut PMEMobjpool,
	readWriteLock: *mut PMEMrwlock,
	object: &'a mut T
}

/*
	pub fn pmemobj_tx_lock(type_: pobj_tx_param, lockp: *mut c_void) -> c_int;
*/

impl<'a, T: Persistable> ReadWriteLock<'a, T>
{
	#[inline(always)]
	fn new(objectPool: *mut PMEMobjpool, readWriteLock: *mut PMEMrwlock, object: &'a mut T) -> Self
	{
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		debug_assert!(!readWriteLock.is_null(), "readWriteLock is null");
		
		Self
		{
			objectPool: objectPool,
			readWriteLock: readWriteLock,
			object: object,
		}
	}
	
	#[inline(always)]
	pub fn readLock(self) -> ReadLockUnlock<'a, T>
	{
		let result = unsafe { pmemobj_rwlock_rdlock(self.objectPool, self.readWriteLock) };
		if likely(result == 0)
		{
			return ReadLockUnlock(self);
		}
		
		match result
		{
			E::EDEADLK => panic!("Deadlock"),
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or readWriteLock was null or readWriteLock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn tryReadLock(self) -> Option<ReadLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_tryrdlock(self.objectPool, self.readWriteLock) };
		if likely(result == 0)
		{
			return Some(ReadLockUnlock(self));
		}
		
		match result
		{
			E::EBUSY => None,
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or readWriteLock was null or readWriteLock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn timedReadLock(self, absoluteTimeOut: &timespec) -> Option<ReadLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_timedwrlock(self.objectPool, self.readWriteLock, absoluteTimeOut) };
		if likely(result == 0)
		{
			return Some(ReadLockUnlock(self));
		}
		
		match result
		{
			E::ETIMEDOUT => None,
			
			E::EDEADLK => panic!("Deadlock"),
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or readWriteLock was null or readWriteLock was invalid or absoluteTimeOut is out-of-range (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn writeLockInTransaction(self, transaction: Transaction)
	{
		let result = unsafe { pmemobj_tx_lock(pobj_tx_param::TX_PARAM_RWLOCK, self.readWriteLock as *mut c_void) };
		if likely(result == 0)
		{
			return;
		}
		Self::writeLockErrorHandling(result);
	}
	
	#[inline(always)]
	pub fn writeLock(self) -> WriteLockUnlock<'a, T>
	{
		let result = unsafe { pmemobj_rwlock_wrlock(self.objectPool, self.readWriteLock) };
		if likely(result == 0)
		{
			return WriteLockUnlock(self);
		}
		Self::writeLockErrorHandling(result)
	}
	
	#[inline(always)]
	fn writeLockErrorHandling(result: c_int) -> WriteLockUnlock<'a, T>
	{
		match result
		{
			E::EDEADLK => panic!("Deadlock"),
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or readWriteLock was null or readWriteLock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn tryWriteLock(self) -> Option<WriteLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_trywrlock(self.objectPool, self.readWriteLock) };
		if likely(result == 0)
		{
			return Some(WriteLockUnlock(self));
		}
		
		match result
		{
			E::EBUSY => None,
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or readWriteLock was null or readWriteLock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn timedWriteLock(self, absoluteTimeOut: &timespec) -> Option<WriteLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_timedwrlock(self.objectPool, self.readWriteLock, absoluteTimeOut) };
		if likely(result == 0)
		{
			return Some(WriteLockUnlock(self));
		}
		
		match result
		{
			E::ETIMEDOUT => None,
			
			E::EDEADLK => panic!("Deadlock"),
			
			E::EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			E::EINVAL => panic!("objectPool or readWriteLock was null or readWriteLock was invalid or absoluteTimeOut is out-of-range (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	fn unlock(&self)
	{
		let result = unsafe { pmemobj_rwlock_unlock(self.objectPool, self.readWriteLock) };
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
