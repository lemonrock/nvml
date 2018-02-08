// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Provides a strong atomically referenced counted ('Arc') wrapper around a FreeList.
/// When the last instance of `CtoFreeListArc` is dropped, the FreeList is dropped and all FreeListElements in the list are dropped.
pub struct CtoFreeListArc<T>(NonNull<FreeList<T>>);

unsafe impl<T> Send for CtoFreeListArc<T>
{
}

unsafe impl<T> Sync for CtoFreeListArc<T>
{
}

impl<T> CtoSafe for CtoFreeListArc<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.deref_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<T> Drop for CtoFreeListArc<T>
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

impl<T> Clone for CtoFreeListArc<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		self.deref().acquire_reference();
		CtoFreeListArc(self.0)
	}
}

impl<T> Deref for CtoFreeListArc<T>
{
	type Target = FreeList<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { self.0.as_ref() }
	}
}

impl<T> DerefMut for CtoFreeListArc<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { self.0.as_mut() }
	}
}

impl<T> CtoFreeListArc<T>
{
	/// Create a new instance.
	/// Supply a `free_list_element_provider` if you want to make sure the elimination array is initially populated.
	/// This can return `None` if it no longer can provide free list elements.
	/// `elimination_array_length` should be equivalent to the number of threads.
	#[inline(always)]
	pub fn new<FreeListElementProvider: Fn(&CtoPoolArc) -> Option<InitializedFreeListElement<T>>>(allocator: &CtoPoolArc, elimination_array_length: EliminationArrayLength, free_list_element_provider: Option<FreeListElementProvider>) -> Self
	{
		CtoFreeListArc(FreeList::new(allocator, elimination_array_length, free_list_element_provider))
	}
}
