// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


trait SpecExtend<T, I>
{
	fn spec_extend(&mut self, iterator: I);
}

impl<T: CtoSafe, I: Iterator<Item=T>> SpecExtend<T, I> for CtoVec<T>
{
	#[inline(always)]
	default fn spec_extend(&mut self, iterator: I)
	{
		self.extend_desugared(iterator)
	}
}

impl<T: CtoSafe, I: TrustedLen<Item=T>> SpecExtend<T, I> for CtoVec<T>
{
	#[inline(always)]
	default fn spec_extend(&mut self, iterator: I)
	{
		let (low, high) = iterator.size_hint();
		if let Some(high_value) = high
		{
			debug_assert_eq!(low, high_value, "TrustedLen iterator's size hint is not exact: {:?}", (low, high));
		}
		
		if let Some(additional) = high
		{
			self.reserve(additional);
			
			unsafe
			{
				let mut ptr = self.as_mut_ptr().offset(self.len() as isize);
				let mut local_len = SetLenOnDrop::new(&mut self.len);
				for element in iterator
				{
					write(ptr, element);
					ptr = ptr.offset(1);
					
					// NB can't overflow since we would have had to alloc the address space.
					local_len.increment_len(1);
				}
			}
		}
		else
		{
			self.extend_desugared(iterator)
		}
	}
}

impl<'a, T: 'a + CtoSafe + Clone, I: Iterator<Item=&'a T>> SpecExtend<&'a T, I> for CtoVec<T>
{
	#[inline(always)]
	default fn spec_extend(&mut self, iterator: I)
	{
		self.spec_extend(iterator.cloned())
	}
}

impl<'a, T: 'a + CtoSafe + Copy> SpecExtend<&'a T, slice::Iter<'a, T>> for CtoVec<T>
{
	#[inline(always)]
	fn spec_extend(&mut self, iterator: slice::Iter<'a, T>)
	{
		let slice = iterator.as_slice();
		self.reserve(slice.len());
		unsafe
		{
			let len = self.len();
			self.set_len(len + slice.len());
			self.get_unchecked_mut(len..).copy_from_slice(slice);
		}
	}
}

impl<T: CtoSafe> SpecExtend<T, CtoVecIntoIter<T>> for CtoVec<T>
{
	#[inline(always)]
	fn spec_extend(&mut self, mut iterator: CtoVecIntoIter<T>)
	{
		unsafe
		{
			self.append_elements(iterator.as_slice() as _);
		}
		iterator.ptr = iterator.end;
	}
}
