// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO pool of persistent memory is a kind of `malloc`- or heap- like allocator.
/// Unlike the system `malloc`, multiple instances of it can be created, one for each bank of Persistent memory.
/// And, it has a graph 'root'.
/// To access the 'root' of the graph, use `deref()` or `deref_mut()`.
/// Persistence does not happen successfully until this object is closed (dropped).
/// Dropping only occurs when there are not more instances of `CtoPoolAllocGuardReference`.
pub struct CtoPoolAlloc<RootValue: CtoSafe>(*mut PMEMctopool, CtoPoolAllocGuardReference, PhantomData<RootValue>);

impl<RootValue: CtoSafe> PartialEq for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
}

impl<RootValue: CtoSafe> Eq for CtoPoolAlloc<RootValue>
{
}

impl<RootValue: CtoSafe> Debug for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.write_str(&format!("CtoPoolAlloc({:?})", self.0))
	}
}

unsafe impl<RootValue: CtoSafe> Send for CtoPoolAlloc<RootValue>
{
}

unsafe impl<RootValue: CtoSafe> Sync for CtoPoolAlloc<RootValue>
{
}

unsafe impl<RootValue: CtoSafe> Alloc for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr>
	{
		self.0.alloc_trait_allocate(&layout)
	}
	
	#[inline(always)]
	unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout)
	{
		self.0.alloc_trait_free(ptr)
	}
	
	#[inline(always)]
	unsafe fn realloc(&mut self, old_pointer: *mut u8, old_layout: Layout, new_layout: Layout) -> Result<*mut u8, AllocErr>
	{
		self.0.alloc_trait_reallocate(old_pointer, &old_layout, &new_layout)
	}
	
	/// Almost useless as the usable size is a property of an allocated size.
	#[inline(always)]
	fn usable_size(&self, layout: &Layout) -> (usize, usize)
	{
		let size = layout.size();
		if size == 0
		{
			(0, 1)
		}
		else
		{
			(size, size)
		}
	}
	
	#[inline(always)]
	unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr>
	{
		self.0.alloc_trait_allocate(&layout).map(|allocation_pointer| Excess(allocation_pointer, self.0.usable_size(allocation_pointer as *mut c_void)))
	}
	
	#[inline(always)]
	unsafe fn realloc_excess(&mut self, old_pointer: *mut u8, old_layout: Layout, new_layout: Layout) -> Result<Excess, AllocErr>
	{
		self.0.alloc_trait_reallocate(old_pointer, &old_layout, &new_layout).map(|allocation_pointer| Excess(allocation_pointer, self.0.usable_size(allocation_pointer as *mut c_void)))
	}
	
	/// Useless. Use realloc.
	#[inline(always)]
	unsafe fn grow_in_place(&mut self, _ptr: *mut u8, _old_layout: Layout, _new_layout: Layout) -> Result<(), CannotReallocInPlace>
	{
		Err(CannotReallocInPlace)
	}
	
	/// Useless. Use realloc.
	#[inline(always)]
	unsafe fn shrink_in_place(&mut self, _ptr: *mut u8, _old_layout: Layout, _new_layout: Layout) -> Result<(), CannotReallocInPlace>
	{
		Err(CannotReallocInPlace)
	}
	
	#[inline(always)]
	fn alloc_one<T>(&mut self) -> Result<Unique<T>, AllocErr>
	where Self: Sized
	{
		unsafe { self.0.alloc_trait_allocate(&Layout::new::<T>()).map(|allocation_pointer| Unique::new_unchecked(allocation_pointer as *mut T)) }
	}
	
	#[inline(always)]
	unsafe fn dealloc_one<T>(&mut self, ptr: Unique<T>)
	where Self: Sized
	{
		self.0.alloc_trait_free(ptr.as_ptr() as *mut u8);
	}
	
	#[inline(always)]
	fn alloc_array<UniqueT>(&mut self, number_of_items: usize) -> Result<Unique<UniqueT>, AllocErr>
	where Self: Sized
	{
		match Layout::array::<UniqueT>(number_of_items)
		{
			Some(ref layout) => self.0.alloc_trait_allocate(layout).map(|allocation_pointer| unsafe { Unique::new_unchecked(allocation_pointer as *mut UniqueT) }),
			
			_ => Err(AllocErr::invalid_input("invalid layout for alloc_array")),
		}
	}
	
	#[inline(always)]
	unsafe fn realloc_array<T>(&mut self, old_pointer: Unique<T>, old_number_of_items: usize, new_number_of_items: usize) -> Result<Unique<T>, AllocErr>
	where Self: Sized
	{
		match (Layout::array::<T>(old_number_of_items), Layout::array::<T>(new_number_of_items))
		{
			(Some(ref old_layout), Some(ref new_layout)) => self.0.alloc_trait_reallocate(old_pointer.as_ptr() as *mut _, old_layout, new_layout).map(|allocation_pointer|Unique::new_unchecked(allocation_pointer as *mut T)),
			
			_ => Err(AllocErr::invalid_input("invalid layout for realloc_array")),
		}
	}
	
	#[inline(always)]
	unsafe fn dealloc_array<T>(&mut self, pointer_to_free: Unique<T>, number_of_items: usize) -> Result<(), AllocErr>
	where Self: Sized
	{
		match Layout::array::<T>(number_of_items)
		{
			Some(_) => Ok(self.0.alloc_trait_free(pointer_to_free.as_ptr() as *mut _)),
			
			_ => Err(AllocErr::invalid_input("invalid layout for dealloc_array")),
		}
	}
}

impl<RootValue: CtoSafe> Deref for CtoPoolAlloc<RootValue>
{
	type Target = RootValue;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		let existing_root = self.0.get_root();
		if unlikely(existing_root.is_null())
		{
			panic!("No root object");
		}
		else
		{
			unsafe { & * (existing_root as *const RootValue) }
		}
	}
}

impl<RootValue: CtoSafe> DerefMut for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		let existing_root = self.0.get_root();
		if unlikely(existing_root.is_null())
		{
			panic!("No root object");
		}
		else
		{
			unsafe { &mut * (existing_root as *mut RootValue) }
		}
	}
}

impl<RootValue: CtoSafe> Borrow<RootValue> for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn borrow(&self) -> &RootValue
	{
		self.deref()
	}
}

impl<RootValue: CtoSafe> BorrowMut<RootValue> for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut RootValue
	{
		self.deref_mut()
	}
}

impl<RootValue: CtoSafe> AsRef<RootValue> for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn as_ref(&self) -> &RootValue
	{
		self.deref()
	}
}

impl<RootValue: CtoSafe> AsMut<RootValue> for CtoPoolAlloc<RootValue>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut RootValue
	{
		self.deref_mut()
	}
}

impl<RootValue: CtoSafe> CtoPoolAlloc<RootValue>
{
	/// Opens a pool, creating it if necessary, and re-initializing any memory that is volatile (eg condition variables, mutex locks, etc).
	/// If the pool does not contain a root, then it is initialized using `root_value_initializer`.
	#[inline(always)]
	pub fn open<InitializationError: error::Error, RootValueInitializer: FnOnce(&mut RootValue, &CtoPoolAllocGuardReference) -> Result<(), InitializationError>>(pool_set_file_path: &Path, layout_name: &str, pool_size: usize, mode: mode_t, root_value_initializer: RootValueInitializer) -> Result<Self, CtoPoolOpenError<InitializationError>>
	{
		let layout_name = CString::new(layout_name).expect("Embedded NULs are not allowed in a layout name");
		let length = layout_name.as_bytes().len();
		assert!(length <= PMEMCTO_MAX_LAYOUT, "layout_name length exceeds PMEMCTO_MAX_LAYOUT, {}", PMEMCTO_MAX_LAYOUT);
		
		let layout_name = layout_name.as_c_str();
		
		let pool_pointer = match pool_set_file_path.create_cto_pool(layout_name, pool_size, mode)
		{
			Err(generic_error) => return Err(CtoPoolOpenError::CreateFailed(generic_error)),
			Ok(pool_pointer) => if pool_pointer.is_null()
			{
				match pool_set_file_path.validate_cto_pool_is_consistent(layout_name)
				{
					Err(generic_error) => return Err(CtoPoolOpenError::ValidationFailed(generic_error)),
					Ok(is_valid) => if is_valid
					{
						()
					}
					else
					{
						return Err(CtoPoolOpenError::Invalid)
					},
				};
				
				match pool_set_file_path.open_cto_pool(layout_name)
				{
					Err(generic_error) => return Err(CtoPoolOpenError::OpenFailed(generic_error)),
					Ok(pool_pointer) => pool_pointer,
				}
			}
			else
			{
				pool_pointer
			},
		};
		
		let cto_pool_alloc_guard_reference = CtoPoolAllocGuardReference::new(pool_pointer);
		
		let cto_pool_alloc: CtoPoolAlloc<RootValue> = CtoPoolAlloc(pool_pointer, cto_pool_alloc_guard_reference, PhantomData);
		
		let existing_root = pool_pointer.get_root();
		if unlikely(existing_root.is_null())
		{
			let new_root = cto_pool_alloc.0.aligned_allocate::<RootValue>().map_err(|pmdk_error| CtoPoolOpenError::RootCreation(CtoPoolAllocationError::Allocation(pmdk_error)))?;
			let root = unsafe { &mut * (new_root as *mut RootValue) };
			root_value_initializer(root, cto_pool_alloc.allocator()).map_err(|initialization_error| CtoPoolOpenError::RootCreation(CtoPoolAllocationError::Initialization(initialization_error)))?;
			pool_pointer.set_root(new_root);
		}
		else
		{
			let root = unsafe { &mut * (existing_root as *mut RootValue) };
			root.cto_pool_opened(cto_pool_alloc.allocator());
		}
		
		Ok(cto_pool_alloc)
	}
	
	/// Returns an object that can be used for allocations.
	#[inline(always)]
	pub fn allocator(&self) -> &CtoPoolAllocGuardReference
	{
		&self.1
	}
}
