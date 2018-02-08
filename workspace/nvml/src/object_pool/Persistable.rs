// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A Persistable is the essential trait that a struct must implement in order to be persistent.
/// Persistable MUST NOT implement Drop, Copy or Clone
pub trait Persistable: Sized
{
	/// Each implementation must have an unique value of this, ideally starting at one (one-based)
	const TypeNumber: TypeNumber;
	
	/// A tuple of the arguments passed to initialize()
	type Arguments;
	
	/// # Arguments
	/// - pointer_to_uninitialized_memory_to_use_for_fields is always non-null
	/// - object_pool is always non-null
	#[inline(always)]
	unsafe fn initialize(pointer_to_uninitialized_memory_to_use_for_fields: *mut Self, object_pool: *mut PMEMobjpool, arguments: &mut Self::Arguments);
	
	/// Size in bytes that pointer_to_uninitialized_memory_to_use_for_fields in initialize() points to.
	/// ie the size of this 'struct'.
	#[inline(always)]
	fn size() -> size_t
	{
		let size = size_of::<Self>() as size_t;
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		size
	}
	
	/// Find this instance's OID. An OID is the unique object identifier, ie an instance identifier, used in the persistent data store.
	#[deprecated(note = "inefficient; access via PersistentObject")]
	#[inline(always)]
	fn oid(&self) -> PMEMoid
	{
		let pointer = self as *const _ as *const c_void;
		let oid = unsafe { pmemobj_oid(pointer) };
		debug_assert!(oid.is_not_null(), "This object is not a Persistable");
		oid
	}
}

/// An example of a Persistable that is the root of a graph of persistable objects
#[repr(C)]
pub struct root
{
	node: PersistentObject<node>,
}

impl Persistable for root
{
	const TypeNumber: TypeNumber = 0;
	
	type Arguments = ();
	
	#[allow(unused_variables)]
	#[inline(always)]
	unsafe fn initialize(pointer_to_uninitialized_memory_to_use_for_fields: *mut Self, object_pool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(pointer_to_uninitialized_memory_to_use_for_fields.is_not_null(), "pointer_to_uninitialized_memory_to_use_for_fields is null");
		debug_assert!(object_pool.is_not_null(), "object_pool is null");
		
		let instance = &mut *pointer_to_uninitialized_memory_to_use_for_fields;
		instance.node.allocate_uninitialized_and_construct_object(object_pool, &mut ()).expect("Allocation failed for node");
	}
}

/// An example of a Persistable that is quite complex, with different synchronisation properties and children.
#[repr(C)]
pub struct node
{
	read_write_lock: PMEMrwlock,
	mutex_lock: PMEMmutex,
	condition_variable: PMEMcond,
	next: PersistentObject<node>,
	foo: PersistentObject<foo>,
	data: u32,
}

impl Persistable for node
{
	const TypeNumber: TypeNumber = 1;
	
	type Arguments = ();
	
	#[allow(unused_variables)]
	#[inline(always)]
	unsafe fn initialize(pointer_to_uninitialized_memory_to_use_for_fields: *mut Self, object_pool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(pointer_to_uninitialized_memory_to_use_for_fields.is_not_null(), "pointer_to_uninitialized_memory_to_use_for_fields is null");
		debug_assert!(object_pool.is_not_null(), "object_pool is null");
		
		let instance = &mut *pointer_to_uninitialized_memory_to_use_for_fields;
		
		(&mut instance.read_write_lock as *mut _).zero(object_pool);
		(&mut instance.mutex_lock as *mut _).zero(object_pool);
		(&mut instance.condition_variable as *mut _).zero(object_pool);
		
		instance.next.allocate_uninitialized_and_construct_object(object_pool, &mut ()).expect("Allocation failed for next");
		instance.foo.allocate_uninitialized_and_construct_object(object_pool, &mut (11)).expect("Allocation failed for foo");
		instance.data = 0;
	}
}

impl ReadWriteLockablePersistable for node
{
	#[inline(always)]
	fn read_write_lock(&mut self) -> &mut PMEMrwlock
	{
		&mut self.read_write_lock
	}
}

impl MutexLockablePersistable for node
{
	#[inline(always)]
	fn mutex_lock(&mut self) -> &mut PMEMmutex
	{
		&mut self.mutex_lock
	}
}

impl ConditionVariableMutexLockablePersistable for node
{
	#[inline(always)]
	fn condition_variable(&mut self) -> &mut PMEMcond
	{
		&mut self.condition_variable
	}
}

impl node
{
	/// Example method
	pub fn manipulate2(this: &mut PersistentObject<Self>)
	{
		{
			let mut lock = this.mutex();
			lock.data = 45;
		}
		{
			let mut lock = this.write();
			lock.data = 34;
		}
	}
}

/// An example of a Persistable
#[repr(C)]
pub struct foo
{
	address: u8,
}

impl Persistable for foo
{
	const TypeNumber: TypeNumber = 2;
	
	type Arguments = (u8);
	
	#[inline(always)]
	unsafe fn initialize(pointer_to_uninitialized_memory_to_use_for_fields: *mut Self, object_pool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(pointer_to_uninitialized_memory_to_use_for_fields.is_not_null(), "pointer_to_uninitialized_memory_to_use_for_fields is null");
		debug_assert!(object_pool.is_not_null(), "object_pool is null");
		
		let instance = &mut *pointer_to_uninitialized_memory_to_use_for_fields;
		instance.address = *arguments;
	}
}
