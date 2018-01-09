// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Structs that are safe to store in persistent memory.
pub trait CtoSafe: Sized
{
	#[doc(hidden)]
	#[inline(always)]
	fn cto_pool_opened(&mut self, _cto_pool_alloc_guard_reference: &CtoPoolAllocGuardReference)
	{
	}
}

impl CtoSafe for u8
{
}

impl CtoSafe for i8
{
}

impl CtoSafe for u16
{
}

impl CtoSafe for i16
{
}

impl CtoSafe for u32
{
}

impl CtoSafe for i32
{
}

impl CtoSafe for u64
{
}

impl CtoSafe for i64
{
}

impl CtoSafe for usize
{
}

impl CtoSafe for isize
{
}

impl CtoSafe for f32
{
}

impl CtoSafe for f64
{
}

impl CtoSafe for bool
{
}

impl<Value: CtoSafe> CtoSafe for Option<Value>
{
}
