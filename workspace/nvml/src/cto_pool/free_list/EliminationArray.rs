// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct EliminationArray<T>
{
	length: EliminationArrayLength,
	array: AlignedVariableLengthArray<T>,
}

impl<T> CtoSafe for EliminationArray<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let mut cache_line_index = 0;
		while cache_line_index < self.length.as_usize()
		{
			self.elimination_array_cache_line_unchecked_mut(cache_line_index).cto_pool_opened(cto_pool_arc);
			
			cache_line_index += 1;
		}
	}
}

impl<T> EliminationArray<T>
{
	#[inline(always)]
	fn random_cache_line(&self) -> &EliminationArrayCacheLine<T>
	{
		let random_index = self.random_index();
		self.elimination_array_cache_line_unchecked(random_index)
	}
	
	#[inline(always)]
	fn random_index(&self) -> usize
	{
		let random_usize = generate_thread_safe_random_usize();
		let random_index = random_usize & self.maximum_inclusive_index();
		random_index
	}
	
	#[inline(always)]
	fn maximum_inclusive_index(&self) -> usize
	{
		self.length.maximum_inclusive_index()
	}
	
	#[inline(always)]
	fn variable_size_of_elimination_array_data(elimination_array_length: EliminationArrayLength) -> usize
	{
		elimination_array_length.as_usize() * size_of::<EliminationArrayCacheLine<T>>()
	}
	
	#[inline(always)]
	fn initialize(&mut self, elimination_array_length: EliminationArrayLength)
	{
		unsafe
		{
			write(&mut self.length, elimination_array_length);
			
			// Effectively zeroed.
			let mut cache_line_index = 0;
			let length = elimination_array_length.as_usize();
			while cache_line_index < length
			{
				let elimination_array_cache_line = self.elimination_array_cache_line_unchecked(cache_line_index);
				elimination_array_cache_line.initialize();
				
				cache_line_index += 1;
			}
		}
	}
	
	#[inline(always)]
	fn elimination_array_cache_line_unchecked(&self, cache_line_index: usize) -> &EliminationArrayCacheLine<T>
	{
		debug_assert!(cache_line_index <= self.maximum_inclusive_index(), "cache_line_index '{}' exceeds self.maximum_inclusive_index() '{}'", cache_line_index, self.maximum_inclusive_index());
		
		let cache_line_base_pointer = &self.array as *const AlignedVariableLengthArray<T> as *const EliminationArrayCacheLine<T>;
		let cache_line_pointer = unsafe { cache_line_base_pointer.offset(cache_line_index as isize) };
		unsafe { & * cache_line_pointer }
	}
	
	#[inline(always)]
	fn elimination_array_cache_line_unchecked_mut(&mut self, cache_line_index: usize) -> &mut EliminationArrayCacheLine<T>
	{
		debug_assert!(cache_line_index <= self.maximum_inclusive_index(), "cache_line_index '{}' exceeds self.maximum_inclusive_index() '{}'", cache_line_index, self.maximum_inclusive_index());
		
		let cache_line_base_pointer = &mut self.array as *mut AlignedVariableLengthArray<T> as *mut EliminationArrayCacheLine<T>;
		let cache_line_pointer = unsafe { cache_line_base_pointer.offset(cache_line_index as isize) };
		unsafe { &mut * cache_line_pointer }
	}
}
