// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Set the length of the vec when the `SetLenOnDrop` value goes out of scope.
//
// The idea is: The length field in SetLenOnDrop is a local variable
// that the optimizer will see does not alias with any stores through the Vec's data
// pointer. This is a workaround for alias analysis issue #32155
struct SetLenOnDrop<'a>
{
	len: &'a mut usize,
	local_len: usize,
}

impl<'a> SetLenOnDrop<'a>
{
	#[inline(always)]
	fn new(len: &'a mut usize) -> Self
	{
		Self
		{
			local_len: *len,
			len
		}
	}
	
	#[inline(always)]
	fn increment_len(&mut self, increment: usize)
	{
		self.local_len += increment;
	}
}

impl<'a> Drop for SetLenOnDrop<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		*self.len = self.local_len;
	}
}
