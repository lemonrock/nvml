// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A trait for persistence operations.
pub trait Persistence
{
	/// flush struct.
	#[inline(always)]
	fn flush_struct<T>(address: &T)
	{
		Self::flush_memory(address as *const T as *mut T as *mut c_void, size_of::<T>())
	}
	
	/// flush NonNull.
	#[inline(always)]
	fn flush_non_null<T>(address: NonNull<T>)
	{
		Self::flush_memory(address.as_ptr() as *mut c_void, size_of::<T>())
	}
	
	/// flush memory.
	#[inline(always)]
	fn flush_memory(address: *mut c_void, length: usize);
	
	/// drain memory.
	#[inline(always)]
	fn drain_memory();
}
