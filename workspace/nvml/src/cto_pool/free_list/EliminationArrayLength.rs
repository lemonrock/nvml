// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A wrapper type ensuring valid lengths for the elimination array used in a FreeList.s
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EliminationArrayLength(usize);

impl EliminationArrayLength
{
	/// number_of_threads will be rounded up to a power of two, with a minimum of 2.
	#[inline(always)]
	pub fn number_of_threads_to_length(number_of_threads: usize) -> Self
	{
		assert_ne!(number_of_threads, 0, "number_of_threads can not be zero");
		
		let length = if number_of_threads == 1
		{
			2
		}
		else
		{
			number_of_threads.checked_next_power_of_two().expect("number_of_threads is so large that it can not be rounded up to a power of two")
		};
		EliminationArrayLength(length)
	}
	
	#[inline(always)]
	fn maximum_inclusive_index(self) -> usize
	{
		self.as_usize() - 1
	}
	
	#[inline(always)]
	fn as_usize(self) -> usize
	{
		self.0
	}
}
