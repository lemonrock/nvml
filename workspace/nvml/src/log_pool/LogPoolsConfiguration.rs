// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A simple way to manage log pools configuration using Serde if desired.
/// Prefer the use of `Configuration` unless only working with block pools.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct LogPoolsConfiguration
{
	/// Folder name for the pool sets for log pools.
	pool_sets_folder_name: String,
	
	/// Configurations.
	log_pool_configurations: HashMap<String, LogPoolConfiguration>
}

impl Default for LogPoolsConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			pool_sets_folder_name: "log".to_string(),
			log_pool_configurations: HashMap::new(),
		}
	}
}

impl LogPoolsConfiguration
{
	/// Opens a set of object pools.
	/// Do not use this method directly unless only using object pools.
	pub fn open(&self, pools_folder_path: &Path) -> HashMap<String, LogPool>
	{
		let log_pool_sets_folder_path = pools_folder_path.join(&self.pool_sets_folder_name);
		
		if unlikely(!log_pool_sets_folder_path.exists())
		{
			return HashMap::new()
		}
		
		assert!(log_pool_sets_folder_path.is_dir(), "log_pool_sets_folder_path '{:?}' is not a folder", log_pool_sets_folder_path);
		
		self.log_pool_configurations.iter().map(|(file_name, log_pool_configuration)| (file_name.to_string(), log_pool_configuration.open_or_create(&log_pool_sets_folder_path, file_name)) ).collect()
	}
}
