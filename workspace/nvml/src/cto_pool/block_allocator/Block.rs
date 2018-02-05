// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A block
pub trait Block: Copy
{
	/// How are blocks persisted?
	type P: Persistence;
	
	/// Must be a power of two.
	const BlockSizeInBytes: usize;
	
	#[doc(hidden)]
	#[inline(always)]
	fn number_of_blocks_required_and_capacity_in_use_of_last_chain(requested_size: usize) -> (usize, usize)
	{
		let remainder = requested_size % Self::BlockSizeInBytes;
		
		if remainder == 0
		{
			(requested_size / Self::BlockSizeInBytes, Self::BlockSizeInBytes)
		}
		else
		{
			((requested_size / Self::BlockSizeInBytes) + 1, remainder)
		}
	}
}
