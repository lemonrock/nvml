// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Initialise memory functions used internally by libpmemcto.
/// Use if different to libc standard (eg if using DPDK).
pub fn initialise_memory_functions
(
	malloc: unsafe extern "C" fn(size: size_t) -> *mut c_void,
	free: unsafe extern "C" fn(ptr: *mut c_void),
	realloc: unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void,
	strdup: unsafe extern "C" fn(s: *const c_char) -> *mut c_char,
	print: unsafe extern "C" fn(s: *const c_char),
)
{
	unsafe { pmemcto_set_funcs(Some(malloc), Some(free), Some(realloc), Some(strdup), Some(print)) }
}
