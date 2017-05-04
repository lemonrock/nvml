// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Initializable
{
	/// # Arguments
	/// - pointerToUninitializedMemoryToUseForFields is always non-null
	/// - objectPool is always non-null
	#[inline(always)]
	unsafe fn initialize(pointerToUninitializedMemoryToUseForFields: *mut Self, objectPool: *mut PMEMobjpool);
}
