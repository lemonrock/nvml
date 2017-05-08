// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct ObjectPoolConfiguration
{
	layoutName: String,
	skipExpensiveDebugChecks: bool,
	transactionCacheSize: u64,
	transactionCacheThreshold: u64,
}

impl Default for ObjectPoolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			layoutName: "ObjectPool".to_string(),
			skipExpensiveDebugChecks: false,
			transactionCacheSize: Self::TX_DEFAULT_RANGE_CACHE_SIZE,
			transactionCacheThreshold: Self::TX_DEFAULT_RANGE_CACHE_THRESHOLD,
		}
	}
}

impl ObjectPoolConfiguration
{
	// Accurate as of May 7th 2017
	// lib/nvml/src/libpmemobj/tx.c
	const TX_DEFAULT_RANGE_CACHE_SIZE: u64 = 1 << 15;
	
	// Accurate as of May 7th 2017
	// lib/nvml/src/libpmemobj/tx.c
	const TX_DEFAULT_RANGE_CACHE_THRESHOLD: u64 = 1 << 12;
	
	pub fn typical(&self, layoutName: String) -> ObjectPoolConfiguration
	{
		Self
		{
			layoutName: layoutName,
			.. Default::default()
		}
	}
	
	pub fn open(&self, poolSetFolderPath: &Path) -> ObjectPool
	{
		let poolSetFilePath = poolSetFolderPath.join(&self.layoutName);
		
		assert!(poolSetFilePath.exists(), "poolSetFilePath '{:?}' does not exist", poolSetFilePath);
		assert!(poolSetFilePath.is_file(), "poolSetFilePath '{:?}' is not a file", poolSetFilePath);
		
		let objectPool = ObjectPool::open(&poolSetFilePath, Some(&self.layoutName)).expect("Could not open poolSetFilePath");
		objectPool.setTransactionDebugSkipExpensiveChecks(self.skipExpensiveDebugChecks);
		objectPool.setTransactionCacheSize(self.transactionCacheSize);
		objectPool.setTransactionCacheSize(self.transactionCacheThreshold);
		objectPool
	}
}
