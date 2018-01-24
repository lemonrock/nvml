// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Like a `AtomicTaggedPointer`, but only suitable for use on the stack.
/// Constructed only by `AtomicTaggedPointer`.
/// Should never have the `LockDereferenceTagBit` set, and is constructed with it cleared.
#[derive(Debug)]
pub(crate) struct NonAtomicTaggedPointer<P>(usize, PhantomData<P>);

unsafe impl<P> Send for NonAtomicTaggedPointer<P>
{
}

unsafe impl<P> Sync for NonAtomicTaggedPointer<P>
{
}

impl<P> Clone for NonAtomicTaggedPointer<P>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl<P> Copy for NonAtomicTaggedPointer<P>
{
}

impl<P> TaggedPointer<P> for NonAtomicTaggedPointer<P>
{
	#[doc(hidden)]
	#[inline(always)]
	fn value(&self) -> usize
	{
		self.0
	}
}

impl<P> PartialEq for NonAtomicTaggedPointer<P>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self._without_lock_dereference_tag_but_with_delete_tag_if_set() == other._without_lock_dereference_tag_but_with_delete_tag_if_set()
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		self._without_lock_dereference_tag_but_with_delete_tag_if_set() != other._without_lock_dereference_tag_but_with_delete_tag_if_set()
	}
}

impl<P> Eq for NonAtomicTaggedPointer<P>
{
}

impl<P> NonAtomicTaggedPointer<P>
{
	// Lint is wrong, as we access Null though StackLink::Null, which is a type definition.
	#[allow(dead_code)]
	pub(crate) const Null: Self = Self::new(0);
	
	#[inline(always)]
	const fn new(value: usize) -> Self
	{
		NonAtomicTaggedPointer(value, PhantomData)
	}
}
