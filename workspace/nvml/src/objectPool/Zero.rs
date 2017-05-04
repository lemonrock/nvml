// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Zero: Sized
{
	/// # Arguments
	/// - self is always non-null
	/// - objectPool is always non-null
	#[inline(always)]
	unsafe fn zero(self, objectPool: *mut PMEMobjpool);
}

macro_rules! zero
{
	($self: ident, $objectPool: ident, $functionNameFragment: ident) =>
	{
		interpolate_idents!
		{
			{
				debug_assert!(!$self.is_null(), "self is null");
				debug_assert!(!$objectPool.is_null(), "objectPool is null");
				
				[pmemobj_ $functionNameFragment _zero]($objectPool, $self)
			}
		}
	}
}

impl Zero for *mut PMEMmutex
{
	#[inline(always)]
	unsafe fn zero(self, objectPool: *mut PMEMobjpool)
	{
		zero!(self, objectPool, mutex);
	}
}

impl Zero for *mut PMEMrwlock
{
	#[inline(always)]
	unsafe fn zero(self, objectPool: *mut PMEMobjpool)
	{
		zero!(self, objectPool, rwlock);
	}
}

impl Zero for *mut PMEMcond
{
	#[inline(always)]
	unsafe fn zero(self, objectPool: *mut PMEMobjpool)
	{
		zero!(self, objectPool, cond);
	}
}
