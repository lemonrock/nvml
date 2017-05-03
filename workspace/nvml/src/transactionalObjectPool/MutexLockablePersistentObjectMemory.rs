// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait MutexLockablePersistentObjectMemory: Persistable
{
	#[doc(hidden)]
	#[inline(always)]
	fn _pmemMutex(&mut self) -> &mut PMEMmutex;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _mutexLock<'a>(&'a mut self) -> MutexLock<'a, Self>
	{
		MutexLock::new(self.persistentObjectPool(), self._pmemMutex(), self)
	}
	
	#[inline(always)]
	fn lockInTransaction<'a>(&'a mut self, transaction: Transaction)
	{
		self._mutexLock().lockInTransaction(transaction)
	}
	
	#[inline(always)]
	fn lock<'a>(&'a mut self) -> MutexUnlock<'a, Self>
	{
		self._mutexLock().lock()
	}
	
	#[inline(always)]
	fn tryLock<'a>(&'a mut self) -> Option<MutexUnlock<'a, Self>>
	{
		self._mutexLock().tryLock()
	}
	
	#[inline(always)]
	fn timedLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<MutexUnlock<'a, Self>>
	{
		self._mutexLock().timedLock(absoluteTimeOut)
	}
}
