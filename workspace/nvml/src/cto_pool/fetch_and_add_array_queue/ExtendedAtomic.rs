// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


trait ExtendedAtomic<T>
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: T);
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent(&self, compare: T, value: T) -> bool;
}

impl ExtendedAtomic<u32> for AtomicU32
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: u32)
	{
		unsafe { (self as *mut Self).write(Self::new(initial_value)) }
	}
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent(&self, compare: u32, value: u32) -> bool
	{
		self.compare_exchange(compare, value, SeqCst, SeqCst).is_ok()
	}
}

impl<T> ExtendedAtomic<*mut T> for AtomicPtr<T>
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: *mut T)
	{
		unsafe { (self as *mut Self).write(Self::new(initial_value)) }
	}
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent(&self, compare: *mut T, value: *mut T) -> bool
	{
		self.compare_exchange(compare, value, SeqCst, SeqCst).is_ok()
	}
}
