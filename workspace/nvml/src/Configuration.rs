// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Configuration
{
	poolsFolderName: String,
	blockPools: BlockPoolsConfiguration,
	logPools: LogPoolsConfiguration,
	objectPools: ObjectPoolsConfiguration,
}

impl Default for Configuration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			poolsFolderName: "pools".to_string(),
			blockPools: Default::default(),
			logPools: Default::default(),
			objectPools: Default::default(),
		}
	}
}

impl Configuration
{
	pub const DefaultPermissionsForPoolSets: mode_t = 0o600;
	
	pub fn open(&self, configurationFolderPath: &Path) -> Pools
	{
		let poolsFolderPath = configurationFolderPath.join(&self.poolsFolderName);
		
		if unlikely(!poolsFolderPath.exists())
		{
			return Default::default()
		}
		
		Pools
		{
			blockPools: self.blockPools.open(&poolsFolderPath),
			logPools: self.logPools.open(&poolsFolderPath),
			objectPools: self.objectPools.open(&poolsFolderPath),
		}
	}
}
