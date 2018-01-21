// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// An iterator that moves out of a vector.
pub struct CtoVecIntoIter<T: CtoSafe>
{
	buf: NonNull<T>,
	cap: usize,
	ptr: *const T,
	end: *const T,
	alloc: CtoPoolAlloc,
}

impl<T: CtoSafe + Debug> Debug for CtoVecIntoIter<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.debug_tuple("CtoVecIntoIter").field(&self.as_slice()).finish()
	}
}

impl<T: CtoSafe> CtoVecIntoIter<T>
{
	/// Returns the remaining items of this iterator as a slice.
	///
	/// # Examples
	///
	/// ```
	/// let vec = vec!['a', 'b', 'c'];
	/// let mut into_iter = vec.into_iter();
	/// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
	/// let _ = into_iter.next().unwrap();
	/// assert_eq!(into_iter.as_slice(), &['b', 'c']);
	/// ```
	#[inline(always)]
	pub fn as_slice(&self) -> &[T]
	{
		unsafe
		{
			from_raw_parts(self.ptr, self.len())
		}
	}
	
	/// Returns the remaining items of this iterator as a mutable slice.
	///
	/// # Examples
	///
	/// ```
	/// let vec = vec!['a', 'b', 'c'];
	/// let mut into_iter = vec.into_iter();
	/// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
	/// into_iter.as_mut_slice()[2] = 'z';
	/// assert_eq!(into_iter.next().unwrap(), 'a');
	/// assert_eq!(into_iter.next().unwrap(), 'b');
	/// assert_eq!(into_iter.next().unwrap(), 'z');
	/// ```
	#[inline(always)]
	pub fn as_mut_slice(&mut self) -> &mut [T]
	{
		unsafe
		{
			from_raw_parts_mut(self.ptr as *mut T, self.len())
		}
	}
}

unsafe impl<T: CtoSafe + Send> Send for CtoVecIntoIter<T>
{
}

unsafe impl<T: CtoSafe + Sync> Sync for CtoVecIntoIter<T>
{
}

impl<T: CtoSafe> Iterator for CtoVecIntoIter<T>
{
	type Item = T;
	
	#[inline(always)]
	fn next(&mut self) -> Option<T>
	{
		unsafe
		{
			if self.ptr as *const _ == self.end
			{
				None
			}
			else
			{
				if size_of::<T>() == 0
				{
					// Purposefully don't use 'ptr.offset' because for vectors with 0-size elements this would return the same pointer.
					self.ptr = arith_offset(self.ptr as *const i8, 1) as *mut T;
					
					// Use a non-null pointer value (self.ptr might be null because of wrapping).
					Some(read(1 as *mut T))
				}
				else
				{
					let old = self.ptr;
					self.ptr = self.ptr.offset(1);
					
					Some(read(old))
				}
			}
		}
	}
	
	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>)
	{
		let exact = match self.ptr.offset_to(self.end)
		{
			Some(x) => x as usize,
			None => (self.end as usize).wrapping_sub(self.ptr as usize),
		};
		(exact, Some(exact))
	}
	
	#[inline(always)]
	fn count(self) -> usize
	{
		self.len()
	}
}

impl<T: CtoSafe> DoubleEndedIterator for CtoVecIntoIter<T>
{
	#[inline(always)]
	fn next_back(&mut self) -> Option<T>
	{
		unsafe
		{
			if self.end == self.ptr
			{
				None
			}
			else
			{
				if size_of::<T>() == 0
				{
					// Purposefully don't use 'ptr.offset' because for vectors with 0-size elements this would return the same pointer.
					self.end = arith_offset(self.end as *const i8, -1) as *mut T;
					
					// Use a non-null pointer value (self.end might be null because of wrapping).
					Some(read(1 as *mut T))
				}
				else
				{
					self.end = self.end.offset(-1);
					
					Some(read(self.end))
				}
			}
		}
	}
}

impl<T: CtoSafe> ExactSizeIterator for CtoVecIntoIter<T>
{
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.ptr == self.end
	}
}

impl<T: CtoSafe> FusedIterator for CtoVecIntoIter<T>
{
}

unsafe impl<T: CtoSafe> TrustedLen for CtoVecIntoIter<T>
{
}

impl<T: CtoSafe> Drop for CtoVecIntoIter<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// destroy the remaining elements
		for _x in self.by_ref()
		{
		}
		
		// RawVec handles deallocation
		let _ = unsafe { RawVec::from_raw_parts_in(self.buf.as_ptr(), self.cap, self.alloc.clone()) };
	}
}
