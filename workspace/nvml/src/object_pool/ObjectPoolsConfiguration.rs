// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct ObjectPoolsConfiguration
{
	preFaultPersistentObjectPoolMemoryAtCreateToImprovePerformance: bool,
	preFaultPersistentObjectPoolMemoryAtOpenToImprovePerformance: bool,
	poolSetsFolderName: String,
	objectPoolConfigurations: HashMap<String, ObjectPoolConfiguration>
}

impl Default for ObjectPoolsConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			preFaultPersistentObjectPoolMemoryAtCreateToImprovePerformance: true,
			preFaultPersistentObjectPoolMemoryAtOpenToImprovePerformance: true,
			poolSetsFolderName: "object".to_string(),
			objectPoolConfigurations: HashMap::new(),
		}
	}
}

impl ObjectPoolsConfiguration
{
	pub fn open(&self, poolsFolderPath: &Path) -> HashMap<String, ObjectPool>
	{
		let objectPoolSetsFolderPath = poolsFolderPath.join(&self.poolSetsFolderName);
		
		if unlikely(!objectPoolSetsFolderPath.exists())
		{
			return HashMap::new()
		}
		
		assert!(objectPoolSetsFolderPath.is_dir(), "objectPoolSetsFolderPath '{:?}' is not a folder", objectPoolSetsFolderPath);
		
		ObjectPool::setPrefaultAtCreate(self.preFaultPersistentObjectPoolMemoryAtCreateToImprovePerformance);
		ObjectPool::setPrefaultAtOpen(self.preFaultPersistentObjectPoolMemoryAtOpenToImprovePerformance);
		
		self.objectPoolConfigurations.iter().map(|(fileName, objectPoolConfiguration)| (fileName.to_string(), objectPoolConfiguration.openOrCreate(&objectPoolSetsFolderPath, fileName)) ).collect()
	}
}
