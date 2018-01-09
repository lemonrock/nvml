// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// An iterator produced by calling `drain_filter` on CtoVec.
#[derive(Debug)]
pub struct CtoVecDrainFilter<'a, T: 'a + CtoSafe, F>
	where F: FnMut(&mut T) -> bool
{
	vec: &'a mut CtoVec<T>,
	idx: usize,
	del: usize,
	old_len: usize,
	pred: F,
}

impl<'a, T: CtoSafe, F> Iterator for CtoVecDrainFilter<'a, T, F>
	where F: FnMut(&mut T) -> bool,
{
	type Item = T;
	
	fn next(&mut self) -> Option<T>
	{
		unsafe
		{
			while self.idx != self.old_len
			{
				let i = self.idx;
				self.idx += 1;
				let v = slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len);
				if (self.pred)(&mut v[i])
				{
					self.del += 1;
					return Some(read(&v[i]));
				}
				else if self.del > 0
				{
					let del = self.del;
					let src: *const T = &v[i];
					let dst: *mut T = &mut v[i - del];
					// This is safe because self.vec has length 0
					// thus its elements will not have Drop::drop
					// called on them in the event of a panic.
					copy_nonoverlapping(src, dst, 1);
				}
			}
			None
		}
	}
	
	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.old_len - self.idx))
	}
}

impl<'a, T: CtoSafe, F> Drop for CtoVecDrainFilter<'a, T, F>
	where F: FnMut(&mut T) -> bool
{
	#[inline(always)]
	fn drop(&mut self)
	{
		for _ in self.by_ref()
		{
		}
		
		unsafe
		{
			self.vec.set_len(self.old_len - self.del);
		}
	}
}
