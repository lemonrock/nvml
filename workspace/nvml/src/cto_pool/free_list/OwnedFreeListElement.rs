// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A wrapper that allows the owner to make use of methods inside a `FreeListElement` via `deref_mut()`.
/// Ensures that thread-unsafe methods, and memory unsafe methods, are never called.
/// It should be safe to move this wrapper into any free list, not necessarily the same one a `FreeListElement` was popped from.
#[derive(Debug)]
pub struct OwnedFreeListElement<T>(NonNull<FreeListElement<T>>);

impl<T> PartialEq for OwnedFreeListElement<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0.as_ptr() == other.0.as_ptr()
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		self.0.as_ptr() == other.0.as_ptr()
	}
}

impl<T> Eq for OwnedFreeListElement<T>
{
}

impl<T> Deref for OwnedFreeListElement<T>
{
	type Target = FreeListElement<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { self.0.as_ref() }
	}
}

impl<T> DerefMut for OwnedFreeListElement<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { self.0.as_mut() }
	}
}

impl<T> OwnedFreeListElement<T>
{
	#[inline(always)]
	fn new(cto_pool_arc: &CtoPoolArc, initial_value: T, trailing_additional_size_in_value_in_bytes: usize) -> OwnedFreeListElement<T>
	{
		OwnedFreeListElement(FreeListElement::new(cto_pool_arc, initial_value, trailing_additional_size_in_value_in_bytes))
	}
	
	#[inline(always)]
	fn internal_clone(&self) -> Self
	{
		OwnedFreeListElement(self.0)
	}
	
	#[inline(always)]
	fn into_inner(self) -> NonNull<FreeListElement<T>>
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) fn from_non_null(free_list_element: NonNull<FreeListElement<T>>) -> Self
	{
		OwnedFreeListElement(free_list_element)
	}
	
	#[inline(always)]
	pub(crate) fn from_non_null_pointer(free_list_element: *mut FreeListElement<T>) -> Self
	{
		OwnedFreeListElement(free_list_element.to_non_null())
	}
	
	#[inline(always)]
	pub(crate) fn to_non_null(&self) -> NonNull<FreeListElement<T>>
	{
		self.0
	}
}
