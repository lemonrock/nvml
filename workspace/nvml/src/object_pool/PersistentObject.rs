// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A wrapper type that acts as a reference to a Persistable.
/// Persistent objects can be iterated upon, giving the next stored instance, if any.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PersistentObject<T: Persistable>
{
	oid: PMEMoid,
	phantom_data: PhantomData<T>
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
		let our_oid = self.oid;
		let other_oid = other.oid;
		our_oid.pool_uuid_lo.cmp(&other_oid.pool_uuid_lo).then_with(|| our_oid.off.cmp(&other_oid.off))
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
			write!(f, "PersistentObject({}, {}, NULL)", T::TypeNumber, self.type_number())
		}
		else
		{
			write!(f, "PersistentObject({}, {}, OID({}, {}))", T::TypeNumber, self.type_number(), self.oid.pool_uuid_lo, self.oid.off)
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
			write!(f, "PersistentObject({}, {}, NULL)", T::TypeNumber, self.type_number())
		}
		else
		{
			write!(f, "PersistentObject({}, {}, {:?})", T::TypeNumber, self.type_number(), self.deref())
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

/// It is possible to violate aliasing rules.
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

/// It is possible to violate aliasing rules.
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
			if likely(next.type_number() == T::TypeNumber)
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
	fn object_pool(&self) -> *mut PMEMobjpool
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		
		let object_pool = self.oid.object_pool();
		debug_assert!(!object_pool.is_null(), "How is the object_pool null for an allocated object?");
		
		object_pool
	}
	
	#[inline(always)]
	fn allocated_useful_size(&self) -> size_t
	{
		self.oid.allocated_useful_size()
	}
	
	#[inline(always)]
	fn type_number(&self) -> TypeNumber
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		
		self.oid.type_number()
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
	/// Obtain a read lock.
	#[inline(always)]
	pub fn read<'a>(&'a mut self) -> ReadLockUnlock<'a, T>
	{
		self.read_write_lock().read()
	}
	
	/// Try to obtain a read lock.
	#[inline(always)]
	pub fn try_read<'a>(&'a mut self) -> Option<ReadLockUnlock<'a, T>>
	{
		self.read_write_lock().try_read()
	}
	
	/// Obtain a read lock. Time out if the read lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_read<'a>(&'a mut self, absolute_time_out: &timespec) -> Option<ReadLockUnlock<'a, T>>
	{
		self.read_write_lock().timed_read(absolute_time_out)
	}
	
	/// Obtain a write lock within a transaction.
	#[inline(always)]
	pub fn write_in_transaction<'a>(&'a mut self, transaction: Transaction)
	{
		self.read_write_lock().write_in_transaction(transaction)
	}
	
	/// Obtain a write lock.
	#[inline(always)]
	pub fn write<'a>(&'a mut self) -> WriteLockUnlock<'a, T>
	{
		self.read_write_lock().write()
	}
	
	/// Try to obtain a write lock.
	#[inline(always)]
	pub fn try_write<'a>(&'a mut self) -> Option<WriteLockUnlock<'a, T>>
	{
		self.read_write_lock().try_write()
	}
	
	/// Obtain a write lock. Time out if the write lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_write<'a>(&'a mut self, absolute_time_out: &timespec) -> Option<WriteLockUnlock<'a, T>>
	{
		self.read_write_lock().timed_write(absolute_time_out)
	}
	
	#[inline(always)]
	fn read_write_lock<'a>(&'a mut self) -> ReadWriteLock<'a, T>
	{
		let persistent_object_pool = self.object_pool();
		let object = self.deref_mut();
		ReadWriteLock::new(persistent_object_pool, object.read_write_lock(), object)
	}
}

impl<T: Persistable> PersistentObject<T>
where T: MutexLockablePersistable
{
	/// Obtain a mutex lock within a transaction.
	#[inline(always)]
	pub fn mutex_in_transaction<'a>(&'a mut self, transaction: Transaction)
	{
		self.mutex_lock().mutex_in_transaction(transaction)
	}
	
	/// Obtain a mutex lock.
	#[inline(always)]
	pub fn mutex<'a>(&'a mut self) -> MutexUnlock<'a, T>
	{
		self.mutex_lock().mutex()
	}
	
	/// Try to obtain a mutex lock.
	#[inline(always)]
	pub fn try_mutex<'a>(&'a mut self) -> Option<MutexUnlock<'a, T>>
	{
		self.mutex_lock().try_mutex()
	}
	
	/// Obtain a mutex lock. Time out if the mutex lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_mutex<'a>(&'a mut self, absolute_time_out: &timespec) -> Option<MutexUnlock<'a, T>>
	{
		self.mutex_lock().timed_mutex(absolute_time_out)
	}
	
	#[inline(always)]
	fn mutex_lock<'a>(&'a mut self) -> MutexLock<'a, T>
	{
		let object_pool = self.object_pool();
		let object = self.deref_mut();
		MutexLock::new(object_pool, object.mutex_lock(), object)
	}
}

impl<T: Persistable> PersistentObject<T>
where T: ConditionVariableMutexLockablePersistable
{
	/// Obtain a mutex lock with a condition variable.
	#[inline(always)]
	pub fn mutex_with_condition_variable<'a>(&'a mut self) -> (MutexUnlock<'a, T>, ConditionVariable<'a, T>)
	{
		let (mutex_lock, condition_variable) = self.mutex_lock_with_condition_variable();
		
		(mutex_lock.mutex(), condition_variable)
	}
	
	/// Try to obtain a mutex lock with a condition variable.
	#[inline(always)]
	pub fn try_mutex_with_condition_variable<'a>(&'a mut self) -> Option<(MutexUnlock<'a, T>, ConditionVariable<'a, T>)>
	{
		let (mutex_lock, condition_variable) = self.mutex_lock_with_condition_variable();
		
		mutex_lock.try_mutex().map(|mutex_unlock| (mutex_unlock, condition_variable))
	}
	
	/// Obtain a mutex lock with a condition variable. Time out if the mutex lock is not obtained in `absolute_time_out`.
	#[inline(always)]
	pub fn timed_mutex_with_condition_variable<'a>(&'a mut self, absolute_time_out: &timespec) -> Option<(MutexUnlock<'a, T>, ConditionVariable<'a, T>)>
	{
		let (mutex_lock, condition_variable) = self.mutex_lock_with_condition_variable();
		
		mutex_lock.timed_mutex(absolute_time_out).map(|mutex_unlock| (mutex_unlock, condition_variable))
	}
	
	#[inline(always)]
	fn mutex_lock_with_condition_variable<'a>(&'a mut self) -> (MutexLock<'a, T>, ConditionVariable<'a, T>)
	{
		let object_pool = self.object_pool();
		let object = self.deref_mut();
		let condition_variable = ConditionVariable::new(object_pool, object.condition_variable());
		let mutex_lock = MutexLock::new(object_pool, object.mutex_lock(), object);
		
		(mutex_lock, condition_variable)
	}
}

#[inline(always)]
fn size<T: Persistable>() -> size_t
{
	let size = T::size();
	debug_assert!(size != 0, "size can not be zero");
	debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
	size
}

impl<T: Persistable> PersistentObject<T>
{
	/// This object as a raw pointer.
	#[inline(always)]
	pub fn as_ptr(&self) -> *mut T
	{
		self.address() as *mut _
	}
	
	/// This object's object pool.
	#[inline(always)]
	pub fn object_pool(&self) -> *mut PMEMobjpool
	{
		let object_pool = self.oid.object_pool();
		debug_assert!(!object_pool.is_null(), "This object does not have a valid OID");
		
		object_pool
	}
	
	/// Allocate and construct the root persistent object.
	/// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd
	#[inline(always)]
	pub fn allocate_uninitialized_and_construct_object_root_object(&mut self, object_pool: *mut PMEMobjpool, arguments: &mut T::Arguments) -> Result<(), PmdkError>
	{
		debug_assert!(T::TypeNumber == 0, "This is not a root type, ie type number is '{}'");
		
		#[inline(always)]
		fn allocate<T: Persistable>(object_pool: *mut PMEMobjpool, oid_pointer: &mut PMEMoid, constructor: pmemobj_constr, arguments: *mut c_void) -> bool
		{
			let size = size::<T>();
			
			let oid = unsafe { pmemobj_root_construct(object_pool, size, constructor, arguments) };
			if likely(!oid.is_null())
			{
				*oid_pointer = oid;
				false
			}
			else
			{
				true
			}
		}
		
		Self::allocate_uninitialized_and_construct_object_internal(object_pool, &mut self.oid, allocate::<T>, arguments)
	}
	
	/// Allocate and construct a persistent object.
	/// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd
	#[inline(always)]
	pub fn allocate_uninitialized_and_construct_object(&mut self, object_pool: *mut PMEMobjpool, arguments: &mut T::Arguments) -> Result<(), PmdkError>
	{
		#[inline(always)]
		fn allocate<T: Persistable>(object_pool: *mut PMEMobjpool, oid_pointer: &mut PMEMoid, constructor: pmemobj_constr, arguments: *mut c_void) -> bool
		{
			let size = size::<T>();
			
			let result = unsafe { pmemobj_alloc(object_pool, oid_pointer, size, T::TypeNumber, constructor, arguments) };
			debug_assert!(result == 0 || result == -1, "result was '{}'", result);
			result == -1
		}
		
		Self::allocate_uninitialized_and_construct_object_internal(object_pool, &mut self.oid, allocate::<T>, arguments)
	}
	
	#[inline(always)]
	fn allocate_uninitialized_and_construct_object_internal<A: FnOnce(*mut PMEMobjpool, &mut PMEMoid, pmemobj_constr, *mut c_void) -> bool>(object_pool: *mut PMEMobjpool, oid: &mut PMEMoid, allocate: A, arguments: &mut T::Arguments) -> Result<(), PmdkError>
	{
		debug_assert!(!object_pool.is_null(), "object_pool is null");
		
		#[thread_local] static mut CapturedPanic: Option<Box<Any + Send + 'static>> = None;
		
		unsafe extern "C" fn constructor<T: Persistable>(object_pool: *mut PMEMobjpool, ptr: *mut c_void, arg: *mut c_void) -> c_int
		{
			let result = catch_unwind(AssertUnwindSafe(||
			{
				debug_assert!(!object_pool.is_null(), "object_pool is null");
				debug_assert!(!ptr.is_null(), "ptr is null");
				debug_assert!(!arg.is_null(), "arg is null");
				
				T::initialize(ptr as *mut T, object_pool, &mut *(arg as *mut T::Arguments))
			}));
			
			match result
			{
				Ok(()) => 0,
				
				Err(panic_payload) =>
				{
					CapturedPanic = Some(panic_payload);
					-1
				},
			}
		}
		
		if unlikely(allocate(object_pool, oid, Some(constructor::<T>), arguments as *mut _ as *mut _))
		{
			let os_error_number = errno().0;
			match os_error_number
			{
				ECANCELED =>
				{
					if let Some(captured_panic) = unsafe { replace(&mut CapturedPanic, None) }
					{
						resume_unwind(captured_panic);
					}
					PmdkError::obj("pmemobj_alloc or pmemobj_root_construct")
				},
				
				_ =>
				{
					debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set and error was '{}'", os_error_number);
					
					PmdkError::obj("pmemobj_alloc or pmemobj_root_construct")
				}
			}
		}
		else
		{
			debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set yet result was 0 (Ok)");
			
			Ok(())
		}
	}
	
	/// Frees this PersistentObject.
	#[inline(always)]
	pub fn free(&mut self)
	{
		unsafe { pmemobj_free(&mut self.oid) };
		self.oid = unsafe { OID_NULL };
	}
	
	/// If returns Err then the transaction will have been aborted; return immediately from work() function.
	/// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn allocate_uninitialized_and_construct_object_in_transaction(&mut self, transaction: Transaction, object_pool: *mut PMEMobjpool, arguments: &mut T::Arguments) -> Result<(), PmdkError>
	{
		self.construct_in_transaction(object_pool, arguments, unsafe { pmemobj_tx_alloc(size::<T>(), T::TypeNumber) })
	}
	
	/// If returns Err then the transaction will have been aborted; return immediately from work() function.
	/// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn allocate_uninitialized_and_construct_object_in_transaction_without_flush(&mut self, transaction: Transaction, object_pool: *mut PMEMobjpool, arguments: &mut T::Arguments) -> Result<(), PmdkError>
	{
		self.construct_in_transaction(object_pool, arguments, unsafe { pmemobj_tx_xalloc(size::<T>(), T::TypeNumber, POBJ_XALLOC_NO_FLUSH) })
	}
	
	/// If returns Err then the transaction will have been aborted; return immediately from work() function.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn free_in_transaction(&mut self, transaction: Transaction) -> Result<(), PmdkError>
	{
		Self::failure_in_transaction(unsafe { pmemobj_tx_free(self.oid) })
	}
	
	/// Adds range snapshot to transaction (implicitly flushes, which, if doing multiple things in a transaction, is inefficient).
	/// If returns Err then the transaction will have been aborted; return immediately from work() function.
	/// size can be zero.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn add_range_snapshot_in_transaction(&self, transaction: Transaction, offset: u64, size: size_t) -> Result<(), PmdkError>
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		debug_assert!(offset + size as u64 <= T::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, T::size());
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		if unlikely(size == 0)
		{
			return Ok(())
		}
		
		Self::failure_in_transaction(unsafe { pmemobj_tx_add_range(self.oid, offset, size) })
	}
	
	/// Adds range snapshot to transaction without a flush.
	/// If returns Err then the transaction will have been aborted; return immediately from work() function.
	/// size can be zero.
	#[allow(unused_variables)]
	#[inline(always)]
	pub fn add_range_snapshot_in_transaction_without_flush(&self, transaction: Transaction, offset: u64, size: size_t) -> Result<(), PmdkError>
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		debug_assert!(offset + size as u64 <= T::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, T::size());
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		if unlikely(size == 0)
		{
			return Ok(())
		}
		
		Self::failure_in_transaction(unsafe { pmemobj_tx_xadd_range(self.oid, offset, size, POBJ_XADD_NO_FLUSH) })
	}
	
	/// Adds self to transaction (implicitly flushes, which, if doing multiple things in a transaction, is inefficient).
	#[inline(always)]
	pub fn add_self_to_transaction(&self, transaction: Transaction) -> Result<(), PmdkError>
	{
		self.add_range_snapshot_in_transaction(transaction, 0, T::size())
	}
	
	/// Adds self to transaction without a flush.
	#[inline(always)]
	pub fn add_self_to_transaction_without_flush(&self, transaction: Transaction) -> Result<(), PmdkError>
	{
		self.add_range_snapshot_in_transaction_without_flush(transaction, 0, T::size())
	}
	
	#[inline(always)]
	fn construct_in_transaction(&mut self, object_pool: *mut PMEMobjpool, arguments: &mut T::Arguments, oid: PMEMoid) -> Result<(), PmdkError>
	{
		if unlikely(oid.is_null())
		{
			PmdkError::obj("pmemobj_tx_xalloc")
		}
		else
		{
			unsafe { T::initialize(oid.address() as *mut T, object_pool, arguments) };
			self.oid = oid;
			Ok(())
		}
	}
	
	#[inline(always)]
	fn failure_in_transaction(result: c_int) -> Result<(), PmdkError>
	{
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			PmdkError::obj("pmemobj_tx_*")
		}
	}
	
	#[inline(always)]
	fn null() -> Self
	{
		Self::new(unsafe { OID_NULL })
	}
	
	#[inline(always)]
	fn new(oid: PMEMoid) -> Self
	{
		Self
		{
			oid,
			phantom_data: PhantomData,
		}
	}
}

#[macro_export]
macro_rules! add_field_to_transaction
{
	($self: ident, $transaction: ident, $selfType: ty, $field: ident) =>
	{
		{
			$self.add_range_snapshot_in_transaction(offset_of!($selfType, $field), ::std::mem::size_of_val(&$self.$field))
		}
	}
}

#[macro_export]
macro_rules! add_field_to_transaction_without_flush
{
	($self: ident, $transaction: ident, $selfType: ty, $field: ident) =>
	{
		{
			$self.add_range_snapshot_in_transaction_without_flush(offset_of!($selfType, $field), ::std::mem::size_of_val(&$self.$field))
		}
	}
}

#[macro_export]
macro_rules! add_contiguous_fields_to_transaction
{
	($self: ident, $transaction: ident, $selfType: ty, $fromInclusiveField: ident, $toInclusiveField: ident) =>
	{
		{
			$self.add_range_snapshot_in_transaction(offset_of!($selfType, $fromInclusiveField), (::std::mem::size_of_val(&$self.$toInclusiveField) + offset_of!($selfType, $toInclusiveField)) - offset_of!($selfType, $fromInclusiveField))
		}
	}
}

#[macro_export]
macro_rules! add_contiguous_fields_to_transaction_without_flush
{
	($self: ident, $transaction: ident, $selfType: ty, $fromInclusiveField: ident, $toInclusiveField: ident) =>
	{
		{
			$self.add_range_snapshot_in_transaction_without_flush(offset_of!($selfType, $fromInclusiveField), (::std::mem::size_of_val(&$self.$toInclusiveField) + offset_of!($selfType, $toInclusiveField)) - offset_of!($selfType, $fromInclusiveField))
		}
	}
}
