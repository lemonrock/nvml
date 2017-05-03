// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ConditionVariableMutexLockablePersistentObjectMemory: MutexLockablePersistentObjectMemory
{
	#[doc(hidden)]
	#[inline(always)]
	fn _pmemConditionVariable(&mut self) -> &mut PMEMcond;
	
	#[inline(always)]
	fn lockWithConditionVariable<'a>(&'a mut self) -> (MutexUnlock<'a, Self>, ConditionVariable<'a, Self>)
	{
		let (mutexLock, conditionVariable) = self._construct();
		
		(mutexLock.lock(), conditionVariable)
	}
	
	#[inline(always)]
	fn tryLock<'a>(&'a mut self) -> Option<(MutexUnlock<'a, Self>, ConditionVariable<'a, Self>)>
	{
		let (mutexLock, conditionVariable) = self._construct();
		
		mutexLock.tryLock().map(|mutexUnlock| (mutexUnlock, conditionVariable))
	}
	
	#[inline(always)]
	fn timedLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<(MutexUnlock<'a, Self>, ConditionVariable<'a, Self>)>
	{
		let (mutexLock, conditionVariable) = self._construct();
		
		mutexLock.timedLock(absoluteTimeOut).map(|mutexUnlock| (mutexUnlock, conditionVariable))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _construct<'a>(&'a mut self) -> (MutexLock<'a, Self>, ConditionVariable<'a, Self>)
	{
		let objectPool = self.persistentObjectPool();
		let conditionVariable = ConditionVariable::new(objectPool, self._pmemConditionVariable());
		let mutexLock = MutexLock::new(objectPool, self._pmemMutex(), self);
		
		(mutexLock, conditionVariable)
	}
}
