// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct BlockMetaData<B: Block>
{
	chain_length_and_bag_stripe_index: AtomicChainLengthAndBagStripeIndex,
	next: AtomicBlockPointer<B>,
	previous: AtomicBlockPointer<B>,
}

impl<B: Block> BlockMetaData<B>
{
	#[inline(always)]
	fn chain_length_and_bag_stripe_index(&self) -> ChainLengthAndBagStripeIndex
	{
		self.chain_length_and_bag_stripe_index.get()
	}
	
	#[inline(always)]
	fn release(&self, chain_length: ChainLength, next_bag_stripe_index: BagStripeIndex)
	{
		self.chain_length_and_bag_stripe_index.set(ChainLengthAndBagStripeIndex::new(chain_length, Some(next_bag_stripe_index)))
	}
	
	#[inline(always)]
	fn acquire(&self, chain_length: ChainLength)
	{
		self.chain_length_and_bag_stripe_index.set(ChainLengthAndBagStripeIndex::new(chain_length, None))
	}
	
	#[inline(always)]
	fn get_next(&self) -> BlockPointer<B>
	{
		self.next.get()
	}
	
	#[inline(always)]
	fn set_next(&self, new_next: BlockPointer<B>)
	{
		self.next.set(new_next)
	}
	
	#[inline(always)]
	fn get_previous(&self) -> BlockPointer<B>
	{
		self.previous.get()
	}
	
	#[inline(always)]
	fn set_previous(&self, new_previous: BlockPointer<B>)
	{
		self.previous.set(new_previous)
	}
}
