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
	
	#[inline(always)]
	pub fn allocateAndInsertAtHead(&mut self, objectPool: &ObjectPool, arguments: &mut T::Arguments) -> Result<PersistentObject<T>, GenericError>
	{
		self.allocateAndInsert(objectPool, arguments, PersistentObject::null(), POBJ_LIST_DEST_HEAD as i32)
	}
	
	#[inline(always)]
	pub fn allocateAndInsertBefore(&mut self, objectPool: &ObjectPool, arguments: &mut T::Arguments, index: PersistentObject<T>) -> Result<PersistentObject<T>, GenericError>
	{
		self.allocateAndInsert(objectPool, arguments, index, POBJ_LIST_DEST_HEAD as i32)
	}
	
	#[inline(always)]
	pub fn allocateAndInsertAfter(&mut self, objectPool: &ObjectPool, arguments: &mut T::Arguments, index: PersistentObject<T>) -> Result<PersistentObject<T>, GenericError>
	{
		self.allocateAndInsert(objectPool, arguments, index, POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	pub fn allocateAndInsertAtTail(&mut self, objectPool: &ObjectPool, arguments: &mut T::Arguments) -> Result<PersistentObject<T>, GenericError>
	{
		self.allocateAndInsert(objectPool, arguments, PersistentObject::null(), POBJ_LIST_DEST_TAIL as i32)
	}
	
	#[inline(always)]
	fn allocateAndInsert(&mut self, objectPool: &ObjectPool, arguments: &mut T::Arguments, index: PersistentObject<T>, directionTowards: c_int) -> Result<PersistentObject<T>, GenericError>
	{
		let size = T::size();
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		#[thread_local] static mut CapturedPanic: Option<Box<Any + Send + 'static>> = None;
		
		unsafe extern "C" fn constructor<T: Persistable>(pop: *mut PMEMobjpool, ptr: *mut c_void, arg: *mut c_void) -> c_int
		{
			let result = catch_unwind(AssertUnwindSafe(||
			{
				debug_assert!(!pop.is_null(), "pop is null");
				debug_assert!(!ptr.is_null(), "ptr is null");
				debug_assert!(!arg.is_null(), "arg is null");
				
				T::initialize(ptr as *mut T, pop, &mut *(arg as *mut T::Arguments))
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
		
		let result = unsafe { pmemobj_list_insert_new(objectPool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, index.oid, directionTowards, T::size(), T::TypeNumber, Some(constructor::<T>), arguments as *mut _ as * mut _) };
		
		if unlikely(result.is_null())
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
					Err(GenericError::new(osErrorNumber, pmemobj_errormsg, "pmemobj_alloc or pmemobj_root_construct"))
				},
				
				_ =>
				{
					debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set and error was '{}'", osErrorNumber);
					
					Err(GenericError::new(osErrorNumber, pmemobj_errormsg, "pmemobj_list_insert_new"))
				}
			}
		}
		else
		{
			debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set yet result was 0 (Ok)");
			
			Ok(PersistentObject::new(result))
		}
	}
	
	#[inline(always)]
	pub fn remove(&mut self, objectPool: &ObjectPool, index: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.removeInternal(objectPool, index, 0)
	}
	
	#[inline(always)]
	pub fn removeAndFree(&mut self, objectPool: &ObjectPool, index: PersistentObject<T>) -> Result<(), GenericError>
	{
		self.removeInternal(objectPool, index, 1)
	}
	
	#[inline(always)]
	fn removeInternal(&mut self, objectPool: &ObjectPool, index: PersistentObject<T>, free: c_int) -> Result<(), GenericError>
	{
		debug_assert!(!index.is_null(), "index is null");
		
		let result = unsafe { pmemobj_list_remove(objectPool.0, T::PersistentCircularDoublyLinkedListEntryFieldOffset, self as *mut _ as *mut c_void, index.oid, free) };
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
