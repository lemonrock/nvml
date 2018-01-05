// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Trait to allow objects containing synchronisation primitives (Mutex, RwLock, CondVar) to be zeroed.
pub trait Zero: Sized
{
	/// # Arguments
	/// - self is always non-null
	/// - object_pool is always non-null
	#[inline(always)]
	unsafe fn zero(self, object_pool: *mut PMEMobjpool);
}

macro_rules! zero_guard
{
	($self: ident, $objectPool: ident) =>
	{
		{
			debug_assert!(!$self.is_null(), "self is null");
			debug_assert!(!$objectPool.is_null(), "objectPool is null");
		}
	}
}

impl Zero for *mut PMEMmutex
{
	#[inline(always)]
	unsafe fn zero(self, object_pool: *mut PMEMobjpool)
	{
		zero_guard!(self, object_pool);
		pmemobj_mutex_zero(object_pool, self)
	}
}

impl Zero for *mut PMEMrwlock
{
	#[inline(always)]
	unsafe fn zero(self, object_pool: *mut PMEMobjpool)
	{
		zero_guard!(self, object_pool);
		pmemobj_rwlock_zero(object_pool, self)
	}
}

impl Zero for *mut PMEMcond
{
	#[inline(always)]
	unsafe fn zero(self, object_pool: *mut PMEMobjpool)
	{
		zero_guard!(self, object_pool);
		pmemobj_cond_zero(object_pool, self)
	}
}
