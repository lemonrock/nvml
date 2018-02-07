// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct EliminationArrayCacheLine<T>
{
	cache_line: [EliminationArrayEntry<T>; MaximumNumberOfFreeListElementPointersThatFitInACacheLine]
}

impl<T> EliminationArrayCacheLine<T>
{
	#[inline(always)]
	fn initialize(&self)
	{
		let mut entry_index = 0;
		while entry_index < MaximumNumberOfFreeListElementPointersThatFitInACacheLine
		{
			self.entry(entry_index).set_initial_value_to_null();
			entry_index += 1;
		}
	}
	
	#[inline(always)]
	fn entry(&self, entry_index: usize) -> &EliminationArrayEntry<T>
	{
		unsafe { self.cache_line.get_unchecked(entry_index) }
	}
}
