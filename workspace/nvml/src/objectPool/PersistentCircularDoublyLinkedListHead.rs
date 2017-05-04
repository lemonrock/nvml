// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// This struct could be either stand-alone or inlined
// Also known as 'list_head'
// Also known as 'head: *mut c_void'
#[repr(C)]
pub struct PersistentCircularDoublyLinkedListHead<T: ListEntryPersistable>
{
	pe_first: PersistentObject<T>,
	lock: PMEMmutex,
}

impl<T: ListEntryPersistable> Initializable for PersistentCircularDoublyLinkedListHead<T>
{
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool)
	{
		debug_assert!(!pointerToUninitializedMemoryToUseForFields.is_null(), "pointerToUninitializedMemoryToUseForFields is null");
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let mut instance = &mut *pointerToUninitializedMemoryToUseForFields;
		instance.pe_first = PersistentObject::null();
		(&mut instance.lock as *mut _).zero(objectPool);
	}
}

impl<T: ListEntryPersistable> PersistentCircularDoublyLinkedListHead<T>
{
	#[inline(always)]
	pub fn insertAtHead(&mut self, objectPool: &ObjectPool, element: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(objectPool, element, PersistentObject::null(), POBJ_LIST_DEST_HEAD as i32)
	}
	
	#[inline(always)]
	pub fn insertBefore(&mut self, objectPool: &ObjectPool, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(objectPool, element, index, POBJ_LIST_DEST_HEAD as i32)
	}
	
	#[inline(always)]
	pub fn insertAfter(&mut self, objectPool: &ObjectPool, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(objectPool, element, index, POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	pub fn insertAtTail(&mut self, objectPool: &ObjectPool, element: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(objectPool, element, PersistentObject::null(), POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	fn insert(&mut self, objectPool: &ObjectPool, element: PersistentObject<T>, index: PersistentObject<T>, directionTowards: c_int) -> Result<(), GenericError>
	{
		debug_assert!(!element.is_null(), "element is null");
		
		let result = unsafe { pmemobj_list_insert(objectPool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, index.oid, directionTowards, element.oid) };
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			Err(GenericError::new(errno().0, pmemobj_errormsg, "pmemobj_list_insert"))
		}
	}
}

impl<T: ListEntryPersistable> PersistentCircularDoublyLinkedListHead<T>
{
	#[inline(always)]
	pub fn isEmpty(&self) -> bool
	{
		self.pe_first.oid.off == 0
	}
	
	/// Returns None if the list is empty
	/// If returns Some(x), then x.is_null() is ALWAYS false
	#[inline(always)]
	pub fn head(&self) -> Option<&PersistentObject<T>>
	{
		if unlikely(self.isEmpty())
		{
			None
		}
		else
		{
			Some(&self.pe_first)
		}
	}
	
	/// Returns None if the list is empty
	/// If returns Some(x), then x.is_null() is ALWAYS false
	#[inline(always)]
	pub fn tail(&self) -> Option<&PersistentObject<T>>
	{
		let pe_first = self.pe_first.oid;
		
		if unlikely(self.isEmpty())
		{
			None
		}
		else
		{
			let pointerToListEntryFieldHolder = pe_first.address() as *const T;
			unsafe { &*pointerToListEntryFieldHolder }.listEntryField().previous()
		}
	}
}
