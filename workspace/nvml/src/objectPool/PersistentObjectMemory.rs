// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PersistentObjectMemory
{
	/// Call .is_null() afterwards
	#[inline(always)]
	fn oid(self) -> PMEMoid;
	
	/// Can be NULL
	#[inline(always)]
	fn persistentObjectPool(self) -> *mut PMEMobjpool;
}

impl PersistentObjectMemory for *const c_void
{
	#[inline(always)]
	fn oid(self) -> PMEMoid
	{
		unsafe { pmemobj_oid(self) }
	}
	
	#[inline(always)]
	fn persistentObjectPool(self) -> *mut PMEMobjpool
	{
		unsafe { pmemobj_pool_by_ptr(self) }
	}
}
