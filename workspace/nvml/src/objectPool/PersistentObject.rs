// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct PersistentObject<T: Persistable>
{
	oid: PMEMoid,
	phantomData: PhantomData<T>
}

impl<T: Persistable> PartialOrd for PersistentObject<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &PersistentObject<T>) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl<T: Persistable> Ord for PersistentObject<T>
{
	#[inline(always)]
	fn cmp(&self, other: &PersistentObject<T>) -> Ordering
	{
		let ourOid = self.oid;
		let otherOid = other.oid;
		ourOid.pool_uuid_lo.cmp(&otherOid.pool_uuid_lo).then_with(|| ourOid.off.cmp(&otherOid.off))
	}
}

impl<T: Persistable> PartialEq for PersistentObject<T>
{
	#[inline(always)]
	fn eq(&self, other: &PersistentObject<T>) -> bool
	{
		self.oid.equals(&other.oid)
	}
}

impl<T: Persistable> Eq for PersistentObject<T>
{
}

impl<T: Persistable> Hash for PersistentObject<T>
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.oid.pool_uuid_lo.hash(state);
		self.oid.off.hash(state);
	}
}

impl<T: Persistable> Debug for PersistentObject<T>
{
	#[inline(always)]
	default fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		if unlikely(self.is_null())
		{
			write!(f, "PersistentObject({}, {}, NULL)", T::TypeNumber, self.typeNumber())
		}
		else
		{
			write!(f, "PersistentObject({}, {}, OID({}, {}))", T::TypeNumber, self.typeNumber(), self.oid.pool_uuid_lo, self.oid.off)
		}
	}
}

impl<T: Persistable> Debug for PersistentObject<T>
where T: Debug
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		if unlikely(self.is_null())
		{
			write!(f, "PersistentObject({}, {}, NULL)", T::TypeNumber, self.typeNumber())
		}
		else
		{
			write!(f, "PersistentObject({}, {}, {:?})", T::TypeNumber, self.typeNumber(), self.deref())
		}
	}
}

impl<T: Persistable> Display for PersistentObject<T>
where T: Display
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		if unlikely(self.is_null())
		{
			write!(f, "NULL")
		}
		else
		{
			self.deref().fmt(f)
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

impl<T: Persistable> Iterator for PersistentObject<T>
{
	type Item = PersistentObject<T>;
	
	fn next(&mut self) -> Option<PersistentObject<T>>
	{
		loop
		{
			// pmemobj_next is safe if self.oid.is_null()
			let next = unsafe { pmemobj_next(self.oid) };
			if unlikely(next.is_null())
			{
				return None;
			}
			// Not necessarily true, but if we're treating a pool as a vectored list then we ought to optimise for this branch
			if likely(next.typeNumber() == T::TypeNumber)
			{
				return Some(PersistentObject::new(self.oid));
			}
		}
	}
}

impl<T: Persistable> OID for PersistentObject<T>
{
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.oid.is_null()
	}
	
	#[inline(always)]
	fn equals(&self, other: &Self) -> bool
	{
		self.oid.equals(&other.oid)
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
	/*
		pub fn pmemobj_root_construct(pop: *mut PMEMobjpool, size: usize, constructor: pmemobj_constr, arg: *mut c_void) -> PMEMoid;
		pub fn pmemobj_root_size(pop: *mut PMEMobjpool) -> usize;
	*/
	
	// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd
	#[inline(always)]
	pub fn allocateUninitializedAndConstruct(&mut self, objectPool: *mut PMEMobjpool) -> Result<(), GenericError>
	{
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let size = T::size();
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		let typeNumber = T::TypeNumber;
		debug_assert!(typeNumber != 0, "typeNumber can not be zero, ie root, for this call");
		
		#[thread_local] static mut CapturedPanic: Option<Box<Any + Send + 'static>> = None;
		
		unsafe extern "C" fn constructor<T: Persistable>(pop: *mut PMEMobjpool, ptr: *mut c_void, arg: *mut c_void) -> c_int
		{
			let result = catch_unwind(AssertUnwindSafe(||
			{
				debug_assert!(!pop.is_null(), "pop is null");
				debug_assert!(!ptr.is_null(), "ptr is null");
				debug_assert!(arg.is_null(), "arg is not null");
				
				T::initialize(ptr as *mut T, pop)
			}));
			
			match result
			{
				Ok(()) => 0,
				
				Err(panicPayload) =>
				{
					CapturedPanic = Some(panicPayload);
					-1
				},
			}
		}
		
		let result = unsafe { pmemobj_alloc(objectPool, &mut self.oid, size, typeNumber, Some(constructor::<T>), null_mut()) };
		if likely(result == 0)
		{
			debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set yet result was 0 (Ok)");
			
			Ok(())
		}
		else if likely(result == -1)
		{
			let osErrorNumber = errno().0;
			match osErrorNumber
			{
				E::ECANCELED =>
				{
					if let Some(capturedPanic) = unsafe { replace(&mut CapturedPanic, None) }
					{
						resume_unwind(capturedPanic);
					}
					Err(GenericError::new(osErrorNumber, pmemobj_errormsg, "pmemobj_alloc"))
				},
				
				_ =>
				{
					debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set and error was '{}'", osErrorNumber);
					
					Err(GenericError::new(osErrorNumber, pmemobj_errormsg, "pmemobj_alloc"))
				}
			}
		}
		else
		{
			panic!("pmemobj_alloc() failed with unexpected result '{}'", result);
		}
	}
	
//	// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd
//	#[inline(always)]
//	fn allocateZeroed(&mut self, objectPool: *mut PMEMobjpool) -> Result<(), GenericError>
//	{
//		debug_assert!(!objectPool.is_null(), "objectPool is null");
//
//		let size = T::size();
//		debug_assert!(size != 0, "size can not be zero");
//		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
//
//		let typeNumber = T::TypeNumber;
//		debug_assert!(typeNumber != 0, "typeNumber can not be zero, ie root, for this call");
//
//		let result = unsafe { pmemobj_zalloc(objectPool, &mut self.oid, size, typeNumber) };
//		if likely(result == 0)
//		{
//			Ok(())
//		}
//		else if likely(result == -1)
//		{
//			Err(GenericError::new(errno().0, pmemobj_errormsg, "pmemobj_zalloc"))
//		}
//		else
//		{
//			panic!("pmemobj_zalloc() failed with unexpected result '{}'", result);
//		}
//	}
	
	#[inline(always)]
	fn new(oid: PMEMoid) -> Self
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
