// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct RemovalCounter(AtomicU64);

impl Default for RemovalCounter
{
	#[inline(always)]
	fn default() -> Self
	{
		RemovalCounter(AtomicU64::new(0))
	}
}

impl RemovalCounter
{
	#[inline(always)]
	fn current_count(&self) -> u64
	{
		self.0.load(Relaxed)
	}
	
	#[inline(always)]
	fn increment(&self)
	{
		self.0.fetch_add(1, Relaxed);
	}
}
