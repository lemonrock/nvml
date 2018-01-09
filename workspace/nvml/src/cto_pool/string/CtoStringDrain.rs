// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A draining iterator for `String`.
pub struct CtoStringDrain<'a>
{
	/// Will be used as &'a mut String in the destructor
	string: *mut CtoString,
	
	/// Start of part to remove
	start: usize,
	
	/// End of part to remove
	end: usize,
	
	/// Current remaining range to remove
	iter: Chars<'a>,
}

impl<'a> Debug for CtoStringDrain<'a>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.pad("Drain { .. }")
	}
}

unsafe impl<'a> Sync for CtoStringDrain<'a>
{
}

unsafe impl<'a> Send for CtoStringDrain<'a>
{
}

impl<'a> Drop for CtoStringDrain<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			// Use Vec::drain. "Reaffirm" the bounds checks to avoid panic code being inserted again.
			let self_vec = (*self.string).as_mut_vec();
			if self.start <= self.end && self.end <= self_vec.len()
			{
				self_vec.drain(self.start..self.end);
			}
		}
	}
}

impl<'a> Iterator for CtoStringDrain<'a>
{
	type Item = char;
	
	#[inline(always)]
	fn next(&mut self) -> Option<char>
	{
		self.iter.next()
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		self.iter.size_hint()
	}
}

impl<'a> DoubleEndedIterator for CtoStringDrain<'a>
{
	#[inline(always)]
	fn next_back(&mut self) -> Option<char>
	{
		self.iter.next_back()
	}
}

impl<'a> FusedIterator for CtoStringDrain<'a>
{
}
