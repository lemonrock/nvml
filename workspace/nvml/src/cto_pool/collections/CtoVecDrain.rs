// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A draining iterator for `Vec<T>`.
pub struct CtoVecDrain<'a, T: 'a + CtoSafe>
{
	/// Index of tail to preserve
	tail_start: usize,
	
	/// Length of tail
	tail_len: usize,
	
	/// Current remaining range to remove
	iter: slice::Iter<'a, T>,
	
	vec: NonNull<CtoVec<T>>,
}

impl<'a, T: 'a + CtoSafe + Debug> Debug for CtoVecDrain<'a, T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.debug_tuple("CtoVecDrain").field(&self.iter.as_slice()).finish()
	}
}

unsafe impl<'a, T: CtoSafe + Sync> Sync for CtoVecDrain<'a, T>
{
}

unsafe impl<'a, T: CtoSafe + Send> Send for CtoVecDrain<'a, T>
{
}

impl<'a, T: CtoSafe> Iterator for CtoVecDrain<'a, T>
{
	type Item = T;
	
	#[inline(always)]
	fn next(&mut self) -> Option<T>
	{
		self.iter.next().map(|elt| unsafe { read(elt as *const _) })
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		self.iter.size_hint()
	}
}

impl<'a, T: CtoSafe> DoubleEndedIterator for CtoVecDrain<'a, T>
{
	#[inline(always)]
	fn next_back(&mut self) -> Option<T>
	{
		self.iter.next_back().map(|elt| unsafe { read(elt as *const _) })
	}
}

impl<'a, T: CtoSafe> Drop for CtoVecDrain<'a, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// exhaust self first
		while let Some(_) = self.next()
		{
		}
		
		if self.tail_len > 0
		{
			unsafe
			{
				let source_vec = self.vec.as_mut();
				// memmove back untouched tail, update to new length
				let start = source_vec.len();
				let tail = self.tail_start;
				let src = source_vec.as_ptr().offset(tail as isize);
				let dst = source_vec.as_mut_ptr().offset(start as isize);
				copy(src, dst, self.tail_len);
				source_vec.set_len(start + self.tail_len);
			}
		}
	}
}

impl<'a, T: CtoSafe> ExactSizeIterator for CtoVecDrain<'a, T>
{
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.iter.is_empty()
	}
}

impl<'a, T: CtoSafe> FusedIterator for CtoVecDrain<'a, T>
{
}

/// Private helper methods for `Splice::drop`
impl<'a, T: CtoSafe> CtoVecDrain<'a, T>
{
	/// The range from `self.vec.len` to `self.tail_start` contains elements
	/// that have been moved out.
	/// Fill that range as much as possible with new elements from the `replace_with` iterator.
	/// Return whether we filled the entire range. (`replace_with.next()` didn’t return `None`.)
	unsafe fn fill<I: Iterator<Item=T>>(&mut self, replace_with: &mut I) -> bool
	{
		let vec = self.vec.as_mut();
		let range_start = vec.len;
		let range_end = self.tail_start;
		let range_slice = from_raw_parts_mut(vec.as_mut_ptr().offset(range_start as isize), range_end - range_start);
		
		for place in range_slice
		{
			if let Some(new_item) = replace_with.next()
			{
				write(place, new_item);
				vec.len += 1;
			}
			else
			{
				return false
			}
		}
		true
	}
	
	/// Make room for inserting more elements before the tail.
	unsafe fn move_tail(&mut self, extra_capacity: usize)
	{
		let vec = self.vec.as_mut();
		let used_capacity = self.tail_start + self.tail_len;
		vec.buf.reserve(used_capacity, extra_capacity);
		
		let new_tail_start = self.tail_start + extra_capacity;
		let src = vec.as_ptr().offset(self.tail_start as isize);
		let dst = vec.as_mut_ptr().offset(new_tail_start as isize);
		copy(src, dst, self.tail_len);
		self.tail_start = new_tail_start;
	}
}
