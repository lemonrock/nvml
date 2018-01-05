// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A simple way to manage block pools configuration using Serde if desired.
/// Prefer the use of `Configuration` unless only working with block pools.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BlockPoolsConfiguration
{
	/// Folder name for the pool sets for block pools.
	pub pool_sets_folder_name: String,
	
	/// Configurations.
	pub block_pool_configurations: HashMap<String, BlockPoolConfiguration>
}

impl Default for BlockPoolsConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			pool_sets_folder_name: "block".to_string(),
			block_pool_configurations: HashMap::new(),
		}
	}
}

impl BlockPoolsConfiguration
{
	/// Opens a set of block pools.
	/// Do not use this method directly unless only using block pools.
	pub fn open(&self, pools_folder_path: &Path) -> HashMap<String, BlockPool>
	{
		let block_pool_sets_folder_path = pools_folder_path.join(&self.pool_sets_folder_name);
		
		if unlikely(!block_pool_sets_folder_path.exists())
		{
			return HashMap::new()
		}
		
		assert!(block_pool_sets_folder_path.is_dir(), "block_pool_sets_folder_path '{:?}' is not a folder", block_pool_sets_folder_path);
		
		self.block_pool_configurations.iter().map(|(file_name, block_pool_configuration)| (file_name.to_string(), block_pool_configuration.open_or_create(&block_pool_sets_folder_path, file_name)) ).collect()
	}
}
