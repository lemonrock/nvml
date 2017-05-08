// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct LogPoolsConfiguration
{
	poolSetsFolderName: String,
	logPoolConfigurations: HashMap<String, LogPoolConfiguration>
}

impl Default for LogPoolsConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			poolSetsFolderName: "log".to_string(),
			logPoolConfigurations: HashMap::new(),
		}
	}
}

impl LogPoolsConfiguration
{
	pub fn open(&self, poolsFolderPath: &Path) -> HashMap<String, LogPool>
	{
		let logPoolSetsFolderPath = poolsFolderPath.join(&self.poolSetsFolderName);
		
		if unlikely(!logPoolSetsFolderPath.exists())
		{
			return HashMap::new()
		}
		
		assert!(logPoolSetsFolderPath.is_dir(), "logPoolSetsFolderPath '{:?}' is not a folder", logPoolSetsFolderPath);
		
		self.logPoolConfigurations.iter().map(|(fileName, logPoolConfiguration)| (fileName.to_string(), logPoolConfiguration.open(&logPoolSetsFolderPath, fileName)) ).collect()
	}
}
