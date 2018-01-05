// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Allow an object to be initialized.
pub trait Initializable
{
	/// # Arguments
	/// - pointer_to_uninitialized_memory_to_use_for_fields is always non-null
	/// - object_pool is always non-null
	#[inline(always)]
	unsafe fn initialize(pointer_to_uninitialized_memory_to_use_for_fields: *mut Self, object_pool: *mut PMEMobjpool);
}
