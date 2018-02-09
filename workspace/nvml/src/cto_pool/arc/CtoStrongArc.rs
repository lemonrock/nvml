// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Provides a strong atomically referenced counted ('Arc') wrapper around a thread-safe inner data structure.
/// When the last instance of `CtoStringArc` is dropped, the inner data structure is dropped.
pub struct CtoStrongArc<I: CtoStrongArcInner>(NonNull<I>);

unsafe impl<I: CtoStrongArcInner> Send for CtoStrongArc<I>
{
}

unsafe impl<I: CtoStrongArcInner> Sync for CtoStrongArc<I>
{
}

impl<I: CtoStrongArcInner> CtoSafe for CtoStrongArc<I>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.deref_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<I: CtoStrongArcInner> Drop for CtoStrongArc<I>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.deref().release_reference()
		{
			unsafe { drop_in_place(self.deref_mut()) }
		}
	}
}

impl<I: CtoStrongArcInner> Clone for CtoStrongArc<I>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		self.deref().acquire_reference();
		CtoStrongArc(self.0)
	}
}

impl<I: CtoStrongArcInner> Deref for CtoStrongArc<I>
{
	type Target = I;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { self.0.as_ref() }
	}
}

impl<I: CtoStrongArcInner> DerefMut for CtoStrongArc<I>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { self.0.as_mut() }
	}
}

impl<I: CtoStrongArcInner> CtoStrongArc<I>
{
	/// Creates a new instance around a reference counted inner.
	/// Use with caution.
	#[inline(always)]
	pub fn new(inner: NonNull<I>) -> Self
	{
		CtoStrongArc(inner)
	}
}
