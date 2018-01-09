// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct CtoPoolAllocGuard
{
	pool_pointer: *mut PMEMctopool,
	counter: AtomicUsize,
}

impl CtoPoolAllocGuard
{
	#[inline(always)]
	fn acquire(&mut self)
	{
		self.counter.fetch_add(1, SeqCst);
	}
	
	/// Returns 'true' if the caller was the last reference.
	#[inline(always)]
	fn release(&mut self) -> bool
	{
		if self.counter.fetch_sub(1, SeqCst) == 1
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
