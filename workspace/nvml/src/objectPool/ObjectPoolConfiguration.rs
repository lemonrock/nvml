// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct ObjectPoolConfiguration
{
	poolSize: Option<usize>,
	permissions: mode_t,
	layoutName: Option<String>,
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
			poolSize: None,
			permissions: Configuration::DefaultPermissionsForPoolSets,
			layoutName: None,
			skipExpensiveDebugChecks: false,
			transactionCacheSize: TX_DEFAULT_RANGE_CACHE_SIZE as u64,
			transactionCacheThreshold: TX_DEFAULT_RANGE_CACHE_THRESHOLD as u64,
		}
	}
}

impl ObjectPoolConfiguration
{
	pub fn openOrCreate(&self, objectPoolSetsFolderPath: &Path, fileName: &str) -> ObjectPool
	{
		let layoutName = match self.layoutName
		{
			None => fileName,
			Some(ref layoutName) => layoutName,
		};
		
		assert!(layoutName.len() + 1 <= PMEMOBJ_MAX_LAYOUT, "layoutName '{}' can not be larger than PMEMOBJ_MAX_LAYOUT '{}' - 1", layoutName, PMEMOBJ_MAX_LAYOUT);
		
		let pool_set_file_path = objectPoolSetsFolderPath.join(fileName);
		let layoutName = Some(layoutName);
		let objectPool = if likely(pool_set_file_path.exists())
		{
			assert!(pool_set_file_path.is_file(), "pool_set_file_path '{:?}' is not a file", pool_set_file_path);
			ObjectPool::open(&pool_set_file_path, layoutName).expect("Could not open ObjectPool")
		}
		else
		{
			let poolSize = match self.poolSize
			{
				None => 0,
				Some(poolSize) =>
				{
					assert!(poolSize >= PMEMOBJ_MIN_POOL, "poolSize '{}' is smaller than PMEMOBJ_MIN_POOL '{}'", poolSize, PMEMOBJ_MIN_POOL);
					poolSize
				},
			};
			ObjectPool::create(&pool_set_file_path, layoutName, poolSize, self.permissions).expect("Could not create ObjectPool")
		};
		
		objectPool.setTransactionDebugSkipExpensiveChecks(self.skipExpensiveDebugChecks);
		objectPool.setTransactionCacheSize(self.transactionCacheSize);
		objectPool.setTransactionCacheSize(self.transactionCacheThreshold);
		objectPool
	}
}
