// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct ExponentialBackOffState<'a>(&'a BackOffState, usize);

impl<'a> ExponentialBackOffState<'a>
{
	#[inline(always)]
	fn new(back_off_state: &'a BackOffState) -> Self
	{
		ExponentialBackOffState(back_off_state, BackOffState::BACK_OFF_ITERATION_INITIAL_VALUE)
	}
	
	#[inline(always)]
	fn exponential_back_off(&mut self)
	{
		self.0.exponential_back_off(&mut self.1)
	}
	
	#[inline(always)]
	fn auto_tune(&mut self)
	{
		self.0.auto_tune(self.1)
	}
}
