// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct PersistentObject<T: Persistable>
{
	oid: PMEMoid,
	phantomData: PhantomData<T>
}

impl<T: Persistable> OID for PersistentObject<T>
{
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.oid.is_null()
	}
	
	#[inline(always)]
	fn equals(&self, right: &Self) -> bool
	{
		self.oid.equals(&right.oid)
	}
	
	#[inline(always)]
	fn persistentObjectPool(&self) -> *mut PMEMobjpool
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		let objectPool = self.oid.persistentObjectPool();
		debug_assert!(!objectPool.is_null(), "How is the objectPool null for an allocated object?");
		objectPool
	}
	
	#[inline(always)]
	fn allocatedUsefulSize(&self) -> size_t
	{
		self.oid.allocatedUsefulSize()
	}
	
	#[inline(always)]
	fn typeNumber(&self) -> TypeNumber
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		
		self.oid.typeNumber()
	}
	
	#[inline(always)]
	fn address(&self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		
		let address = self.oid.address();
		debug_assert!(!address.is_null(), "How is the address null for an allocated object?");
		address
	}
	
	#[inline(always)]
	fn next(&self) -> Self
	{
		if unlikely(self.is_null())
		{
			return PersistentObject
			{
				oid: self.oid,
				phantomData: PhantomData,
			};
		}
		
		let mut next;
		while
		{
			next = self.oid.next();
			!next.is_null() && next.typeNumber() != T::TypeNumber
		}
		{
		}
		
		PersistentObject
		{
			oid: next,
			phantomData: PhantomData,
		}
	}
}

/// It is possible to violate aliasing rules
impl<T: Persistable> Deref for PersistentObject<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		
		unsafe { &*self.as_ptr() }
	}
}

/// It is possible to violate aliasing rules
impl<T: Persistable> DerefMut for PersistentObject<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		
		unsafe { &mut *self.as_ptr() }
	}
}

impl<T: Persistable> PersistentObject<T>
where T: ReadWriteLockablePersistable
{
	#[inline(always)]
	fn readWriteLock<'a>(&'a mut self) -> ReadWriteLock<'a, T>
	{
		let persistentObjectPool = self.persistentObjectPool();
		let object = self.deref_mut();
		ReadWriteLock::new(persistentObjectPool, object.pmemReadWriteLock(), object)
	}
	
	#[inline(always)]
	pub fn readLock<'a>(&'a mut self) -> ReadLockUnlock<'a, T>
	{
		self.readWriteLock().readLock()
	}
	
	#[inline(always)]
	pub fn tryReadLock<'a>(&'a mut self) -> Option<ReadLockUnlock<'a, T>>
	{
		self.readWriteLock().tryReadLock()
	}
	
	#[inline(always)]
	pub fn timedReadLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<ReadLockUnlock<'a, T>>
	{
		self.readWriteLock().timedReadLock(absoluteTimeOut)
	}
	
	#[inline(always)]
	pub fn writeLockInTransaction<'a>(&'a mut self, transaction: Transaction)
	{
		self.readWriteLock().writeLockInTransaction(transaction)
	}
	
	#[inline(always)]
	pub fn writeLock<'a>(&'a mut self) -> WriteLockUnlock<'a, T>
	{
		self.readWriteLock().writeLock()
	}
	
	#[inline(always)]
	pub fn tryWriteLock<'a>(&'a mut self) -> Option<WriteLockUnlock<'a, T>>
	{
		self.readWriteLock().tryWriteLock()
	}
	
	#[inline(always)]
	pub fn timedWriteLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<WriteLockUnlock<'a, T>>
	{
		self.readWriteLock().timedWriteLock(absoluteTimeOut)
	}
}

impl<T: Persistable> PersistentObject<T>
where T: MutexLockablePersistable
{
	#[inline(always)]
	fn mutexLock<'a>(&'a mut self) -> MutexLock<'a, T>
	{
		let persistentObjectPool = self.persistentObjectPool();
		let object = self.deref_mut();
		MutexLock::new(persistentObjectPool, object.pmemMutex(), object)
	}
	
	#[inline(always)]
	pub fn lockInTransaction<'a>(&'a mut self, transaction: Transaction)
	{
		self.mutexLock().lockInTransaction(transaction)
	}
	
	#[inline(always)]
	pub fn lock<'a>(&'a mut self) -> MutexUnlock<'a, T>
	{
		self.mutexLock().lock()
	}
	
	#[inline(always)]
	pub fn tryLock<'a>(&'a mut self) -> Option<MutexUnlock<'a, T>>
	{
		self.mutexLock().tryLock()
	}
	
	#[inline(always)]
	pub fn timedLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<MutexUnlock<'a, T>>
	{
		self.mutexLock().timedLock(absoluteTimeOut)
	}
}

impl<T: Persistable> PersistentObject<T>
where T: ConditionVariableMutexLockablePersistable
{
	#[inline(always)]
	pub fn lockWithConditionVariable<'a>(&'a mut self) -> (MutexUnlock<'a, T>, ConditionVariable<'a, T>)
	{
		let (mutexLock, conditionVariable) = self.construct();
		
		(mutexLock.lock(), conditionVariable)
	}
	
	#[inline(always)]
	pub fn tryLockWithConditionVariable<'a>(&'a mut self) -> Option<(MutexUnlock<'a, T>, ConditionVariable<'a, T>)>
	{
		let (mutexLock, conditionVariable) = self.construct();
		
		mutexLock.tryLock().map(|mutexUnlock| (mutexUnlock, conditionVariable))
	}
	
	#[inline(always)]
	pub fn timedLockWithConditionVariable<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<(MutexUnlock<'a, T>, ConditionVariable<'a, T>)>
	{
		let (mutexLock, conditionVariable) = self.construct();
		
		mutexLock.timedLock(absoluteTimeOut).map(|mutexUnlock| (mutexUnlock, conditionVariable))
	}
	
	#[inline(always)]
	fn construct<'a>(&'a mut self) -> (MutexLock<'a, T>, ConditionVariable<'a, T>)
	{
		let objectPool = self.persistentObjectPool();
		let object = self.deref_mut();
		let conditionVariable = ConditionVariable::new(objectPool, object.pmemConditionVariable());
		let mutexLock = MutexLock::new(objectPool, object.pmemMutex(), object);
		
		(mutexLock, conditionVariable)
	}
}

impl<T: Persistable> PersistentObject<T>
{
	#[inline(always)]
	pub fn allocateZeroed(&mut self, objectPool: &ObjectPool)
	{
		debug_assert!(self.oid.is_null(), "We should not be allocating zero'd without free-ing first");
		
		let result = unsafe { pmemobj_zalloc(objectPool.0, &mut self.oid, T::size(), T::TypeNumber) };
		debug_assert!(result == 0, "result was {}", result);
		
		objectPool.allocateZeroed::<T>(&mut self.oid)
	}
	
	#[inline(always)]
	pub fn new(oid: PMEMoid) -> Self
	{
		Self
		{
			oid: oid,
			phantomData: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn as_ptr(&self) -> *mut T
	{
		self.address() as *mut _
	}
	
	#[inline(always)]
	pub fn persistentObjectPool(&self) -> *mut PMEMobjpool
	{
		let persistentObjectPool = self.oid.persistentObjectPool();
		debug_assert!(!persistentObjectPool.is_null(), "This object does not have a valid OID");
		persistentObjectPool
	}
	
	#[inline(always)]
	pub fn free(&mut self)
	{
		unsafe { pmemobj_free(&mut self.oid) };
		self.oid = unsafe { OID_NULL };
	}
	
	#[inline(always)]
	pub fn freeInTransaction(self, transaction: Transaction) -> c_int
	{
		transaction.free(self.oid)
	}
	
	/// size can be zero
	#[inline(always)]
	pub fn addRangeSnapshotInTransaction(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= T::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, T::size());
		
		transaction.addRangeSnapshotInTransaction(self.oid, offset, size)
	}
	
	/// Can only be called from a work() function
	/// If returns !=0 then the transaction will have been aborted; return immediately from work() function
	/// No checks are made for offset or size
	/// size can be zero
	#[inline(always)]
	pub fn addRangeSnapshotInTransactionWithoutFlush(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= T::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, T::size());
		
		transaction.addRangeSnapshotInTransactionWithoutFlush(self.oid, offset, size)
	}
}
