// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


trait Allocator
{
	#[inline(always)]
	fn allocate<P: PersistentMemoryWrapper, InitializationError, Initializer: FnOnce(&mut P::Value) -> Result<(), InitializationError>>(self, initializer: Initializer, cto_pool_alloc_guard_reference: &CtoPoolAllocGuardReference) -> Result<P, CtoPoolAllocationError<InitializationError>>;
	
	#[inline(always)]
	fn aligned_allocate<T>(self) -> Result<*mut T, PmdkError>;
	
	#[inline(always)]
	fn alloc_trait_allocate(self, layout: &Layout) -> Result<*mut u8, AllocErr>;
	
	#[inline(always)]
	fn alloc_trait_reallocate(self, old_pointer: *mut u8, old_layout: &Layout, new_layout: &Layout) -> Result<*mut u8, AllocErr>;
	
	#[inline(always)]
	fn alloc_trait_free(self, pointer_to_free: *mut u8);
	
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

impl Allocator for *mut PMEMctopool
{
	#[inline(always)]
	fn allocate<P: PersistentMemoryWrapper, InitializationError, Initializer: FnOnce(&mut P::Value) -> Result<(), InitializationError>>(self, initializer: Initializer, cto_pool_alloc_guard_reference: &CtoPoolAllocGuardReference) -> Result<P, CtoPoolAllocationError<InitializationError>>
	{
		debug_assert!(!self.is_null(), "self is null");
		
		match self.aligned_allocate::<P::PersistentMemory>()
		{
			Err(allocation_error) => return Err(CtoPoolAllocationError::Allocation(allocation_error)),
			
			Ok(persistent_memory_pointer) => match unsafe { P::initialize_persistent_memory(persistent_memory_pointer, cto_pool_alloc_guard_reference, initializer) }
			{
				Ok(outer) => Ok(outer),
				
				Err(initialization_error) =>
				{
					self.free(persistent_memory_pointer);
					
					Err(CtoPoolAllocationError::Initialization(initialization_error))
				}
			},
		}
	}
	
	#[inline(always)]
	fn aligned_allocate<T>(self) -> Result<*mut T, PmdkError>
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let alignment = align_of::<T>();
		let size = size_of::<T>() as size_t;
		self.aligned_alloc(alignment, size).map(|pointer| pointer as *mut T)
	}
	
	#[inline(always)]
	fn alloc_trait_allocate(self, layout: &Layout) -> Result<*mut u8, AllocErr>
	{
		debug_assert!(!self.is_null(), "self is null");
		
		Self::map_allocation_result(self.aligned_alloc(layout.align(), layout.size()), "PMDK libpmemcto.pmemcto_aligned_alloc failed", layout)
	}
	
	#[inline(always)]
	fn alloc_trait_reallocate(self, old_pointer: *mut u8, old_layout: &Layout, new_layout: &Layout) -> Result<*mut u8, AllocErr>
	{
		debug_assert!(!self.is_null(), "self is null");
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
				Self::map_allocation_result(self.realloc(old_pointer as *mut _, new_size), "PMDK libpmemcto.pmemcto_realloc failed", new_layout)
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
	fn alloc_trait_free(self, pointer_to_free: *mut u8)
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!pointer_to_free.is_null(), "jemalloc (the underlying allocator for libpmemobj) does not pass out null");
		
		self.free(pointer_to_free)
	}
}
