// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A tagged pointer is a pseudo-union.
/// A tagged pointer can be null.
/// A tagged pointer has a minimum alignment of 4 bytes.
/// A tagged pointer has a 'Delete' mark.
/// A tagged pointer has a 'LockDereference' mark, which is used in a dereference spinlock.
/// A tagged pointer should NEVER be on the stack; it is meaningless.
/// Note: We do not use `AtomicPtr` as it does not support `fetch_or()` and `fetch_and()`.
/// Note: This struct can not be `Copy` or `Clone`; do not add `#[derive(Copy, Clone)]` or `impl` to it.
#[derive(Debug)]
pub(crate) struct AtomicTaggedPointer<P>(AtomicUsize, PhantomData<P>);

impl<P> TaggedPointer<P> for AtomicTaggedPointer<P>
{
	#[doc(hidden)]
	#[inline(always)]
	fn value(&self) -> usize
	{
		self.0.load(Acquire)
	}
}

impl<P> PartialEq for AtomicTaggedPointer<P>
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

impl<P> Eq for AtomicTaggedPointer<P>
{
}

impl<P> AtomicTaggedPointer<P>
{
	pub(crate) const Null: Self = Self::new(0);
	
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CAS(&self, old_tagged_pointer: NonAtomicTaggedPointer<P>, new_tagged_pointer: NonAtomicTaggedPointer<P>) -> bool
	{
		self.0.compare_exchange(old_tagged_pointer.value(), new_tagged_pointer.value(), AcqRel, Relaxed).is_ok()
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, new_tagged_pointer: NonAtomicTaggedPointer<P>)
	{
		self.0.store(new_tagged_pointer.value(), Release)
	}
	
	const LockDereferenceTagBit: usize = <AtomicTaggedPointer<P> as TaggedPointer<P>>::LockDereferenceTagBit;
	
	const LockDereferenceTagBitMask: usize = <AtomicTaggedPointer<P> as TaggedPointer<P>>::LockDereferenceTagBitMask;
	
	#[inline(always)]
	pub(crate) fn acquire_spinlock(&self) -> NonAtomicTaggedPointer<P>
	{
		// If the LockDereferenceBit is already set, this loop won't change it.
		// The loop exits when the LockDereferenceBit has been set by us.
		let mut previous;
		while
		{
			previous = self.0.fetch_or(Self::LockDereferenceTagBit, Acquire);
			previous & Self::LockDereferenceTagBit != 0
		}
		{
			Back_Off()
		}
		
		NonAtomicTaggedPointer::new(previous)
	}
	
	/// ***MUST*** only be called after `acquire_spinlock()`.
	#[inline(always)]
	pub(crate) fn release_spinlock(&self, _to_make_it_harder_to_abuse: NonAtomicTaggedPointer<P>)
	{
		debug_assert!(self.has_lock_dereference_tag(), "Is not locked");
		
		// Atomically clears the LockDereferenceTagBit.
		let previous = self.0.fetch_and(Self::LockDereferenceTagBitMask, Release);
		
		debug_assert_eq!(previous & Self::LockDereferenceTagBit, Self::LockDereferenceTagBit, "Was not previously locked, yet we tested that that the lock_dereference_tag was set. This implies that another thread unlocked this tagged pointer when it did not hold the lock.");
	}
	
	#[inline(always)]
	pub(crate) fn from_raw_pointer(raw_pointer: *mut P) -> Self
	{
		let raw_pointer = raw_pointer as usize;
		
		debug_assert_eq!(raw_pointer % Self::MinimumPointerAlignment, 0, "raw_pointer does not have a minimum alignment of {} (required to use the bottom few bits for tags)", Self::MinimumPointerAlignment);
		
		Self::new(raw_pointer)
	}
	
	#[inline(always)]
	const fn new(value: usize) -> Self
	{
		AtomicTaggedPointer(AtomicUsize::new(value), PhantomData)
	}
}
