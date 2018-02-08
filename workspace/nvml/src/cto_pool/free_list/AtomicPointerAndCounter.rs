// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


struct AtomicPointerAndCounter<T>(AtomicU64Pair, PhantomData<T>);

impl<T> Default for AtomicPointerAndCounter<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		let pointer_and_counter: PointerAndCounter<T> = PointerAndCounter::default();
		AtomicPointerAndCounter(AtomicU64Pair(UnsafeCell::new(pointer_and_counter.get_u64_pair())), PhantomData)
	}
}

impl<T> CtoSafe for AtomicPointerAndCounter<T>
{
	#[inline(always)]
	default fn cto_pool_opened(&mut self, _cto_pool_arc: &CtoPoolArc)
	{
	}
}

impl<T: CtoSafe> CtoSafe for AtomicPointerAndCounter<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let pointer = self.get_pointer();
		if pointer.is_not_null()
		{
			unsafe { &mut * pointer }.cto_pool_opened(cto_pool_arc)
		}
	}
}

impl<T> AtomicPointerAndCounter<T>
{
	// Pointers are limited to 2^48 - 1
	const ImpossiblePointer: u64 = !0;
	
	const ImpossibleCounter: u64 = !0;
	
	const ImpossibleValue: (u64, u64) = (Self::ImpossiblePointer, Self::ImpossibleCounter);
	
	#[inline(always)]
	fn get_pointer(&self) -> *mut T
	{
		self.get_pointer_and_counter().get_pointer()
	}
	
	#[inline(always)]
	fn get_pointer_and_counter(&self) -> PointerAndCounter<T>
	{
		PointerAndCounter::from_u64_pair(self.get())
	}
	
	// On x86_64, this operation is always strong.
	#[inline(always)]
	fn compare_and_swap_weak(&self, was: &mut PointerAndCounter<T>, new: PointerAndCounter<T>) -> bool
	{
		match self.0.compare_and_swap_strong(was.get_u64_pair(), new.get_u64_pair())
		{
			(true, _) => true,
			(false, (pointer, counter)) =>
				{
					was.set_pointer(pointer as *mut T);
					was.set_counter(counter);
					false
				}
		}
	}
	
	#[inline(always)]
	fn get(&self) -> (u64, u64)
	{
		self.0.simulate_load(Self::ImpossibleValue)
	}
}
