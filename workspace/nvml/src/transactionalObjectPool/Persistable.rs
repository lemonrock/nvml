// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Persistable: Sized
{
	const TypeNumber: TypeNumber;
	
	#[inline(always)]
	fn size() -> size_t
	{
		size_of::<Self>() as size_t
	}
	
	#[inline(always)]
	fn oid(&self) -> PMEMoid
	{
		let pointer = self as *const _ as *const c_void;
		let oid = pointer.oid();
		debug_assert!(!oid.is_null(), "This object is not a Persistable");
		oid
	}
	
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
	
	#[inline(always)]
	fn free(self, transaction: Transaction) -> c_int
	{
		transaction.free(self.oid())
	}
	
	/// size can be zero
	#[inline(always)]
	fn addRangeSnapshotInTransaction(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= Self::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, Self::size());
		
		transaction.addRangeSnapshotInTransaction(self.oid(), offset, size)
	}

	/// Can only be called from a work() function
	/// If returns !=0 then the transaction will have been aborted; return immediately from work() function
	/// No checks are made for offset or size
	/// size can be zero
	#[inline(always)]
	fn addRangeSnapshotInTransactionWithoutFlush(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= Self::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, Self::size());
		
		transaction.addRangeSnapshotInTransactionWithoutFlush(self.oid(), offset, size)
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
	mutex: PMEMmutex,
	next: OidWrapper<node>,
	foo: OidWrapper<foo>,
	data: u32,
}

impl Persistable for node
{
	const TypeNumber: TypeNumber = 1;
}

impl MutexLockablePersistentObjectMemory for node
{
	#[inline(always)]
	fn _pmemMutex(&mut self) -> &mut PMEMmutex
	{
		&mut self.mutex
	}
}

impl node
{
	pub fn manipulate(&mut self)
	{
		let mut lock = self.lock();
		lock.data = 45;
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
