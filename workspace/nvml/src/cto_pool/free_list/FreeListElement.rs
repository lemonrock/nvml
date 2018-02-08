// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A free list element, once `push()'d`, can never be freed until the FreeList is dropped.
/// It is recommended that `T` is `Option<SomeValue>`, to make it easier to take the value.
#[derive(Debug)]
pub struct FreeListElement<T>
{
	next: *mut FreeListElement<T>,
	value: T,
}

impl<T: Copy> FreeListElement<T>
{
	/// Returns a copy of the value.
	/// Useful if T is a raw pointer or `NonNull`.
	#[inline(always)]
	pub fn copy_value(&mut self) -> T
	{
		self.value
	}
}

impl<T: Clone> FreeListElement<T>
{
	/// Returns a clone of the value.
	/// Useful if T is an Arc.
	/// Note that the original value will not be dropped until the FreeList itself is dropped, once this FreeListElement has been `push()'d`.
	#[inline(always)]
	pub fn clone_value(&mut self) -> T
	{
		self.value.clone()
	}
}

impl<V> FreeListElement<Option<V>>
{
	/// Takes the value held by this FreeListElement, replacing it with `None`, and dropping neither.
	#[inline(always)]
	pub fn take_value(&mut self) -> Option<V>
	{
		self.replace_value(None)
	}
	
	/// Takes the value held by this FreeListElement, replacing it with `None`, and dropping neither.
	/// Panics if the value is already `None`, ie it was already taken.
	#[inline(always)]
	pub fn take_value_once(&mut self) -> V
	{
		self.take_value().expect("value was already taken")
	}
}



impl<T> FreeListElement<T>
{
	/// Returns the value held by this FreeListElement, replacing it with the `replacement` and dropping neither.
	/// The replacement value will not be dropped until the FreeList itself is dropped, once this FreeListElement has been `push()'d`.
	#[inline(always)]
	pub fn replace_value(&mut self, replacement: T) -> T
	{
		unsafe { replace(&mut self.value, replacement) }
	}
	
	#[inline(always)]
	fn free_list_is_being_dropped_or_was_never_pushed_ever_so_free(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		unsafe { drop_in_place(&mut self.value) };
		
		cto_pool_arc.pool_pointer().free(self);
	}
}
//	#[inline(always)]
//	fn new(value: T, cto_pool_arc: &CtoPoolArc) -> NonNull<Self>
//	{
//
//	}

/*
The state and element structures are both public, present in the lfds711_freelist.h header file, so that users can embed them in their own structures (and where necessary pass them to sizeof). Expected use is that user structures which are to enter freelists contain within themselves a struct lfds711_freelist_element, and this is used when calling lfds711_freelist_push, and the value set in the freelist element is a pointer to the user structure entering the freelist. This approach permits zero run-time allocation of store and also ensures the freelist element is normally in the same memory page as the user data it refers to.


Once a freelist element structure has been pushed to the stack, it cannot be deallocated (free, or stack allocation lifetimes ending due to say a thread ending, etc) until lfds711_freelist_cleanup has returned



Consider populating the elimination array before first pop()

*/
