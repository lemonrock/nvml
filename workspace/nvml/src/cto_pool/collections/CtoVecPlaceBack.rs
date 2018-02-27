// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A place for insertion at the back of a `Vec`.
#[must_use = "places do nothing unless written to with `<-` syntax"]
#[derive(Debug)]
pub struct CtoVecPlaceBack<'a, T: 'a + CtoSafe>
{
	vec: &'a mut CtoVec<T>,
}

impl<'a, T: CtoSafe> Placer<T> for CtoVecPlaceBack<'a, T>
{
	type Place = CtoVecPlaceBack<'a, T>;
	
	#[inline(always)]
	fn make_place(self) -> Self
	{
		// This will panic or abort if we would allocate > `isize::MAX` bytes or if the length increment would overflow for zero-sized types.
		if self.vec.len == self.vec.buf.cap()
		{
			self.vec.buf.double();
		}
		self
	}
}

unsafe impl<'a, T: CtoSafe> Place<T> for CtoVecPlaceBack<'a, T>
{
	#[inline(always)]
	fn pointer(&mut self) -> *mut T
	{
		unsafe { self.vec.as_mut_ptr().offset(self.vec.len as isize) }
	}
}

impl<'a, T: CtoSafe> InPlace<T> for CtoVecPlaceBack<'a, T>
{
	type Owner = &'a mut T;
	
	#[inline(always)]
	unsafe fn finalize(mut self) -> &'a mut T
	{
		let ptr = self.pointer();
		self.vec.len += 1;
		&mut *ptr
	}
}
