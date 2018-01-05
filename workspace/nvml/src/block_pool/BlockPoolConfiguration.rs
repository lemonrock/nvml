// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents block pool configuration which can be persisted or deserialized using Serde.
/// Use `BlockPoolsConfiguration` or `Configuration` to manage multiple pools.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BlockPoolConfiguration
{
	/// Permissions for this pool. 0o600 is a good option.
	pub permissions: mode_t,
	
	/// Pool size in bytes. Ought to be a power of 2 and a multiple of block_size.
	/// Must be at least PMEMBLK_MIN_POOL (As of May 7th 2017, 16MB + 4KB)
	pub pool_size: Option<usize>,
	
	/// Block size in bytes. Ought to be a power of 2.
	/// Rounded up to PMEMBLK_MIN_BLK. (As of May 7th 2017, 512 bytes).
	pub block_size: Option<usize>,
}

impl Default for BlockPoolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			permissions: Configuration::DefaultPermissionsForPoolSets,
			pool_size: None,
			block_size: None,
		}
	}
}

impl BlockPoolConfiguration
{
	/// Open or create (if necessary) a block pool.
	/// Do not use this method directly unless only using one block pool.
	pub fn open_or_create(&self, block_pool_sets_folder_path: &Path, file_name: &str) -> BlockPool
	{
		let pool_set_file_path = block_pool_sets_folder_path.join(file_name);
		
		if likely(pool_set_file_path.exists())
		{
			assert!(pool_set_file_path.is_file(), "pool_set_file_path '{:?}' is not a file", pool_set_file_path);
			BlockPool::open(&pool_set_file_path, self.block_size).expect("Could not open BlockPool")
		}
		else
		{
			
			let block_size = match self.block_size
			{
				None => PMEMBLK_MIN_BLK,
				Some(block_size) => min(block_size, PMEMBLK_MIN_BLK)
			};
			
			let pool_size = match self.pool_size
			{
				None => 0,
				Some(pool_size) =>
				{
					assert!(pool_size >= PMEMBLK_MIN_POOL, "pool_size '{}' is smaller than PMEMBLK_MIN_POOL '{}'", pool_size, PMEMBLK_MIN_POOL);
					pool_size
				},
			};
			BlockPool::create(&pool_set_file_path, block_size, pool_size, self.permissions).expect("Could not create BlockPool")
		}
	}
}
