// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BlockPoolConfiguration
{
	permissions: mode_t,
	poolSize: Option<usize>,
	blockSize: Option<usize>,
}

impl Default for BlockPoolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			permissions: Configuration::DefaultPermissionsForPoolSets,
			poolSize: None,
			blockSize: None,
		}
	}
}

impl BlockPoolConfiguration
{
	pub fn openOrCreate(&self, objectPoolSetsFolderPath: &Path, fileName: &str) -> BlockPool
	{
		let poolSetFilePath = objectPoolSetsFolderPath.join(fileName);
		
		if likely(poolSetFilePath.exists())
		{
			assert!(poolSetFilePath.is_file(), "poolSetFilePath '{:?}' is not a file", poolSetFilePath);
			BlockPool::open(&poolSetFilePath, self.blockSize).expect("Could not open BlockPool")
		}
		else
		{
			
			let blockSize = match self.blockSize
			{
				None => PMEMBLK_MIN_BLK,
				Some(blockSize) => min(blockSize, PMEMBLK_MIN_BLK)
			};
			
			let poolSize = match self.poolSize
			{
				None => 0,
				Some(poolSize) =>
				{
					assert!(poolSize >= PMEMOBJ_MIN_POOL, "poolSize '{}' is smaller than PMEMOBJ_MIN_POOL '{}'", poolSize, PMEMOBJ_MIN_POOL);
					poolSize
				},
			};
			BlockPool::create(&poolSetFilePath, blockSize, poolSize, self.permissions).expect("Could not create BlockPool")
		}
	}
}
