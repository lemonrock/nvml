// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Default)]
pub struct Pools
{
	blockPools: HashMap<String, BlockPool>,
	logPools: HashMap<String, LogPool>,
	objectPools: HashMap<String, ObjectPool>,
}

impl Pools
{
	#[inline(always)]
	pub fn getBlockPool(&self, poolName: &str) -> Option<BlockPool>
	{
		self.blockPools.get(poolName).map(|pool| pool.clone())
	}
	
	#[inline(always)]
	pub fn getLogPool(&self, poolName: &str) -> Option<LogPool>
	{
		self.logPools.get(poolName).map(|pool| pool.clone())
	}
	
	#[inline(always)]
	pub fn getObjectPool(&self, poolName: &str) -> Option<ObjectPool>
	{
		self.objectPools.get(poolName).map(|pool| pool.clone())
	}
}
