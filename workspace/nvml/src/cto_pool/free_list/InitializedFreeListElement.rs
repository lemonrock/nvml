// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A specialist type that, once, pushed, can no longer be dropped.
pub struct InitializedFreeListElement<'free_list, T: 'free_list>
{
	inner: OwnedFreeListElement<T>,
	free_list: &'free_list FreeList<T>,
}

impl<'free_list, T> Drop for InitializedFreeListElement<'free_list, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.inner.deref_mut().free_list_is_being_dropped_or_was_never_pushed_ever_so_free(&self.free_list.cto_pool_arc)
	}
}

impl<'free_list, T> Deref for InitializedFreeListElement<'free_list, T>
{
	type Target = FreeListElement<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.inner.deref()
	}
}

impl<'free_list, T> DerefMut for InitializedFreeListElement<'free_list, T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.inner.deref_mut()
	}
}

impl<'free_list, T> InitializedFreeListElement<'free_list, T>
{
	/// Pushes onto a free list.
	/// Once pushed, can no longer be dropped until the free list is dropped.
	/// Which might be never...
	#[inline(always)]
	pub fn push(self)
	{
		let free_list = self.free_list;
		free_list.push(self.into_inner());
	}
	
	#[inline(always)]
	fn into_inner(self) -> OwnedFreeListElement<T>
	{
		let inner = self.inner.internal_clone();
		forget(self);
		inner
	}
}
