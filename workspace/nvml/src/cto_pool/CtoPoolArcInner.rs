// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct CtoPoolArcInner
{
	reference_counter: AtomicUsize,
	pool_pointer: *mut PMEMctopool,
}

impl CtoPoolArcInner
{
	const MinimumReference: usize = 1;
	
	#[inline(always)]
	fn new(pool_pointer: *mut PMEMctopool) -> Self
	{
		Self
		{
			pool_pointer,
			reference_counter: AtomicUsize::new(Self::MinimumReference),
		}
	}
	
	#[inline(always)]
	fn acquire_reference(&self)
	{
		self.reference_counter.fetch_add(1, SeqCst);
	}
	
	/// Returns 'true' if the caller was the last reference.
	#[inline(always)]
	fn release_reference(&self) -> bool
	{
		if self.reference_counter.fetch_sub(1, SeqCst) == Self::MinimumReference
		{
			self.pool_pointer.close();
			true
		}
		else
		{
			false
		}
	}
}
