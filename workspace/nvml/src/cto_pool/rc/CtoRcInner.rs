// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) struct CtoRcInner<Value: CtoSafe>
{
	strong_counter: CtoRcCounter,
	weak_counter: CtoRcCounter,
	cto_pool_arc: CtoPoolArc,
	value: Value,
}

impl<Value: CtoSafe> CtoSafe for CtoRcInner<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.replace(&mut self.cto_pool_arc);
		self.value.cto_pool_opened(cto_pool_arc)
	}
}

impl<Value: CtoSafe> Deref for CtoRcInner<Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.value
	}
}

impl<Value: CtoSafe> DerefMut for CtoRcInner<Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.value
	}
}

impl<Value: CtoSafe> CtoRcInner<Value>
{
	#[inline(always)]
	pub(crate) fn strong_count(&self) -> usize
	{
		self.strong_counter.count()
	}
	
	#[inline(always)]
	pub(crate) fn strong_count_increment(&self)
	{
		self.strong_counter.increment();
	}
	
	#[inline]
	pub(crate) fn strong_count_decrement(&self)
	{
		self.strong_counter.decrement();
	}
	
	#[inline(always)]
	pub(crate) fn weak_count(&self) -> usize
	{
		self.weak_counter.count()
	}
	
	#[inline(always)]
	pub(crate) fn weak_count_increment(&self)
	{
		self.weak_counter.increment();
	}
	
	#[inline(always)]
	pub(crate) fn weak_count_decrement(&self)
	{
		self.weak_counter.decrement()
	}
	
	#[inline(always)]
	pub(crate) fn is_unique(&self) -> bool
	{
		self.strong_count() == 1 && self.weak_count() == 0
	}
}
