// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A simple way to manage object pools configuration using Serde if desired.
/// Prefer the use of `Configuration` unless only working with object pools.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct ObjectPoolsConfiguration
{
	/// Performance enhancement to improve performance once a pool is created.
	pub prefault_object_pool_memory_to_improve_performance_at_create: bool,
	
	/// Performance enhancement to improve performance once a pool is opened.
	pub prefault_object_pool_memory_to_improve_performance_at_open: bool,
	
	/// Folder name for the pool sets for object pools.
	pub pools_sets_folder_name: String,
	
	/// Configurations.
	pub object_pool_configurations: HashMap<String, ObjectPoolConfiguration>
}

impl Default for ObjectPoolsConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			prefault_object_pool_memory_to_improve_performance_at_create: true,
			prefault_object_pool_memory_to_improve_performance_at_open: true,
			pools_sets_folder_name: "object".to_string(),
			object_pool_configurations: HashMap::new(),
		}
	}
}

impl ObjectPoolsConfiguration
{
	/// Opens a set of object pools.
	/// Do not use this method directly unless only using object pools.
	pub fn open(&self, pools_folder_path: &Path) -> HashMap<String, ObjectPool>
	{
		let object_pool_sets_folder_path = pools_folder_path.join(&self.pools_sets_folder_name);
		
		if unlikely(!object_pool_sets_folder_path.exists())
		{
			return HashMap::new()
		}
		
		assert!(object_pool_sets_folder_path.is_dir(), "object_pool_sets_folder_path '{:?}' is not a folder", object_pool_sets_folder_path);
		
		ObjectPool::set_prefault_at_create(self.prefault_object_pool_memory_to_improve_performance_at_create);
		ObjectPool::set_prefault_at_open(self.prefault_object_pool_memory_to_improve_performance_at_open);
		
		self.object_pool_configurations.iter().map(|(file_name, object_pool_configuration)| (file_name.to_string(), object_pool_configuration.open_or_create(&object_pool_sets_folder_path, file_name)) ).collect()
	}
}
