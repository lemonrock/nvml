// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A convenient way to use the methods on a cto pool.
pub trait PMEMctopoolEx
{
	/// Prints statistics to standard error or calls the callback specified using `print` in `initialise_memory_functions()`.
	/// self can not be null.
	#[inline(always)]
	fn print_statistics(self);
	
	/// Close the CTO pool.
	/// self can not be null.
	#[inline(always)]
	fn close(self);
	
	/// Get the root pointer which can be null.
	/// self can not be null.
	#[inline(always)]
	fn get_root<T>(self) -> *mut T;
	
	/// Set the root pointer. Should only be used if the current root pointer is null. Root pointer to set should not be null.
	/// Should be a pointer to an object previously created with one of the following methods on *mut PMEMctopool:-
	/// * `malloc()`
	/// * `aligned_alloc()`
	/// * `calloc()`
	/// * `strdup()`
	/// * `wcsdup()`
	/// The persistent object must eventually be free'd with our `free()`.
	/// self can not be null.
	#[inline(always)]
	fn set_root<T>(self, root: *mut T);
	
	/// The size_of::<T> must not be zero.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn malloc<T>(self) -> Result<*mut T, PmdkError>;
	
	/// The size_of::<T> must not be zero.
	/// The align_of::<T> must be a power of two.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn aligned_alloc<T>(self) -> Result<*mut T, PmdkError>;
	
	/// Aligned allocation.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn aligned_allocate_from_layout(self, layout: &Layout) -> Result<*mut u8, PmdkError>;
	
	/// The size_of::<T> must not be zero.
	/// Count can not be zero.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn calloc<T>(self, count: size_t) -> Result<*mut T, PmdkError>;
	
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn strdup(self, value: &CStr) -> Result<*mut c_char, PmdkError>;
	
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn wcsdup(self, value: *const wchar_t) -> Result<*mut wchar_t, PmdkError>;
	
	/// Pointer must not be null.
	/// self can not be null.
	#[inline(always)]
	fn usable_size<T>(self, pointer: *mut T) -> size_t;
	
	/// Pointer must not be null.
	/// new_size can not be zero.
	/// If memory can not be allocated, returns a PmdkError with `isENOMEM()` true. Never returns `Ok(null_mut())`.
	/// self can not be null.
	#[inline(always)]
	fn realloc(self, pointer: *mut c_void, new_size: size_t) -> Result<*mut c_void, PmdkError>;
	
	/// Pointer must not be null.
	/// self can not be null.
	#[inline(always)]
	fn free<T>(self, pointer: *mut T);
}

impl PMEMctopoolEx for *mut PMEMctopool
{
	#[inline(always)]
	fn print_statistics(self)
	{
		debug_assert!(!self.is_null(), "self can not be null");
		
		unsafe { pmemcto_stats_print(self, null()) }
	}
	
	#[inline(always)]
	fn close(self)
	{
		debug_assert!(!self.is_null(), "self can not be null");
		
		unsafe { pmemcto_close(self) }
	}
	
	#[inline(always)]
	fn get_root<T>(self) -> *mut T
	{
		debug_assert!(!self.is_null(), "self can not be null");
		
		unsafe { pmemcto_get_root_pointer(self) as *mut T }
	}
	
	#[inline(always)]
	fn set_root<T>(self, root: *mut T)
	{
		debug_assert!(!self.is_null(), "self can not be null");
		debug_assert!(!root.is_null(), "root can not be null");
		
		unsafe { pmemcto_set_root_pointer(self, root as *mut _) }
	}
	
	#[inline(always)]
	fn malloc<T>(self) -> Result<*mut T, PmdkError>
	{
		debug_assert!(!self.is_null(), "self can not be null");
		
		let size = size_of::<T>() as size_t;
		debug_assert!(size != 0, "size_of::<T>() can not be zero");
		
		let result = unsafe { pmemcto_malloc(self, size) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_malloc")
		}
		else
		{
			Ok(result as *mut T)
		}
	}
	
	#[inline(always)]
	fn aligned_alloc<T>(self) -> Result<*mut T, PmdkError>
	{
		#[inline(always)]
		fn is_power_of_two(value: size_t) -> bool
		{
			(value != 0) && ((value & (value - 1)) == 0)
		}
		
		debug_assert!(!self.is_null(), "self can not be null");
		
		let alignment = align_of::<T>();
		debug_assert!(!is_power_of_two(alignment), "alignment must be a power of two");
		
		let size = size_of::<T>() as size_t;
		debug_assert!(size != 0, "size_of::<T>() can not be zero");
		
		let result = unsafe { pmemcto_aligned_alloc(self, alignment, size) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_aligned_alloc")
		}
		else
		{
			Ok(result as *mut T)
		}
	}
	
	#[inline(always)]
	fn aligned_allocate_from_layout(self, layout: &Layout) -> Result<*mut u8, PmdkError>
	{
		#[inline(always)]
		fn is_power_of_two(value: size_t) -> bool
		{
			(value != 0) && ((value & (value - 1)) == 0)
		}
		
		let alignment = layout.align();
		debug_assert!(!is_power_of_two(alignment), "align() must be a power of two");
		
		let size = layout.size();
		debug_assert!(size != 0, "size() can not be zero");
		
		let result = unsafe { pmemcto_aligned_alloc(self, alignment, size) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_aligned_alloc")
		}
		else
		{
			Ok(result as *mut u8)
		}
	}
	
	#[inline(always)]
	fn calloc<T>(self, count: size_t) -> Result<*mut T, PmdkError>
	{
		debug_assert!(!self.is_null(), "self can not be null");
		debug_assert!(count != 0, "count can not be zero");
		
		let size = size_of::<T>() as size_t;
		debug_assert!(size != 0, "size_of::<T>() can not be zero");
		
		let result = unsafe { pmemcto_calloc(self, count, size) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_calloc")
		}
		else
		{
			Ok(result as *mut T)
		}
	}
	
	#[inline(always)]
	fn strdup(self, string: &CStr) -> Result<*mut c_char, PmdkError>
	{
		debug_assert!(!self.is_null(), "self can not be null");
		
		let result = unsafe { pmemcto_strdup(self, string.as_ptr()) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_strdup")
		}
		else
		{
			Ok(result as *mut c_char)
		}
	}
	
	#[inline(always)]
	fn wcsdup(self, string: *const wchar_t) -> Result<*mut wchar_t, PmdkError>
	{
		debug_assert!(!self.is_null(), "self can not be null");
		debug_assert!(!string.is_null(), "string can not be null");
		
		let result = unsafe { pmemcto_wcsdup(self, string) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_wcsdup")
		}
		else
		{
			Ok(result as *mut wchar_t)
		}
	}
	
	#[inline(always)]
	fn usable_size<T>(self, pointer: *mut T) -> size_t
	{
		debug_assert!(!self.is_null(), "self can not be null");
		debug_assert!(!pointer.is_null(), "pointer can not be null");
		
		unsafe { pmemcto_malloc_usable_size(self, pointer as *mut _) }
	}
	
	#[inline(always)]
	fn realloc(self, pointer: *mut c_void, new_size: size_t) -> Result<*mut c_void, PmdkError>
	{
		debug_assert!(!self.is_null(), "self can not be null");
		debug_assert!(!pointer.is_null(), "pointer can not be null");
		debug_assert!(new_size != 0, "new_size can not be zero");
		
		let result = unsafe { pmemcto_realloc(self, pointer, new_size) };
		if unlikely(result.is_null())
		{
			PmdkError::cto("pmemcto_realloc")
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn free<T>(self, pointer: *mut T)
	{
		debug_assert!(!self.is_null(), "self can not be null");
		debug_assert!(!pointer.is_null(), "pointer can not be null");
		
		unsafe { pmemcto_free(self, pointer as *mut _) }
	}
}
