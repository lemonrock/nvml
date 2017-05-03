// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ReadWriteLockablePersistentObjectMemory: Persistable
{
	#[doc(hidden)]
	#[inline(always)]
	fn _pmemReadWriteLock(&mut self) -> &mut PMEMrwlock;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _readWriteLock<'a>(&'a mut self) -> ReadWriteLock<'a, Self>
	{
		ReadWriteLock::new(self.persistentObjectPool(), self._pmemReadWriteLock(), self)
	}
	
	#[inline(always)]
	fn readLock<'a>(&'a mut self) -> ReadLockUnlock<'a, Self>
	{
		self._readWriteLock().readLock()
	}
	
	#[inline(always)]
	fn tryReadLock<'a>(&'a mut self) -> Option<ReadLockUnlock<'a, Self>>
	{
		self._readWriteLock().tryReadLock()
	}
	
	#[inline(always)]
	fn timedReadLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<ReadLockUnlock<'a, Self>>
	{
		self._readWriteLock().timedReadLock(absoluteTimeOut)
	}
	
	#[inline(always)]
	fn writeLockInTransaction<'a>(&'a mut self, transaction: Transaction)
	{
		self._readWriteLock().writeLockInTransaction(transaction)
	}
	
	#[inline(always)]
	fn writeLock<'a>(&'a mut self) -> WriteLockUnlock<'a, Self>
	{
		self._readWriteLock().writeLock()
	}
	
	#[inline(always)]
	fn tryWriteLock<'a>(&'a mut self) -> Option<WriteLockUnlock<'a, Self>>
	{
		self._readWriteLock().tryWriteLock()
	}
	
	#[inline(always)]
	fn timedWriteLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<WriteLockUnlock<'a, Self>>
	{
		self._readWriteLock().timedWriteLock(absoluteTimeOut)
	}
}
