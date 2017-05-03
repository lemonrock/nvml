// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Persistable: Sized
{
	const TypeNumber: TypeNumber;
	
	#[inline(always)]
	fn size() -> size_t
	{
		let size = size_of::<Self>() as size_t;
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		size
	}
	
	#[deprecated(note = "inefficient; access via OidWrapper")]
	#[inline(always)]
	fn oid(&self) -> PMEMoid
	{
		let pointer = self as *const _ as *const c_void;
		let oid = pointer.oid();
		debug_assert!(!oid.is_null(), "This object is not a Persistable");
		oid
	}
	
	#[deprecated(note = "inefficient; access via OidWrapper")]
	#[inline(always)]
	fn persistentObjectPool(&self) -> *mut PMEMobjpool
	{
		let persistentObjectPool = self.oid().persistentObjectPool();
		debug_assert!(!persistentObjectPool.is_null(), "This object does not have a valid OID");
		persistentObjectPool
	}
	
	/// It is important to now zero-initialise all PMEMmutex, etc types; all OIDs are invalid
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateUninitializedInTransaction(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateUninitializedInTransaction::<Self>(Self::size(), Self::TypeNumber)
	}
	
	/// It is important to now zero-initialise all PMEMmutex, etc types; all OIDs are invalid
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateUninitializedInTransactionWithoutFlush(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateUninitializedInTransactionWithoutFlush::<Self>(Self::size(), Self::TypeNumber)
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateZeroedInTransaction(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateZeroedInTransaction::<Self>(Self::size(), Self::TypeNumber)
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateZeroedInTransactionWithoutFlush(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateZeroedInTransactionWithoutFlush::<Self>(Self::size(), Self::TypeNumber)
	}
}

#[repr(C)]
pub struct root
{
	node: OidWrapper<node>,
}

impl Persistable for root
{
	const TypeNumber: TypeNumber = 0;
}

#[repr(C)]
pub struct node
{
	readWriteLock: PMEMrwlock,
	mutex: PMEMmutex,
	conditionVariable: PMEMcond,
	next: OidWrapper<node>,
	foo: OidWrapper<foo>,
	data: u32,
}

impl Persistable for node
{
	const TypeNumber: TypeNumber = 1;
}

impl ReadWriteLockablePersistentObjectMemory for node
{
	#[inline(always)]
	fn _pmemReadWriteLock(&mut self) -> &mut PMEMrwlock
	{
		&mut self.readWriteLock
	}
}

impl MutexLockablePersistentObjectMemory for node
{
	#[inline(always)]
	fn _pmemMutex(&mut self) -> &mut PMEMmutex
	{
		&mut self.mutex
	}
}

impl ConditionVariableMutexLockablePersistentObjectMemory for node
{
	#[inline(always)]
	fn _pmemConditionVariable(&mut self) -> &mut PMEMcond
	{
		&mut self.conditionVariable
	}
}

impl node
{
	pub fn manipulate(&mut self)
	{
		{
			let mut lock = self.lock();
			lock.data = 45;
		}
		{
			let mut lock = self.writeLock();
			lock.data = 34;
		}
	}
}

#[repr(C)]
pub struct foo
{
	address: u8,
}

impl Persistable for foo
{
	const TypeNumber: TypeNumber = 2;
}