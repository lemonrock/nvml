// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct BagStripeIndex(u5);

impl BagStripeIndex
{
	#[inline(always)]
	pub(crate) fn get_bag_stripe<'bag_stripe, B: Block>(&self, bag_stripe_array: &'bag_stripe [BagStripe<B>; BagStripeArrayLength]) -> &'bag_stripe BagStripe<B>
	{
		unsafe { bag_stripe_array.get_unchecked(self.as_index()) }
	}
	
	#[inline(always)]
	pub(crate) fn as_index(self) -> usize
	{
		self.0 as usize
	}
}
