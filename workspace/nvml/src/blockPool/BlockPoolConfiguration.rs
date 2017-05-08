// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct BlockPoolConfiguration
{
	#[serde(default = "BlockPoolConfiguration::default_permissions")] permissions: mode_t,
	blockSize: usize,
}

impl BlockPoolConfiguration
{
	#[inline(always)]
	fn default_permissions() -> mode_t
	{
		Configuration::DefaultPermissionsForPoolSets
	}
	
	pub fn open(&self, blockPoolSetsFolderPath: &Path, fileName: &str) -> BlockPool
	{
		debug_assert!(self.blockSize != 0, "blockSize should not be zero");
		
		let poolSetFilePath = blockPoolSetsFolderPath.join(fileName);
		
		assert!(poolSetFilePath.exists(), "poolSetFilePath '{:?}' does not exist", poolSetFilePath);
		assert!(poolSetFilePath.is_file(), "poolSetFilePath '{:?}' is not a file", poolSetFilePath);
		
		BlockPool::open(&poolSetFilePath, Some(self.blockSize)).expect("Could not open BlockPool")
	}
}
