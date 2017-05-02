// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Initialise libpmem libraries' memory allocation functions. Needed if using, say, DPDK with libpmem
/// Done as a macro rather than a function as strdup() needs to wrap malloc(), yet we can not pass a pointer to malloc() to strdup() or use a closure (which has a hidden self parameter and thus a different signature)
///
/// # Arguments
///
/// * `$malloc` - An `unsafe extern "C" fn(size: size_t) -> *mut c_void` function. If defined in rust, should be specified `#[inline(always)]`, as it is used inside a generated `strdup()` function.
/// * `$free` - An `unsafe extern "C" fn(ptr: *mut c_void)` function
/// * `$realloc` - An `unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void` function
///
/// # Example
///
/// This example uses DPDK.
/// ```
/// extern crate dpdk_sys;
/// extern crate libc;
///
/// use ::libc::c_void;
/// use ::libc::size_t;
/// use ::std::ptr::null;
///
/// #[inline(always)]
/// unsafe extern "C" fn dpdkMalloc(size: size_t) -> *mut c_void
/// {
/// 	::dpdk_sys::rte_malloc(null(), size, 0)
/// }
///
/// unsafe extern "C" fn dpdkRealloc(ptr: *mut c_void, size: size_t) -> *mut c_void
/// {
/// 	::dpdk_sys::rte_realloc(ptr, size, 0)
/// }
///
/// initialiseMemoryFunctions!(dpdkMalloc, dpdkRealloc ::dpdk_sys::rte_free);
/// ```
#[macro_export]
macro_rules! initialiseMemoryFunctions
{
	($malloc: path, $free: path, $realloc: path) =>
	{
		{
			use $crate::libc::c_char;
			use $crate::libc::strlen;
			use $crate::rust_extra::unlikely;
			use ::std::ptr::copy_nonoverlapping;
			use ::std::ptr::null_mut;
			
			// Based on __strdup() in musl libc; wraps $malloc()
			#[inline(always)]
			unsafe extern "C" fn strdup(source: *const c_char) -> *mut c_char
			{
				let length = strlen(source);
				let lengthIncludingTrailingAsciiNul = length + 1;
				let destination = $malloc(lengthIncludingTrailingAsciiNul) as *mut c_char;
				if unlikely(destination.is_null())
				{
					null_mut()
				}
				else
				{
					copy_nonoverlapping(source, destination, lengthIncludingTrailingAsciiNul);
					destination
				}
			}
			
			#[inline(always)]
			fn initialiseMemoryFunctions
			(
				malloc: unsafe extern "C" fn(size: size_t) -> *mut c_void,
				free: unsafe extern "C" fn(ptr: *mut c_void),
				realloc: unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void,
				strdup: unsafe extern "C" fn(s: *const c_char) -> *mut c_char
			)
			{
				$crate::blockPool::initialiseMemoryFunctions(malloc, free, realloc, strdup);
				$crate::logPool::initialiseMemoryFunctions(malloc, free, realloc, strdup);
				$crate::transactionalObjectPool::initialiseMemoryFunctions(malloc, free, realloc, strdup);
			}
			
			initialiseMemoryFunctions($malloc, $free, $realloc, strdup)
		}
	}
}
