// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct LogPoolConfiguration
{
	permissions: mode_t,
	poolSize: Option<usize>,
}

impl Default for LogPoolConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			permissions: Configuration::DefaultPermissionsForPoolSets,
			poolSize: None,
		}
	}
}

impl LogPoolConfiguration
{
	pub fn openOrCreate(&self, objectPoolSetsFolderPath: &Path, fileName: &str) -> LogPool
	{
		let poolSetFilePath = objectPoolSetsFolderPath.join(fileName);
		
		if likely(poolSetFilePath.exists())
		{
			assert!(poolSetFilePath.is_file(), "poolSetFilePath '{:?}' is not a file", poolSetFilePath);
			LogPool::open(&poolSetFilePath).expect("Could not open LogPool")
		}
		else
		{
			let poolSize = match self.poolSize
			{
				None => 0,
				Some(poolSize) =>
				{
					assert!(poolSize >= PMEMLOG_MIN_POOL, "poolSize '{}' is smaller than PMEMLOG_MIN_POOL '{}'", poolSize, PMEMLOG_MIN_POOL);
					poolSize
				},
			};
			LogPool::create(&poolSetFilePath, poolSize, self.permissions).expect("Could not create LogPool")
		}
	}
}
