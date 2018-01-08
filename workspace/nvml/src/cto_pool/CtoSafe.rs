// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A marker trait.
/// Exists to prevent regular Rust types such as Vec<T> and other stack-allocated code from being passed to a CTO pool.
pub trait CtoSafe: Sized
{
	/// Used internally when opening persistent memory pools for the first time.
	/// Exists for the convenience of objects with fields that should not be persistent, eg Mutexes, RwLocks and CondVars.
	#[inline(always)]
	fn reinitialize(&mut self, _cto_pool_inner: &Arc<CtoPoolInner>)
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

impl<T: CtoSafe> CtoSafe for Option<T>
{
}
