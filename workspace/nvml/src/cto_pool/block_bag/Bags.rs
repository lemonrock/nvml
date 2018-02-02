// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Bags
pub struct Bags<B: Block>
{
	bags: [Bag<B>; InclusiveMaximumChainLength],
	block_meta_data_items: [BlockMetaData<B>],
}

impl<B: Block> Bags<B>
{
	/// add
	#[inline(always)]
	pub fn add(&self, chain_length: ChainLength, add_block: BlockPointer<B>)
	{
		debug_assert!(add_block.is_not_null(), "add_block should not be null");
		
		let bag = chain_length.get_bag(&self.bags);
		bag.add(chain_length, add_block, &self.block_meta_data_items)
	}
	
	/// remove
	#[inline(always)]
	pub fn remove(&self, chain_length: ChainLength) -> BlockPointer<B>
	{
		let bag = chain_length.get_bag(&self.bags);
		bag.remove(chain_length, &self.block_meta_data_items)
	}
	
	/// try_to_cut
	#[inline(always)]
	pub fn try_to_cut(&self, might_not_be_in_bag_block: BlockPointer<B>) -> bool
	{
		debug_assert!(might_not_be_in_bag_block.is_not_null(), "might_not_be_in_bag_block should not be null");
		
		let might_not_be_in_bag_block_meta_data = might_not_be_in_bag_block.expand_to_pointer_to_meta_data_unchecked(&self.block_meta_data_items);
		
		let mut chain_length_and_bag_stripe_index = might_not_be_in_bag_block_meta_data.chain_length_and_bag_stripe_index();
		while let Some(bag_stripe_index) = chain_length_and_bag_stripe_index.bag_stripe_index()
		{
			let chain_length = chain_length_and_bag_stripe_index.chain_length();
			let bag: &Bag<B> = chain_length.get_bag(&self.bags);
			
			if bag.try_to_cut(chain_length, might_not_be_in_bag_block, might_not_be_in_bag_block_meta_data, &self.block_meta_data_items, bag_stripe_index)
			{
				return true
			}
			
			spin_loop_hint();
			chain_length_and_bag_stripe_index = might_not_be_in_bag_block_meta_data.chain_length_and_bag_stripe_index();
		}
		
		false
	}
}
