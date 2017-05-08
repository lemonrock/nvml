// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Enabling pre-faulting reduces the impact of page faults, but makes start-up much slower
pub fn initialise(preFaultPersistentObjectPoolMemoryToImprovePerformance: bool)
{
	ObjectPool::setPrefaultAtCreate(preFaultPersistentObjectPoolMemoryToImprovePerformance);
	ObjectPool::setPrefaultAtOpen(preFaultPersistentObjectPoolMemoryToImprovePerformance);
}
