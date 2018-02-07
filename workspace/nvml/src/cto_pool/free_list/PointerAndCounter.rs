// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct PointerAndCounter<T>(*mut T, u64, PhantomData<T>);

impl<T> Clone for PointerAndCounter<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		PointerAndCounter::new(self.0, self.1)
	}
}

impl<T> Copy for PointerAndCounter<T>
{
}

impl<T> Default for PointerAndCounter<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::InitialValue
	}
}

impl<T> PointerAndCounter<T>
{
	const FirstCounter: u64 = 0;
	
	const InitialValue: Self = Self::from_pointer(null_mut::<T>());
	
	#[inline(always)]
	fn get_pointer(self) -> *mut T
	{
		self.0
	}
	
	#[inline(always)]
	fn set_pointer(&mut self, pointer: *mut T)
	{
		self.0 = pointer
	}
	
	#[inline(always)]
	fn get_counter(self) -> u64
	{
		self.1
	}
	
	#[inline(always)]
	fn get_incremented_counter(self) -> u64
	{
		self.get_counter() + 1
	}
	
	#[inline(always)]
	fn set_counter(&mut self, value: u64)
	{
		self.1 = value
	}
	
	#[inline(always)]
	const fn from_pointer(pointer: *mut T) -> Self
	{
		PointerAndCounter::new(pointer, Self::FirstCounter)
	}
	
	#[inline(always)]
	const fn new(pointer: *mut T, counter: u64) -> Self
	{
		PointerAndCounter(pointer, counter, PhantomData)
	}
	
	#[inline(always)]
	fn from_u64_pair(pair: (u64, u64)) -> Self
	{
		unsafe { transmute(pair) }
	}
	
	#[inline(always)]
	fn get_u64_pair(self) -> (u64, u64)
	{
		unsafe { transmute(self) }
	}
}
