// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct PersistentCircularDoublyLinkedList<T: ListElementPersistable>
{
	head: *mut c_void,
	objectPool: *mut PMEMobjpool,
	phantomData: PhantomData<T>
}

impl<T: ListElementPersistable> PersistentCircularDoublyLinkedList<T>
{
	#[inline(always)]
	pub fn insertAtHead(&self, element: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(element, PersistentObject::null(), POBJ_LIST_DEST_HEAD as i32)
	}
	
	#[inline(always)]
	pub fn insertBefore(&self, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(element, index, POBJ_LIST_DEST_HEAD as i32)
	}
	
	#[inline(always)]
	pub fn insertAfter(&self, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(element, index, POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	pub fn insertAtTail(&self, element: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.insert(element, PersistentObject::null(), POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	fn insert(&self, element: PersistentObject<T>, index: PersistentObject<T>, directionTowards: c_int) -> Result<(), GenericError>
	{
		let result = unsafe { pmemobj_list_insert(self.objectPool, T::ListEntryStructOffset, self.head, index.oid, directionTowards, element.oid) };
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

pub trait ListElementPersistable: Persistable
{
	#[inline(always)]
	const ListEntryStructOffset: size_t = 0;
}
