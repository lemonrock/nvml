// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// #[repr(C)] is required otherwise `from_raw_value_pointer()` will be very broken indeed.
#[repr(C)]
pub(crate) struct CtoBoxInner<Value: CtoSafe>
{
	// Field order matters. `value: Value` must be first otherwise `from_raw_value_pointer()` will be very broken indeed.
	value: Value,
	cto_pool_arc: CtoPoolArc,
}

impl<Value: CtoSafe> Deref for CtoBoxInner<Value>
{
	type Target = Value;
	
	fn deref(&self) -> &Value
	{
		&self.value
	}
}

impl<Value: CtoSafe> DerefMut for CtoBoxInner<Value>
{
	fn deref_mut(&mut self) -> &mut Value
	{
		&mut self.value
	}
}

impl<Value: CtoSafe> CtoBoxInner<Value>
{
	#[inline(always)]
	fn common_initialization(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.write(&mut self.cto_pool_arc);
	}
	
	#[inline(always)]
	fn allocated<InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&mut self, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<(), InitializationError>
	{
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
}
