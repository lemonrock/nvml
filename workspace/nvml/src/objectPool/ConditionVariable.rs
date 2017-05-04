// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ConditionVariable<'a, T: Persistable + 'a>
{
	objectPool: *mut PMEMobjpool,
	conditionVariable: *mut PMEMcond,
	phantomData: PhantomData<&'a mut T>,
}

impl<'a, T: Persistable> ConditionVariable<'a, T>
{
	#[inline(always)]
	fn new(objectPool: *mut PMEMobjpool, conditionVariable: *mut PMEMcond) -> Self
	{
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		debug_assert!(!conditionVariable.is_null(), "conditionVariable is null");
		
		Self
		{
			objectPool: objectPool,
			conditionVariable: conditionVariable,
			phantomData: PhantomData,
		}
	}
	
	/// Always recheck whatever predicate we were waiting on after this function returns due to spurious wake ups
	#[inline(always)]
	pub fn wait(&self, lockedMutex: MutexUnlock<'a, T>) -> MutexUnlock<'a, T>
	{
		let result = unsafe { pmemobj_cond_wait(self.objectPool, self.conditionVariable, lockedMutex.0.mutex) };
		if likely(result == 0)
		{
			return lockedMutex;
		}
		
		match result
		{
			E::EINVAL => panic!("objectPool or conditionVariable was null or conditionVariable was invalid (none of these things should occur)"),
			E::EPERM => panic!("Mutex was not owned by calling thread"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	/// Always recheck whatever predicate we were waiting on after this function returns due to spurious wake ups and time out expiry being coincidental with signalOne() or signalAll()
	#[inline(always)]
	pub fn timedWait(&self, lockedMutex: MutexUnlock<'a, T>, absoluteTimeOut: &timespec) -> (MutexUnlock<'a, T>, bool)
	{
		let result = unsafe { pmemobj_cond_timedwait(self.objectPool, self.conditionVariable, lockedMutex.0.mutex, absoluteTimeOut) };
		if likely(result == 0)
		{
			return (lockedMutex, true);
		}
		
		match result
		{
			E::ETIMEDOUT => (lockedMutex, false),
			
			E::EINVAL => panic!("objectPool or conditionVariable was null or conditionVariable was invalid or absoluteTimeOut was out of range (none of these things should occur)"),
			E::EPERM => panic!("Mutex was not owned by calling thread"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
	
	#[inline(always)]
	pub fn signalAll(&self, lockedMutex: MutexUnlock<'a, T>)
	{
		Self::signal(unsafe { pmemobj_cond_broadcast(self.objectPool, self.conditionVariable) }, lockedMutex);
	}
	
	#[inline(always)]
	pub fn signalOne(&self, lockedMutex: MutexUnlock<'a, T>)
	{
		Self::signal(unsafe { pmemobj_cond_signal(self.objectPool, self.conditionVariable) }, lockedMutex);
	}
	
	#[inline(always)]
	fn signal(result: c_int, lockedMutex: MutexUnlock<'a, T>)
	{
		if likely(result == 0)
		{
			drop(lockedMutex);
			return;
		}
		
		match result
		{
			E::EINVAL => panic!("objectPool or conditionVariable was null or conditionVariable was invalid (none of these things should occur)"),
			
			_ => panic!("Unexpected error '{}'", result),
		}
	}
}
