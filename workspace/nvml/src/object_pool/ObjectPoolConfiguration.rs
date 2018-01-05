// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents object pool configuration which can be persisted or deserialized using Serde.
/// Use `ObjectPoolsConfiguration` or `Configuration` to manage multiple pools.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct ObjectPoolConfiguration
{
	/// Permissions for this pool. 0o600 is a good option.
	pub permissions: mode_t,
	
	/// Pool size in bytes. Ought to be a power of 2 and a multiple of block_size.
	/// Must be at least `nvml_sys::PMEMOBJ_MIN_POOL` (As of May 7th 2017, 1024 * 1024 * 8)
	pub pool_size: Option<usize>,
	
	/// Name of layout.
	pub layout_name: Option<String>,
	
	/// skip expensive debug checks.
	pub skip_expensive_debug_checks: bool,
	
	/// transaction cache size in bytes (use `nvml_sys::TX_DEFAULT_RANGE_CACHE_SIZE` as a default).
	pub transaction_cache_size: u64,
	
	/// transaction cache threshold in bytes (use `nvml_sys::TX_DEFAULT_RANGE_CACHE_THRESHOLD` as a default).
	pub transaction_cache_threshold: u64,
}

impl Default for ObjectPoolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			pool_size: None,
			permissions: Configuration::DefaultPermissionsForPoolSets,
			layout_name: None,
			skip_expensive_debug_checks: false,
			transaction_cache_size: TX_DEFAULT_RANGE_CACHE_SIZE as u64,
			transaction_cache_threshold: TX_DEFAULT_RANGE_CACHE_THRESHOLD as u64,
		}
	}
}

impl ObjectPoolConfiguration
{
	/// Open or create (if necessary) an object pool.
	/// Do not use this method directly unless only using one object pool.
	pub fn open_or_create(&self, object_pool_sets_folder_path: &Path, file_name: &str) -> ObjectPool
	{
		let layout_name = match self.layout_name
		{
			None => file_name,
			Some(ref layout_name) => layout_name,
		};
		
		assert!(layout_name.len() + 1 <= PMEMOBJ_MAX_LAYOUT, "layout_name '{}' can not be larger than PMEMOBJ_MAX_LAYOUT '{}' - 1", layout_name, PMEMOBJ_MAX_LAYOUT);
		
		let pool_set_file_path = object_pool_sets_folder_path.join(file_name);
		let layout_name = Some(layout_name);
		let object_pool = if likely(pool_set_file_path.exists())
		{
			assert!(pool_set_file_path.is_file(), "pool_set_file_path '{:?}' is not a file", pool_set_file_path);
			ObjectPool::open(&pool_set_file_path, layout_name).expect("Could not open ObjectPool")
		}
		else
		{
			let pool_size = match self.pool_size
			{
				None => 0,
				Some(pool_size) =>
				{
					assert!(pool_size >= PMEMOBJ_MIN_POOL, "pool_size '{}' is smaller than PMEMOBJ_MIN_POOL '{}'", pool_size, PMEMOBJ_MIN_POOL);
					pool_size
				},
			};
			ObjectPool::create(&pool_set_file_path, layout_name, pool_size, self.permissions).expect("Could not create ObjectPool")
		};
		
		object_pool.set_transaction_debug_skip_expensive_checks(self.skip_expensive_debug_checks);
		object_pool.set_transaction_cache_size_and_threshold(self.transaction_cache_size, self.transaction_cache_threshold);
		object_pool
	}
}
