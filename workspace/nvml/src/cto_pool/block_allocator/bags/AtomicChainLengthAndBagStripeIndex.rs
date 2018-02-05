// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct AtomicChainLengthAndBagStripeIndex(AtomicU16);

impl Default for AtomicChainLengthAndBagStripeIndex
{
	#[inline(always)]
	fn default() -> Self
	{
		AtomicChainLengthAndBagStripeIndex(AtomicU16::new(ChainLengthAndBagStripeIndex::default().0))
	}
}

impl AtomicChainLengthAndBagStripeIndex
{
	#[inline(always)]
	pub(crate) fn get(&self) -> ChainLengthAndBagStripeIndex
	{
		ChainLengthAndBagStripeIndex(self.0.load(Acquire))
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, new_value: ChainLengthAndBagStripeIndex)
	{
		self.0.store(new_value.0, Release)
	}
}
