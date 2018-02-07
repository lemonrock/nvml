// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct BackOffState
{
	spin_lock: BestSpinLockForCompilationTarget,
	back_off_iteration_frequency_counters: [AtomicUsize; 2],
	metric: AtomicUsize,
	total_operations: AtomicUsize,
}

impl Default for BackOffState
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			spin_lock: BestSpinLockForCompilationTarget::default(),
			back_off_iteration_frequency_counters: [AtomicUsize::new(0), AtomicUsize::new(0)],
			metric: AtomicUsize::new(1),
			total_operations: AtomicUsize::new(0),
		}
	}
}

impl BackOffState
{
	const BACK_OFF_ITERATION_INITIAL_VALUE: usize = 0;
	
	const BACK_OFF_ITERATION_LIMIT: usize = 10;
	
	#[inline(always)]
	fn exponential_back_off(&self, back_off_iteration: &mut usize)
	{
		if *back_off_iteration == Self::BACK_OFF_ITERATION_LIMIT
		{
			*back_off_iteration = Self::BACK_OFF_ITERATION_INITIAL_VALUE
		}
		else
		{
			let end = (0x1 << *back_off_iteration) * self.metric();
			let mut counter = 0;
			while counter < end
			{
				counter += 1
			}
		}
		
		*back_off_iteration = *back_off_iteration + 1
	}
	
	#[inline(always)]
	fn auto_tune(&self, back_off_iteration: usize)
	{
		if back_off_iteration < 2
		{
			self.increment_back_off_iteration_frequency_counter(back_off_iteration);
		}
		
		let total_operations = self.increment_total_operations();
		
		if total_operations >= 9999 && self.spin_lock.is_unlocked()
		{
			if self.spin_lock.try_to_acquire_spin_lock()
			{
				// "if E[1] is less than 1/100th of E[0], decrease the metric, to increase E[1]"
				if self.load_back_off_iteration_frequency_counter(1) < self.load_back_off_iteration_frequency_counter(0) / 100
				{
					if self.metric() >= 11
					{
						self.metric.fetch_sub(10, Relaxed);
					}
					else
					{
						self.metric.fetch_add(10, Relaxed);
					}
				}
				
				self.reset_back_off_iteration_frequency_counter(0);
				self.reset_back_off_iteration_frequency_counter(1);
				self.reset_total_operations();
				
				fence(Release);
				self.spin_lock.unlock_spin_lock();
			}
		}
	}
	
	#[inline(always)]
	fn metric(&self) -> usize
	{
		self.metric.load(Relaxed)
	}
	
	#[inline(always)]
	fn increment_total_operations(&self) -> usize
	{
		self.total_operations.fetch_add(1, Relaxed)
	}
	
	#[inline(always)]
	fn reset_total_operations(&self)
	{
		self.total_operations.store(0, Relaxed)
	}
	
	#[inline(always)]
	fn back_off_iteration_frequency_counter(&self, index: usize) -> &AtomicUsize
	{
		unsafe { self.back_off_iteration_frequency_counters.get_unchecked(index) }
	}
	
	#[inline(always)]
	fn load_back_off_iteration_frequency_counter(&self, index: usize) -> usize
	{
		self.back_off_iteration_frequency_counter(index).load(Relaxed)
	}
	
	#[inline(always)]
	fn increment_back_off_iteration_frequency_counter(&self, index: usize)
	{
		self.back_off_iteration_frequency_counter(index).fetch_add(1, Relaxed);
	}
	
	#[inline(always)]
	fn reset_back_off_iteration_frequency_counter(&self, index: usize)
	{
		self.back_off_iteration_frequency_counter(index).store(0, Relaxed)
	}
}
