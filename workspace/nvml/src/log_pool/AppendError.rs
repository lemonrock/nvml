// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	/// Reason for failing to append to log.
	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum AppendError
	{
		/// The log pool has no more space.
		OutOfSpace
		{
			description("No more space (currently) available in persistent memory log pool")
		}
		
		/// The log pool is backed by read-only memory.
		ReadOnly
		{
			description("Persistent memory log pool is backed by (currently) read-only memory")
		}
	}
}
