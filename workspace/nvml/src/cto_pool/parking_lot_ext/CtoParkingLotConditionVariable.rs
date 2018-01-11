// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Wrapper around a parking lot `Condvar` that is safe to store in persistent memory.
pub struct CtoParkingLotConditionVariable(Condvar);

impl Deref for CtoParkingLotConditionVariable
{
	type Target = Condvar;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl CtoSafe for CtoParkingLotConditionVariable
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, _cto_pool_arc: &CtoPoolArc)
	{
		unsafe { write(&mut self.0, Condvar::new()) };
	}
}

impl CtoParkingLotConditionVariable
{
	/// Create a new instance on the Stack (or inside a persistent memory object).
	#[inline(always)]
	pub fn new() -> Self
	{
		CtoParkingLotConditionVariable(Condvar::new())
	}
	
	/// Access the condvar.
	#[inline(always)]
	pub fn condvar(&self) -> &Condvar
	{
		self.deref()
	}
}

