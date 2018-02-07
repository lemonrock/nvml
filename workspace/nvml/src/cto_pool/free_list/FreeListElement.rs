// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A free list element, once constructed, can never be freed.
#[derive(Debug)]
pub struct FreeListElement<T>
{
	next: *mut FreeListElement<T>,
	value: T,
}

impl<T> Drop for FreeListElement<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { drop_in_place(&mut self.value) }
	}
}

impl<T> FreeListElement<T>
{
//	#[inline(always)]
//	fn new(value: T, cto_pool_arc: &CtoPoolArc) -> NonNull<Self>
//	{
//
//	}
}
