// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug)]
struct CtoRcInner<T: CtoSafe>
{
	strong_counter: CtoRcCounter,
	weak_counter: CtoRcCounter,
	cto_pool_inner: Arc<CtoPoolInner>,
	value: T,
}

impl<T: CtoSafe> CtoSafe for CtoRcInner<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.cto_pool_inner = cto_pool_inner.clone();
	}
}

impl<T: CtoSafe> Deref for CtoRcInner<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.value
	}
}

impl<T: CtoSafe> DerefMut for CtoRcInner<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.value
	}
}

impl<T: CtoSafe> CtoRcInner<T>
{
	#[inline(always)]
	fn free(&mut self)
	{
		CtoPoolInner::free(&self.cto_pool_inner, &mut self.value)
	}
	
	#[inline(always)]
	fn strong_count(&self) -> usize
	{
		self.strong_counter.count()
	}
	
	#[inline(always)]
	fn strong_count_increment(&self)
	{
		self.strong_counter.increment();
	}
	
	#[inline]
	fn strong_count_decrement(&self)
	{
		self.strong_counter.decrement();
	}
	
	#[inline(always)]
	fn weak_count(&self) -> usize
	{
		self.weak_counter.count()
	}
	
	#[inline(always)]
	fn weak_count_increment(&self)
	{
		self.weak_counter.increment();
	}
	
	#[inline(always)]
	fn weak_count_decrement(&self)
	{
		self.weak_counter.decrement()
	}
	
	#[inline(always)]
	fn is_unique(&self) -> bool
	{
		self.strong_count() == 1 && self.weak_count() == 0
	}
}
