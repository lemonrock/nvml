// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// CTO pool equivalent to a Rust Vec.
pub struct CtoVec<T: CtoSafe>
{
	buf: RawVec<T, CtoPool>,
	len: usize,
}

impl<T: CtoSafe> CtoVec<T>
{
	/// Constructs a new, empty `CtoVec<T, Root>`.
	#[inline(always)]
	pub fn new(cto_pool: &CtoPool) -> CtoVec<T>
	{
		Self
		{
			buf: RawVec::new_in(cto_pool.clone()),
			len: 0,
		}
	}
	
	/// Constructs a new, empty `CtoVec<T>` with the specified capacity.
	#[inline(always)]
	pub fn with_capacity(capacity: usize, cto_pool: &CtoPool) -> CtoVec<T>
	{
		Self
		{
			buf: RawVec::with_capacity_in(capacity, cto_pool.clone()),
			len: 0,
		}
	}
}
