// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Utility trait for enforcing the form of inner data.
pub trait CtoStrongArcInner: Drop + CtoSafe
{
	/// Minimum references to initialize with and drop with.
	const MinimumReference: usize = 1;
	
	/// Acquire a reference.
	#[inline(always)]
	fn acquire_reference(&self)
	{
		self.reference_counter().fetch_add(1, SeqCst);
	}
	
	/// Release a reference.
	/// Returns 'true' if the caller was the last reference.
	/// Caller should then `drop_in_place(&mut self)`
	#[inline(always)]
	fn release_reference(&self) -> bool
	{
		self.reference_counter().fetch_sub(1, SeqCst) == Self::MinimumReference
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn new_reference_counter() -> AtomicUsize
	{
		AtomicUsize::new(Self::MinimumReference)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn reference_counter(&self) -> &AtomicUsize;
}
