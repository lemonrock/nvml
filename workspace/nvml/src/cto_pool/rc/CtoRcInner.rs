// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// #[repr(C)] is required otherwise `from_raw_value_pointer()` will be very broken indeed.
#[repr(C)]
pub(crate) struct CtoRcInner<Value: CtoSafe>
{
	// Field order matters. `value: Value` must be first otherwise `from_raw_value_pointer()` will be very broken indeed.
	value: Value,
	strong_counter: CtoRcCounter,
	weak_counter: CtoRcCounter,
	cto_pool_arc: CtoPoolArc,
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
	fn common_initialization(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.write(&mut self.cto_pool_arc);
	}
	
	#[inline(always)]
	fn allocated<InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&mut self, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<(), InitializationError>
	{
		unsafe { write(&mut self.strong_counter, CtoRcCounter::default()) };
		
		unsafe { write(&mut self.weak_counter, CtoRcCounter::default()) };
		
		self.common_initialization(cto_pool_arc);
		
		initializer(&mut self.value, cto_pool_arc)
	}
	
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.common_initialization(cto_pool_arc);
		
		self.value.cto_pool_opened(cto_pool_arc)
	}
	
	#[inline(always)]
	fn into_raw_value_pointer(&mut self) -> *mut Value
	{
		self.deref_mut()
	}
	
	#[inline(always)]
	fn from_raw_value_pointer(raw_value_pointer: *mut Value) -> *mut Self
	{
		// Works because Value is the first field and we use #[repr(C)]
		raw_value_pointer as *mut Self
	}
	
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
