// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This struct could be either stand-alone or inlined.
/// Also known as 'list_head'.
/// Also known as 'head: *mut c_void'.
#[repr(C)]
pub struct PersistentCircularDoublyLinkedListHead<T: ListEntryPersistable>
{
	pe_first: PersistentObject<T>,
	lock: PMEMmutex,
}

impl<T: ListEntryPersistable> Initializable for PersistentCircularDoublyLinkedListHead<T>
{
	#[inline(always)]
	unsafe fn initialize(pointer_to_uninitialized_memory_to_use_for_fields: *mut Self, object_pool: *mut PMEMobjpool)
	{
		debug_assert!(!pointer_to_uninitialized_memory_to_use_for_fields.is_null(), "pointer_to_uninitialized_memory_to_use_for_fields is null");
		debug_assert!(!object_pool.is_null(), "object_pool is null");
		
		let instance = &mut *pointer_to_uninitialized_memory_to_use_for_fields;
		instance.pe_first = PersistentObject::null();
		(&mut instance.lock as *mut _).zero(object_pool);
	}
}

impl<T: ListEntryPersistable> PersistentCircularDoublyLinkedListHead<T>
{
	/// Insert at head.
	#[inline(always)]
	pub fn insert_at_head(&mut self, object_pool: &ObjectPool, element: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.insert(object_pool, element, PersistentObject::null(), POBJ_LIST_DEST_HEAD as i32)
	}
	
	/// Insert before index.
	#[inline(always)]
	pub fn insert_before_index(&mut self, object_pool: &ObjectPool, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.insert(object_pool, element, index, POBJ_LIST_DEST_HEAD as i32)
	}
	
	/// Insert after index.
	#[inline(always)]
	pub fn insert_after_index(&mut self, object_pool: &ObjectPool, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.insert(object_pool, element, index, POBJ_LIST_DEST_TAIL as i32)
	}
	
	/// Insert at tail.
	#[inline(always)]
	pub fn insert_at_tail(&mut self, object_pool: &ObjectPool, element: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.insert(object_pool, element, PersistentObject::null(), POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	fn insert(&mut self, object_pool: &ObjectPool, element: PersistentObject<T>, index: PersistentObject<T>, direction_towards: c_int) -> Result<(), PmdkError>
	{
		debug_assert!(!element.is_null(), "element is null");
		
		let result = unsafe { pmemobj_list_insert(object_pool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, index.oid, direction_towards, element.oid) };
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			PmdkError::obj("pmemobj_list_insert")
		}
	}
	
	/// Allocate then insert at head.
	#[inline(always)]
	pub fn allocate_and_insert_at_head(&mut self, object_pool: &ObjectPool, arguments: &mut T::Arguments) -> Result<PersistentObject<T>, PmdkError>
	{
		self.allocate_and_insert(object_pool, arguments, PersistentObject::null(), POBJ_LIST_DEST_HEAD as i32)
	}
	
	/// Allocate then insert before index.
	#[inline(always)]
	pub fn allocate_and_insert_before_index(&mut self, object_pool: &ObjectPool, arguments: &mut T::Arguments, index: PersistentObject<T>) -> Result<PersistentObject<T>, PmdkError>
	{
		self.allocate_and_insert(object_pool, arguments, index, POBJ_LIST_DEST_HEAD as i32)
	}
	
	/// Allocate then insert after index.
	#[inline(always)]
	pub fn allocate_and_insert_after_index(&mut self, object_pool: &ObjectPool, arguments: &mut T::Arguments, index: PersistentObject<T>) -> Result<PersistentObject<T>, PmdkError>
	{
		self.allocate_and_insert(object_pool, arguments, index, POBJ_LIST_DEST_TAIL as i32)
	}
	
	/// Allocate then insert at tail.
	#[inline(always)]
	pub fn allocate_and_insert_at_tail(&mut self, object_pool: &ObjectPool, arguments: &mut T::Arguments) -> Result<PersistentObject<T>, PmdkError>
	{
		self.allocate_and_insert(object_pool, arguments, PersistentObject::null(), POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	fn allocate_and_insert(&mut self, object_pool: &ObjectPool, arguments: &mut T::Arguments, index: PersistentObject<T>, direction_towards: c_int) -> Result<PersistentObject<T>, PmdkError>
	{
		let size = T::size();
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
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
		
		let result = unsafe { pmemobj_list_insert_new(object_pool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, index.oid, direction_towards, T::size(), T::TypeNumber, Some(constructor::<T>), arguments as *mut _ as * mut _) };
		
		if unlikely(result.is_null())
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
					
					PmdkError::obj("pmemobj_list_insert_new")
				}
			}
		}
		else
		{
			debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set yet result was 0 (Ok)");
			
			Ok(PersistentObject::new(result))
		}
	}
	
	/// Remove at index.
	#[inline(always)]
	pub fn remove(&mut self, object_pool: &ObjectPool, index: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.remove_internal(object_pool, index, 0)
	}
	
	/// Remove at index then free.
	#[inline(always)]
	pub fn remove_and_free(&mut self, object_pool: &ObjectPool, index: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.remove_internal(object_pool, index, 1)
	}
	
	#[inline(always)]
	fn remove_internal(&mut self, object_pool: &ObjectPool, index: PersistentObject<T>, free: c_int) -> Result<(), PmdkError>
	{
		debug_assert!(!index.is_null(), "index is null");
		
		let result = unsafe { pmemobj_list_remove(object_pool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, index.oid, free) };
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			PmdkError::obj("pmemobj_list_insert")
		}
	}
	
	/// Remove from this list then inset in a new list at head.
	#[inline(always)]
	pub fn remove_from_this_list_and_insert_new_list_at_head(&mut self, object_pool: &ObjectPool, to: &mut Self, element: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.remove_from_this_list_and_insert_new_list(object_pool, to, element, PersistentObject::null(), POBJ_LIST_DEST_HEAD as i32)
	}
	
	/// Remove from this list then inset in a new list before index.
	#[inline(always)]
	pub fn remove_from_this_list_and_insert_new_list_before_index(&mut self, object_pool: &ObjectPool, to: &mut Self, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.remove_from_this_list_and_insert_new_list(object_pool, to, element, index, POBJ_LIST_DEST_HEAD as i32)
	}
	
	/// Remove from this list then inset in a new list after index.
	#[inline(always)]
	pub fn remove_from_this_list_and_insert_new_list_after_index(&mut self, object_pool: &ObjectPool, to: &mut Self, element: PersistentObject<T>, index: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.remove_from_this_list_and_insert_new_list(object_pool, to, element, index, POBJ_LIST_DEST_TAIL as i32)
	}
	
	/// Remove from this list then inset in a new list at tail.
	#[inline(always)]
	pub fn remove_from_this_list_and_insert_new_list_at_tail(&mut self, object_pool: &ObjectPool, to: &mut Self, element: PersistentObject<T>) -> Result<(), PmdkError>
	{
		self.remove_from_this_list_and_insert_new_list(object_pool, to, element, PersistentObject::null(), POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	fn remove_from_this_list_and_insert_new_list(&mut self, object_pool: &ObjectPool, to: &mut Self, element: PersistentObject<T>, index: PersistentObject<T>, direction_toward: c_int) -> Result<(), PmdkError>
	{
		debug_assert!(!element.is_null(), "element is null");
		
		let result = unsafe { pmemobj_list_move(object_pool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, T::PersistentCircularDoublyLinkedListEntryFieldOffset, to as *mut _ as *mut c_void, index.oid, direction_toward, element.oid) };
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		if likely(result == 0)
		{
			Ok(())
		}
		else
		{
			PmdkError::obj("pmemobj_list_move")
		}
	}
}

impl<T: ListEntryPersistable> PersistentCircularDoublyLinkedListHead<T>
{
	/// Is this list empty?
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.pe_first.oid.off == 0
	}
	
	/// Returns None if the list is empty.
	/// If returns Some(x), then x.is_null() is ALWAYS false.
	#[inline(always)]
	pub fn head(&self) -> Option<&PersistentObject<T>>
	{
		if unlikely(self.is_empty())
		{
			None
		}
		else
		{
			Some(&self.pe_first)
		}
	}
	
	/// Returns None if the list is empty.
	/// If returns Some(x), then x.is_null() is ALWAYS false.
	#[inline(always)]
	pub fn tail(&self) -> Option<&PersistentObject<T>>
	{
		let pe_first = self.pe_first.oid;
		
		if unlikely(self.is_empty())
		{
			None
		}
		else
		{
			let pointer_to_list_entry_field_holder = pe_first.address() as *const T;
			unsafe { &*pointer_to_list_entry_field_holder }.list_entry_field().previous()
		}
	}
}
