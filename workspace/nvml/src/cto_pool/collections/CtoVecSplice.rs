// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A splicing iterator for `Vec`.
#[derive(Debug)]
pub struct CtoVecSplice<'a, I: Iterator + 'a>
	where <I as Iterator>::Item: CtoSafe
{
	drain: CtoVecDrain<'a, I::Item>,
	replace_with: I,
}

impl<'a, I: Iterator> Iterator for CtoVecSplice<'a, I>
	where <I as Iterator>::Item: CtoSafe
{
	type Item = I::Item;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		self.drain.next()
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		self.drain.size_hint()
	}
}

impl<'a, I: Iterator> DoubleEndedIterator for CtoVecSplice<'a, I>
	where <I as Iterator>::Item: CtoSafe
{
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item>
	{
		self.drain.next_back()
	}
}

impl<'a, I: Iterator> ExactSizeIterator for CtoVecSplice<'a, I>
	where <I as Iterator>::Item: CtoSafe
{
}

impl<'a, I: Iterator> Drop for CtoVecSplice<'a, I>
	where <I as Iterator>::Item: CtoSafe
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// exhaust drain first
		while let Some(_) = self.drain.next()
		{
		}
		
		unsafe
		{
			if self.drain.tail_len == 0
			{
				self.drain.vec.as_mut().extend(self.replace_with.by_ref());
				return
			}
			
			// First fill the range left by drain().
			if !self.drain.fill(&mut self.replace_with)
			{
				return
			}
			
			// There may be more elements. Use the lower bound as an estimate.
			// Is the upper bound a better guess? Or something else?
			let (lower_bound, _upper_bound) = self.replace_with.size_hint();
			if lower_bound > 0
			{
				self.drain.move_tail(lower_bound);
				if !self.drain.fill(&mut self.replace_with) {
					return
				}
			}
			
			// Collect any remaining elements.
			// This is a zero-length vector which does not allocate if `lower_bound` was exact.
			let mut collected = self.replace_with.by_ref().collect::<Vec<I::Item>>().into_iter();
			
			// Now we have an exact count.
			if collected.len() > 0
			{
				self.drain.move_tail(collected.len());
				let filled = self.drain.fill(&mut collected);
				debug_assert!(filled);
				debug_assert_eq!(collected.len(), 0);
			}
		}
		// Let `Drain::drop` move the tail back if necessary and restore `vec.len`.
	}
}
