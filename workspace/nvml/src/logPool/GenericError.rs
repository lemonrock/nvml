// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GenericError
{
	osErrorNumber: i32,
	lastErrorMessageOnThisThread: Result<String, LastErrorMessageOnThisThreadIsInvalidError>,
}

impl Display for GenericError
{
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>
	{
		let lastErrorMessageOnThisThread = self.lastErrorMessageOnThisThread.as_ref();
		if likely(lastErrorMessageOnThisThread.is_ok())
		{
			write!(formatter, "Generic Error Number '{}': '{}'", self.osErrorNumber, lastErrorMessageOnThisThread.unwrap())
		}
		else
		{
			write!(formatter, "Generic Error Number '{}': No message ({})", self.osErrorNumber, lastErrorMessageOnThisThread.unwrap_err())
		}
	}
}

impl error::Error for GenericError
{
	fn description(&self) -> &str
	{
		"Generic Error"
	}
	
	fn cause(&self) -> Option<&error::Error>
	{
		None
	}
}

impl GenericError
{
	#[inline(always)]
	fn new(osErrorNumber: i32) -> Self
	{
		Self
		{
			osErrorNumber: osErrorNumber,
			lastErrorMessageOnThisThread: LastErrorMessageOnThisThreadIsInvalidError::lastErrorMessageOnThisThread(),
		}
	}
}
