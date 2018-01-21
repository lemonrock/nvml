// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Rust `Alloc` allocator to be used with `RawVec` and other collection objects.
#[derive(Clone)]
pub struct CtoPoolAlloc(CtoPoolArc);

impl CtoSafe for CtoPoolAlloc
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.write(&mut self.0);
	}
}

impl PartialEq for CtoPoolAlloc
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.pool_pointer() == other.pool_pointer()
	}
}

impl Eq for CtoPoolAlloc
{
}

impl Debug for CtoPoolAlloc
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.write_str(&format!("CtoPoolAlloc({:?})", self.pool_pointer()))
	}
}

unsafe impl Send for CtoPoolAlloc
{
}

unsafe impl Sync for CtoPoolAlloc
{
}

unsafe impl Alloc for CtoPoolAlloc
{
	#[inline(always)]
	unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr>
	{
		self.pool_pointer().alloc_trait_allocate(&layout)
	}
	
	#[inline(always)]
	unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout)
	{
		self.pool_pointer().alloc_trait_free(ptr)
	}
	
	#[inline(always)]
	unsafe fn realloc(&mut self, old_pointer: *mut u8, old_layout: Layout, new_layout: Layout) -> Result<*mut u8, AllocErr>
	{
		self.pool_pointer().alloc_trait_reallocate(old_pointer, &old_layout, &new_layout)
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
		self.pool_pointer().alloc_trait_allocate(&layout).map(|allocation_pointer| Excess(allocation_pointer, self.pool_pointer().usable_size(allocation_pointer as *mut c_void)))
	}
	
	#[inline(always)]
	unsafe fn realloc_excess(&mut self, old_pointer: *mut u8, old_layout: Layout, new_layout: Layout) -> Result<Excess, AllocErr>
	{
		self.pool_pointer().alloc_trait_reallocate(old_pointer, &old_layout, &new_layout).map(|allocation_pointer| Excess(allocation_pointer, self.pool_pointer().usable_size(allocation_pointer as *mut c_void)))
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
	fn alloc_one<T>(&mut self) -> Result<NonNull<T>, AllocErr>
		where Self: Sized
	{
		unsafe { self.pool_pointer().alloc_trait_allocate(&Layout::new::<T>()).map(|allocation_pointer| NonNull::new_unchecked(allocation_pointer as *mut T)) }
	}
	
	#[inline(always)]
	unsafe fn dealloc_one<T>(&mut self, ptr: NonNull<T>)
		where Self: Sized
	{
		self.pool_pointer().alloc_trait_free(ptr.as_ptr() as *mut u8);
	}
	
	#[inline(always)]
	fn alloc_array<T>(&mut self, number_of_items: usize) -> Result<NonNull<T>, AllocErr>
		where Self: Sized
	{
		match Layout::array::<T>(number_of_items)
		{
			Some(ref layout) => self.pool_pointer().alloc_trait_allocate(layout).map(|allocation_pointer| unsafe { NonNull::new_unchecked(allocation_pointer as *mut T) }),
			
			_ => Err(AllocErr::invalid_input("invalid layout for alloc_array")),
		}
	}
	
	#[inline(always)]
	unsafe fn realloc_array<T>(&mut self, old_pointer: NonNull<T>, old_number_of_items: usize, new_number_of_items: usize) -> Result<NonNull<T>, AllocErr>
		where Self: Sized
	{
		match (Layout::array::<T>(old_number_of_items), Layout::array::<T>(new_number_of_items))
		{
			(Some(ref old_layout), Some(ref new_layout)) => self.pool_pointer().alloc_trait_reallocate(old_pointer.as_ptr() as *mut _, old_layout, new_layout).map(|allocation_pointer|NonNull::new_unchecked(allocation_pointer as *mut T)),
			
			_ => Err(AllocErr::invalid_input("invalid layout for realloc_array")),
		}
	}
	
	#[inline(always)]
	unsafe fn dealloc_array<T>(&mut self, pointer_to_free: NonNull<T>, number_of_items: usize) -> Result<(), AllocErr>
		where Self: Sized
	{
		match Layout::array::<T>(number_of_items)
		{
			Some(_) => Ok(self.pool_pointer().alloc_trait_free(pointer_to_free.as_ptr() as *mut _)),
			
			_ => Err(AllocErr::invalid_input("invalid layout for dealloc_array")),
		}
	}
}

impl CtoPoolAlloc
{
	#[inline(always)]
	fn allocator(&self) -> &CtoPoolArc
	{
		&self.0
	}
	
	#[inline(always)]
	fn pool_pointer(&self) -> *mut PMEMctopool
	{
		self.allocator().pool_pointer()
	}
}
