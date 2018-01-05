// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


trait ObjectPoolControl
{
	#[inline(always)]
	fn get_bool_global(self) -> bool;
	
	#[inline(always)]
	fn set_bool_global(self, argument: bool);
	
	#[inline(always)]
	fn get_bool(self, objectPool: *mut PMEMobjpool) -> bool;
	
	#[inline(always)]
	fn set_bool(self, objectPool: *mut PMEMobjpool, argument: bool);
	
	#[inline(always)]
	fn get_integer(self, objectPool: *mut PMEMobjpool) -> i64;
	
	#[inline(always)]
	fn set_integer(self, objectPool: *mut PMEMobjpool, argument: i64);
	
	#[doc(hidden)]
	#[inline(always)]
	fn _as_c_char_ptr(self) -> *const c_char;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _get<T: Sized>(self, objectPool: *mut PMEMobjpool, argument: &mut T) -> c_int;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _set<T: Sized>(self, objectPool: *mut PMEMobjpool, argument: &mut T) -> c_int;
}

impl ObjectPoolControl for &'static [u8]
{
	#[inline(always)]
	fn get_bool_global(self) -> bool
	{
		self.get_bool(null_mut())
	}
	
	#[inline(always)]
	fn set_bool_global(self, argument: bool)
	{
		self.set_bool(null_mut(), argument)
	}
	
	#[inline(always)]
	fn get_bool(self, objectPool: *mut PMEMobjpool) -> bool
	{
		let mut argument: c_int = unsafe { uninitialized() };
		let result = self._get(objectPool, &mut argument);
		debug_assert!(result == 0, "result was '{}'", result);
		
		if unlikely(result == -1)
		{
			panic!("get_bool failed");
		}
		
		argument != 0
	}
	
	#[inline(always)]
	fn set_bool(self, objectPool: *mut PMEMobjpool, argument: bool)
	{
		let mut argument = if argument
		{
			1
		}
		else
		{
			0
		};
		let result = self._set(objectPool, &mut argument);
		debug_assert!(result == 0, "result was '{}'", result);
		
		if unlikely(result == -1)
		{
			panic!("set_bool failed");
		}
	}
	
	#[inline(always)]
	fn get_integer(self, objectPool: *mut PMEMobjpool) -> i64
	{
		let mut argument: c_longlong = unsafe { uninitialized() };
		let result = self._get(objectPool, &mut argument);
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		
		if unlikely(result == -1)
		{
			panic!("get_integer failed");
		}
		
		argument
	}
	
	#[inline(always)]
	fn set_integer(self, objectPool: *mut PMEMobjpool, mut argument: i64)
	{
		let result = self._set(objectPool, &mut argument);
		debug_assert!(result == 0 || result == -1, "result was '{}'", result);
		
		if unlikely(result == -1)
		{
			panic!("set_integer failed");
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _as_c_char_ptr(self) -> *const c_char
	{
		self.as_ptr() as *const _
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _get<T: Sized>(self, objectPool: *mut PMEMobjpool, argument: &mut T) -> c_int
	{
		unsafe { pmemobj_ctl_get(objectPool, self._as_c_char_ptr(), argument as *mut _ as *mut c_void) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _set<T: Sized>(self, objectPool: *mut PMEMobjpool, argument: &mut T) -> c_int
	{
		unsafe { pmemobj_ctl_set(objectPool, self._as_c_char_ptr(), argument as *mut _ as *mut c_void) }
	}
}
