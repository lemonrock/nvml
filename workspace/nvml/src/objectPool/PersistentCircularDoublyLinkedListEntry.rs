// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// This struct is intended to be inlined, ie used anonymously, within a Persistable
// Also known as 'list_entry'
#[repr(C)]
pub struct PersistentCircularDoublyLinkedListEntry<T: ListEntryPersistable>
{
	pe_next: PersistentObject<T>,
	pe_prev: PersistentObject<T>,
}

impl<T: ListEntryPersistable> Initializable for PersistentCircularDoublyLinkedListEntry<T>
{
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool)
	{
		debug_assert!(!pointerToUninitializedMemoryToUseForFields.is_null(), "pointerToUninitializedMemoryToUseForFields is null");
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let mut instance = &mut *pointerToUninitializedMemoryToUseForFields;
		instance.pe_next = PersistentObject::null();
		instance.pe_prev = PersistentObject::null();
	}
}

impl<T: ListEntryPersistable> PersistentCircularDoublyLinkedListEntry<T>
{
	/// Returns None if there isn't a next entry
	/// If returns Some(x), then x.is_null() is ALWAYS false
	#[inline(always)]
	pub fn next(&self) -> Option<&PersistentObject<T>>
	{
		if unlikely(self.pe_next.is_null())
		{
			None
		}
		else
		{
			Some(&self.pe_next)
		}
	}
	
	/// Returns None if there isn't a next entry
	/// If returns Some(x), then x.is_null() is ALWAYS false
	#[inline(always)]
	pub fn previous(&self) -> Option<&PersistentObject<T>>
	{
		if unlikely(self.pe_prev.is_null())
		{
			None
		}
		else
		{
			Some(&self.pe_prev)
		}
	}
}

#[repr(C)]
pub struct fooListEntry
{
	LIST_ENTRY_FIELD: PersistentCircularDoublyLinkedListEntry<fooListEntry>,
	someData: u32,
}

impl Persistable for fooListEntry
{
	const TypeNumber: TypeNumber = 2;
	
	type Arguments = (u32);
	
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool, arguments: &mut Self::Arguments)
	{
		debug_assert!(!pointerToUninitializedMemoryToUseForFields.is_null(), "pointerToUninitializedMemoryToUseForFields is null");
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let mut instance = &mut *pointerToUninitializedMemoryToUseForFields;
		PersistentCircularDoublyLinkedListEntry::initialize(&mut instance.LIST_ENTRY_FIELD, objectPool);
		
		instance.someData = *arguments;
	}
}

impl ListEntryPersistable for fooListEntry
{
	#[inline(always)]
	fn listEntryField(&self) -> &PersistentCircularDoublyLinkedListEntry<Self>
	{
		&self.LIST_ENTRY_FIELD
	}
}
