// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Chain length
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChainLength(u10);

impl ChainLength
{
	#[inline(always)]
	fn get_bag<'bag, B: Block>(&self, bags: &'bag [Bag<B>; InclusiveMaximumChainLength]) -> &'bag Bag<B>
	{
		unsafe { bags.get_unchecked(self.as_index()) }
	}
	
	#[inline(always)]
	fn as_index(self) -> usize
	{
		self.0 as usize
	}
	
	/// x
	#[inline(always)]
	pub fn as_length(self) -> usize
	{
		self.0 as usize + 1
	}
}
