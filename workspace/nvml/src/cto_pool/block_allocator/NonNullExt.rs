// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


trait NonNullExt<T>
{
	#[inline(always)]
	fn offset(self, offset: usize) -> Self;
	
	#[inline(always)]
	fn difference(self, larger_pointer: Self) -> usize;
	
	#[inline(always)]
	fn longer_as_ref<'long>(self) -> &'long T;
}

impl<T> NonNullExt<T> for NonNull<T>
{
	#[inline(always)]
	fn offset(self, offset: usize) -> Self
	{
		debug_assert!(offset <= (::std::isize::MAX as usize), "offset exceeds isize::MAX");
		
		unsafe { NonNull::new_unchecked(self.as_ptr().offset(offset as isize)) }
	}
	
	#[inline(always)]
	fn difference(self, larger_pointer: Self) -> usize
	{
		let larger_pointer = larger_pointer.as_ptr() as usize;
		let self_pointer = self.as_ptr() as usize;
		
		debug_assert!(larger_pointer >= self_pointer, "larger_pointer can not be less than self");
		
		larger_pointer - self_pointer
	}
	
	#[inline(always)]
	fn longer_as_ref<'long>(self) -> &'long T
	{
		unsafe { & * self.as_ptr() }
	}
}
