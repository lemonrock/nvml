// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BlockPoolsConfiguration
{
	poolSetsFolderName: String,
	blockPoolConfigurations: HashMap<String, BlockPoolConfiguration>
}

impl Default for BlockPoolsConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			poolSetsFolderName: "block".to_string(),
			blockPoolConfigurations: HashMap::new(),
		}
	}
}

impl BlockPoolsConfiguration
{
	pub fn open(&self, poolsFolderPath: &Path) -> HashMap<String, BlockPool>
	{
		let blockPoolSetsFolderPath = poolsFolderPath.join(&self.poolSetsFolderName);
		
		if unlikely(!blockPoolSetsFolderPath.exists())
		{
			return HashMap::new()
		}
		
		assert!(blockPoolSetsFolderPath.is_dir(), "blockPoolSetsFolderPath '{:?}' is not a folder", blockPoolSetsFolderPath);
		
		self.blockPoolConfigurations.iter().map(|(fileName, blockPoolConfiguration)| (fileName.to_string(), blockPoolConfiguration.open(&blockPoolSetsFolderPath, fileName)) ).collect()
	}
}
