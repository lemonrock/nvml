// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) struct CtoBoxInner<Value: CtoSafe>
{
	cto_pool_alloc_guard_reference: CtoPoolArc,
	value: Value,
}

impl<T: CtoSafe> CtoSafe for CtoBoxInner<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_alloc_guard_reference: &CtoPoolArc)
	{
		self.cto_pool_alloc_guard_reference = cto_pool_alloc_guard_reference.clone();
		self.value.cto_pool_opened(cto_pool_alloc_guard_reference);
	}
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
