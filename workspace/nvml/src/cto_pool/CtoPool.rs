// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO pool of persistent memory is a kind of `malloc`- or heap- like allocator.
/// Unlike the system `malloc`, multiple instances of it can be created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtoPool(Arc<CtoPoolInner>);

unsafe impl Send for CtoPool
{
}

unsafe impl Sync for CtoPool
{
}

unsafe impl Alloc for CtoPool
{
	#[inline(always)]
	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr>
	{
		self.alloc_trait_allocate(&layout)
	}
	
	#[inline(always)]
	unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout)
	{
		self.alloc_trait_free(ptr)
	}
	
	#[inline(always)]
	unsafe fn realloc(&mut self, old_pointer: *mut u8, old_layout: Layout, new_layout: Layout) -> Result<*mut u8, AllocErr>
	{
		self.alloc_trait_reallocate(old_pointer, &old_layout, &new_layout)
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
		self.alloc_trait_allocate(&layout).map(|allocation_pointer| Excess(allocation_pointer, self.as_ptr().usable_size(allocation_pointer as *mut c_void)))
	}
	
	#[inline(always)]
	unsafe fn realloc_excess(&mut self, old_pointer: *mut u8, old_layout: Layout, new_layout: Layout) -> Result<Excess, AllocErr>
	{
		self.alloc_trait_reallocate(old_pointer, &old_layout, &new_layout).map(|allocation_pointer| Excess(allocation_pointer, self.as_ptr().usable_size(allocation_pointer as *mut c_void)))
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
		unsafe { self.alloc_trait_allocate(&Layout::new::<T>()).map(|allocation_pointer| Unique::new_unchecked(allocation_pointer as *mut T)) }
	}
	
	#[inline(always)]
	unsafe fn dealloc_one<T>(&mut self, ptr: Unique<T>)
	where Self: Sized
	{
		self.alloc_trait_free(ptr.as_ptr() as *mut u8);
	}
	
	#[inline(always)]
	fn alloc_array<UniqueT>(&mut self, number_of_items: usize) -> Result<Unique<UniqueT>, AllocErr>
	where Self: Sized
	{
		match Layout::array::<UniqueT>(number_of_items)
		{
			Some(ref layout) => self.alloc_trait_allocate(layout).map(|allocation_pointer| unsafe { Unique::new_unchecked(allocation_pointer as *mut UniqueT) }),
			
			_ => Err(AllocErr::invalid_input("invalid layout for alloc_array")),
		}
	}
	
	#[inline(always)]
	unsafe fn realloc_array<T>(&mut self, old_pointer: Unique<T>, old_number_of_items: usize, new_number_of_items: usize) -> Result<Unique<T>, AllocErr>
	where Self: Sized
	{
		match (Layout::array::<T>(old_number_of_items), Layout::array::<T>(new_number_of_items))
		{
			(Some(ref old_layout), Some(ref new_layout)) => self.alloc_trait_reallocate(old_pointer.as_ptr() as *mut _, old_layout, new_layout).map(|allocation_pointer|Unique::new_unchecked(allocation_pointer as *mut T)),
			
			_ => Err(AllocErr::invalid_input("invalid layout for realloc_array")),
		}
	}
	
	#[inline(always)]
	unsafe fn dealloc_array<T>(&mut self, pointer_to_free: Unique<T>, number_of_items: usize) -> Result<(), AllocErr>
	where Self: Sized
	{
		match Layout::array::<T>(number_of_items)
		{
			Some(_) => Ok(self.alloc_trait_free(pointer_to_free.as_ptr() as *mut _)),
			
			_ => Err(AllocErr::invalid_input("invalid layout for dealloc_array")),
		}
	}
}

impl CtoPool
{
	/// Opens a pool, creating it if necessary, and instantiating a root object if one is missing.
	/// This method is unsafe, because nothing stops T being of a different layout (struct type).
	/// Additionally, nothing stops the layout of T changing from compile to compile.
	/// Returns a CtoPool, which can be used as an `Alloc` instead of `Heap`, and a Read-Write lock to access the root of the CTO object graph.
	#[inline(always)]
	pub fn open<Value: CtoSafe + Sync, InitializationError: error::Error, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(pool_set_file_path: &Path, layout_name: &str, pool_size: usize, mode: mode_t, root_initializer: Initializer) -> Result<(Self, RwLock<CtoRootBox<Value>>), CtoPoolOpenError<InitializationError>>
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
		
		let cto_pool_inner = Arc::new(CtoPoolInner(pool_pointer));
		
		let existing_root = cto_pool_inner.get_root();
		let cto_root_box = if unlikely(existing_root.is_null())
		{
			let mut new_cto_root_box = CtoPoolAllocator(&cto_pool_inner).allocate_root_box(root_initializer).map_err(|cto_pool_allocation_error| CtoPoolOpenError::RootCreation(cto_pool_allocation_error))?;
			cto_pool_inner.set_root(CtoRootBox::as_ptr(&mut new_cto_root_box));
			new_cto_root_box
		}
		else
		{
			let mutable_root_reference = unsafe { &mut * (existing_root as *mut Value) };
			mutable_root_reference.reinitialize(&cto_pool_inner);
			CtoRootBox
			{
				persistent_memory_pointer: existing_root,
			}
		};
		
		Ok((CtoPool(cto_pool_inner), RwLock::new(cto_root_box)))
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut PMEMctopool
	{
		(self.0).0
	}
	
	#[inline(always)]
	fn alloc_trait_allocate(&self, layout: &Layout) -> Result<*mut u8, AllocErr>
	{
		Self::map_allocation_result(self.as_ptr().aligned_alloc(layout.align(), layout.size()), "PMDK libpmemcto.pmemcto_aligned_alloc failed", layout)
	}
	
	#[inline(always)]
	fn alloc_trait_reallocate(&self, old_pointer: *mut u8, old_layout: &Layout, new_layout: &Layout) -> Result<*mut u8, AllocErr>
	{
		debug_assert!(!old_pointer.is_null(), "jemalloc (the underlying allocator for libpmemobj) does not pass out null for size == 0");
		
		let old_size = old_layout.size();
		let new_size = new_layout.size();
		let alignment_is_unchanged = old_layout.align() == new_layout.align();
		
		if alignment_is_unchanged
		{
			if unlikely(old_size == new_size)
			{
				Ok(old_pointer)
			}
			else
			{
				Self::map_allocation_result(self.as_ptr().realloc(old_pointer as *mut _, new_size), "PMDK libpmemcto.pmemcto_realloc failed", new_layout)
			}
		}
		else
		{
			let new_pointer = self.alloc_trait_allocate(new_layout)?;
			unsafe { copy_nonoverlapping(old_pointer as *const _, new_pointer, min(old_size, new_size)) };
			self.alloc_trait_free(old_pointer);
			Ok(new_pointer)
		}
	}
	
	#[inline(always)]
	fn alloc_trait_free(&self, pointer_to_free: *mut u8)
	{
		debug_assert!(!pointer_to_free.is_null(), "jemalloc (the underlying allocator for libpmemobj) does not pass out null");
		
		self.as_ptr().free(pointer_to_free)
	}
	
	#[inline(always)]
	fn map_allocation_result(allocation_result: Result<*mut c_void, PmdkError>, error_message: &'static str, request: &Layout) -> Result<*mut u8, AllocErr>
	{
		match allocation_result
		{
			Err(pmdk_error) => Err
			(
				if pmdk_error.is_ENOMEM()
				{
					AllocErr::Exhausted { request: request.clone() }
				}
				else
				{
					AllocErr::invalid_input(error_message)
				}
			),
			
			Ok(allocation_pointer) => Ok(allocation_pointer as *mut u8)
		}
	}
}
