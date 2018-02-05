// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ChainLengthAndBagStripeIndex(u16);

impl ChainLengthAndBagStripeIndex
{
	#[inline(always)]
	fn default() -> Self
	{
		ChainLengthAndBagStripeIndex::new(ChainLength::from_length(1), None)
	}
}

impl ChainLengthAndBagStripeIndex
{
	const Bit15: u16 = 0x8000;
	
	const BagStripeIndexShift: u16 = 10;
	
	#[inline(always)]
	pub(crate) fn new(chain_length: ChainLength, bag_stripe_index: Option<BagStripeIndex>) -> Self
	{
		let mut value = chain_length.0;
		if let Some(bag_stripe_index) = bag_stripe_index
		{
			value += ((bag_stripe_index.0 as u16) << Self::BagStripeIndexShift) | Self::Bit15;
		}
		ChainLengthAndBagStripeIndex(value)
	}
	
	#[inline(always)]
	pub(crate) fn chain_length(self) -> ChainLength
	{
		const Bits0To9Mask: u16 = 0x03FF;
		ChainLength(self.0 & Bits0To9Mask)
	}
	
	#[inline(always)]
	pub(crate) fn bag_stripe_index(self) -> Option<BagStripeIndex>
	{
		const Bits14To10Mask: u16 = 0x7C00;
		if self.0 & Self::Bit15 == Self::Bit15
		{
			Some(BagStripeIndex(((self.0 & Bits14To10Mask) >> Self::BagStripeIndexShift) as u5))
		}
		else
		{
			None
		}
	}
}
