// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct CtoRcCounter(Cell<usize>);

impl Default for CtoRcCounter
{
	#[inline(always)]
	fn default() -> Self
	{
		CtoRcCounter(Cell::new(0))
	}
}

impl CtoRcCounter
{
	#[inline(always)]
	fn count(&self) -> usize
	{
		self.0.get()
	}
	
	// The use of a checked_add is to handle overflow of the reference count. This is a degenerate scenario with minimal overhead.
	#[inline(always)]
	fn increment(&self)
	{
		self.0.set(self.count().checked_add(1).unwrap_or_else(|| abort()));
	}
	
	#[inline]
	fn decrement(&self)
	{
		self.0.set(self.count() - 1);
	}
}
