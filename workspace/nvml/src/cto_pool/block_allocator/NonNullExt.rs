// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


trait NonNullExt
{
	#[inline(always)]
	fn offset(self, offset: usize) -> Self;
	
	#[inline(always)]
	fn difference(self, larger_pointer: Self) -> usize;
}

impl<T> NonNullExt for NonNull<T>
{
	#[inline(always)]
	fn offset(self, offset: usize) -> Self
	{
		unsafe { NonNull::new_unchecked(self.as_ptr().offset(offset)) }
	}
	
	#[inline(always)]
	fn difference(self, larger_pointer: Self) -> usize
	{
		debug_assert!(larger_pointer >= self, "larger_pointer can not be less than self");
		
		let offset = (larger_pointer.as_ptr() as usize) - (self.as_ptr() as usize);
	}
}
