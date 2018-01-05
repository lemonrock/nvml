// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents block pool configuration which can be persisted or deserialized using Serde.
/// Use `LogPoolsConfiguration` or `Configuration` to manage multiple pools.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct LogPoolConfiguration
{
	/// Permissions for this pool. 0o600 is a good option.
	pub permissions: mode_t,
	
	/// Pool size in bytes. Ought to be a power of 2.
	/// Must be at least `nvml_sys::PMEMLOG_MIN_POOL` (As of May 7th 2017, 1024 * 1024 * 2 bytes)
	pub pool_size: Option<usize>,
}

impl Default for LogPoolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			permissions: Configuration::DefaultPermissionsForPoolSets,
			pool_size: None,
		}
	}
}

impl LogPoolConfiguration
{
	/// Open or create (if necessary) a log pool.
	/// Do not use this method directly unless only using one log pool.
	pub fn open_or_create(&self, log_pool_sets_folder_path: &Path, file_name: &str) -> LogPool
	{
		let pool_set_file_path = log_pool_sets_folder_path.join(file_name);
		
		if likely(pool_set_file_path.exists())
		{
			assert!(pool_set_file_path.is_file(), "pool_set_file_path '{:?}' is not a file", pool_set_file_path);
			LogPool::open(&pool_set_file_path).expect("Could not open LogPool")
		}
		else
		{
			let pool_size = match self.pool_size
			{
				None => 0,
				Some(pool_size) =>
				{
					assert!(pool_size >= PMEMLOG_MIN_POOL, "pool_size '{}' is smaller than PMEMLOG_MIN_POOL '{}'", pool_size, PMEMLOG_MIN_POOL);
					pool_size
				},
			};
			LogPool::create(&pool_set_file_path, pool_size, self.permissions).expect("Could not create LogPool")
		}
	}
}
