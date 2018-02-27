// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


quick_error!
{
	/// Reason for failing to instantiate.
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum OutOfMemoryError
	{
		/// The free list has no more space.
		FreeList
		{
			description("No more space (currently) available in FreeList")
		}
		
		/// The cto pool arc has no more space.
		CtoPoolArc(cause: PmdkError)
		{
			cause(cause)
			description("No more space (currently) available in CtoPoolArc")
		}
	}
}
