// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO pool of persistent memory is a kind of `malloc`- or heap- like allocator.
/// Unlike the system `malloc`, multiple instances of it can be created.
#[derive(Debug)]
pub struct CtoPool<T: CtoSafe + Send + Sync>(Arc<CtoPoolInner>, RwLock<CtoRootBox<T>>);

unsafe impl<T: CtoSafe + Send + Sync> Send for CtoPool<T>
{
}

unsafe impl<T: CtoSafe + Send + Sync> Sync for CtoPool<T>
{
}

impl<T: CtoSafe + Send + Sync> PartialEq for CtoPool<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
}

impl<T: CtoSafe + Send + Sync> Eq for CtoPool<T>
{
}

/*
use ::std::heap::Alloc;
use ::std::heap::AllocErr;
use ::std::heap::Layout;

unsafe impl<T: CtoSafe + Send + Sync> Alloc for CtoPool<T>
{
	#[inline(always)]
	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr>
	{
		let pointer = (self.0).0.aligned_allocate_from_layout(&layout).map_err(|_| AllocError::Unsupported
		{
			details: "Not easily supplied"
		})?;
		if pointer.is_null()
		{
			Err(AllocError::Exhausted
			{
				request: layout
			})
		}
		else
		{
			Ok(pointer)
		}
		
		TODO:
		xxx; // adjust all other methods in PMEMctopoolEx to check for null from malloc, etc..
	}
	
	#[inline(always)]
	unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout)
	{
		debug_assert!(!ptr.is_null(), "ptr is null");
		
		(self.0).0.free(ptr)
	}
	
	xxxx - use realloc
	unsafe fn realloc(&mut self,
		ptr: *mut u8,
		layout: Layout,
		new_layout: Layout) -> Result<*mut u8, AllocErr> {
		let new_size = new_layout.size();
		let old_size = layout.size();
		let aligns_match = layout.align == new_layout.align;
		
		if new_size >= old_size && aligns_match {
			if let Ok(()) = self.grow_in_place(ptr, layout.clone(), new_layout.clone()) {
				return Ok(ptr);
			}
		} else if new_size < old_size && aligns_match {
			if let Ok(()) = self.shrink_in_place(ptr, layout.clone(), new_layout.clone()) {
				return Ok(ptr);
			}
		}
		
		// otherwise, fall back on alloc + copy + dealloc.
		let result = self.alloc(new_layout);
		if let Ok(new_ptr) = result {
			ptr::copy_nonoverlapping(ptr as *const u8, new_ptr, cmp::min(old_size, new_size));
			self.dealloc(ptr, layout);
		}
		result
	}
	
	unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr> {
		
		(self.0).0.usable_size(xxxxx)
		
	//	let usable_size = self.usable_size(&layout);
		self.alloc(layout).map(|p| Excess(p, usable_size.1))
	}
	
	unsafe fn realloc_excess(&mut self,
		ptr: *mut u8,
		layout: Layout,
		new_layout: Layout) -> Result<Excess, AllocErr> {
}
*/

impl<T: CtoSafe + Send + Sync> CtoPool<T>
{
	/// Opens a pool, creating it if necessary, and instantiating a root object if one is missing.
	/// This method is unsafe, because nothing stops T being of a different layout (struct type).
	/// Additionally, nothing stops the layout of T changing from compile to compile.
	#[inline(always)]
	pub fn open<InitializationError: error::Error, Initializer: FnOnce(&mut T, &CtoPoolAllocator) -> Result<(), InitializationError>>(pool_set_file_path: &Path, layout_name: &str, pool_size: usize, mode: mode_t, root_initializer: Initializer) -> Result<Self, CtoPoolOpenError<InitializationError>>
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
			let new_cto_root_box = CtoPoolAllocator(&cto_pool_inner).allocate_root_box(root_initializer).map_err(|cto_pool_allocation_error| CtoPoolOpenError::RootCreation(cto_pool_allocation_error))?;
			cto_pool_inner.set_root(CtoRootBox::as_ptr(&new_cto_root_box));
			new_cto_root_box
		}
		else
		{
			let mutable_root_reference = unsafe { &mut * (existing_root as *mut T) };
			mutable_root_reference.reinitialize(&cto_pool_inner);
			CtoRootBox(existing_root)
		};
		
		Ok(CtoPool(cto_pool_inner, RwLock::new(cto_root_box)))
	}
	
	/// Returns a Read-Write lock to access the root of the CTO object graph.
	#[inline(always)]
	pub fn root(&self) -> &RwLock<CtoRootBox<T>>
	{
		&self.1
	}
}
