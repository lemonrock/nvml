// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A structure that represents a Read-Write lock.
pub struct ReadWriteLock<'a, T: Persistable + 'a>
{
	object_pool: *mut PMEMobjpool,
	read_write_lock: *mut PMEMrwlock,
	object: &'a mut T
}

impl<'a, T: Persistable> ReadWriteLock<'a, T>
{
	#[inline(always)]
	fn new(object_pool: *mut PMEMobjpool, read_write_lock: *mut PMEMrwlock, object: &'a mut T) -> Self
	{
		debug_assert!(object_pool.is_not_null(), "object_pool is null");
		debug_assert!(read_write_lock.is_not_null(), "read_write_lock is null");
		
		Self
		{
			object_pool,
			read_write_lock,
			object,
		}
	}
	
	/// Obtain a read lock.
	#[inline(always)]
	pub fn read(self) -> ReadLockUnlock<'a, T>
	{
		let result = unsafe { pmemobj_rwlock_rdlock(self.object_pool, self.read_write_lock) };
		if likely(result == 0)
		{
			return ReadLockUnlock(self);
		}
		
		match result
		{
			EDEADLK => panic!("Deadlock"),
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or read_write_lock was null or read_write_lock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Try to obtain a read lock.
	#[inline(always)]
	pub fn try_read(self) -> Option<ReadLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_tryrdlock(self.object_pool, self.read_write_lock) };
		if likely(result == 0)
		{
			return Some(ReadLockUnlock(self));
		}
		
		match result
		{
			EBUSY => None,
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or read_write_lock was null or read_write_lock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Obtain a read lock. Time out if the read lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_read(self, absolute_time_out: &timespec) -> Option<ReadLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_timedwrlock(self.object_pool, self.read_write_lock, absolute_time_out) };
		if likely(result == 0)
		{
			return Some(ReadLockUnlock(self));
		}
		
		match result
		{
			ETIMEDOUT => None,
			
			EDEADLK => panic!("Deadlock"),
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or read_write_lock was null or read_write_lock was invalid or absolute_time_out is out-of-range (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Obtain a write lock within a transaction.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn write_in_transaction(self, transaction: Transaction)
	{
		let result = unsafe { pmemobj_tx_lock(pobj_tx_param_TX_PARAM_RWLOCK, self.read_write_lock as *mut c_void) };
		if likely(result == 0)
		{
			return;
		}
		Self::write_lock_error_handling(result);
	}
	
	/// Obtain a write lock.
	#[inline(always)]
	pub fn write(self) -> WriteLockUnlock<'a, T>
	{
		let result = unsafe { pmemobj_rwlock_wrlock(self.object_pool, self.read_write_lock) };
		if likely(result == 0)
		{
			return WriteLockUnlock(self);
		}
		Self::write_lock_error_handling(result)
	}
	
	/// Try to obtain a write lock.
	#[inline(always)]
	pub fn try_write(self) -> Option<WriteLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_trywrlock(self.object_pool, self.read_write_lock) };
		if likely(result == 0)
		{
			return Some(WriteLockUnlock(self));
		}
		
		match result
		{
			EBUSY => None,
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or read_write_lock was null or read_write_lock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Obtain a write lock. Time out if the write lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_write(self, absolute_time_out: &timespec) -> Option<WriteLockUnlock<'a, T>>
	{
		let result = unsafe { pmemobj_rwlock_timedwrlock(self.object_pool, self.read_write_lock, absolute_time_out) };
		if likely(result == 0)
		{
			return Some(WriteLockUnlock(self));
		}
		
		match result
		{
			ETIMEDOUT => None,
			
			EDEADLK => panic!("Deadlock"),
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or read_write_lock was null or read_write_lock was invalid or absolute_time_out is out-of-range (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	fn unlock(&self)
	{
		let result = unsafe { pmemobj_rwlock_unlock(self.object_pool, self.read_write_lock) };
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
	fn write_lock_error_handling(result: c_int) -> WriteLockUnlock<'a, T>
	{
		match result
		{
			EDEADLK => panic!("Deadlock"),
			
			EAGAIN => panic!("EAGAIN; too many locks of the same lock in this thread"),
			EINVAL => panic!("object_pool or read_write_lock was null or read_write_lock was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
}
