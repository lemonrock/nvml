// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Copy)]
struct VariableLengthArray(PhantomData<u8>);

impl VariableLengthArray
{
	#[inline(always)]
	unsafe fn as_ptr(&self) -> *const u8
	{
		transmute(self)
	}
	
	#[inline(always)]
	unsafe fn as_mut_ptr(&mut self) -> *mut u8
	{
		transmute(self)
	}
	
	#[inline(always)]
	unsafe fn as_slice(&self, length: usize) -> &[u8]
	{
		from_raw_parts(self.as_ptr(), length)
	}
	
	#[inline(always)]
	unsafe fn as_mut_slice(&mut self, length: usize) -> &mut [u8]
	{
		from_raw_parts_mut(self.as_mut_ptr(), length)
	}
}

impl Debug for VariableLengthArray
{
	#[inline(always)]
	fn fmt(&self, fmt: &mut Formatter) -> fmt::Result
	{
		fmt.write_str("VariableLengthArray")
	}
}

impl Clone for VariableLengthArray
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::default()
	}
}
