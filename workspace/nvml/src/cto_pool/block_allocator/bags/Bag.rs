// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct Bag<B: Block>
{
	bag_stripe_index_counter: BagStripeIndexCounter,
	removal_counter: RemovalCounter,
	bag_stripe_array: [BagStripe<B>; BagStripeArrayLength],
}

impl<B: Block> Default for Bag<B>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			bag_stripe_index_counter: BagStripeIndexCounter::default(),
			removal_counter: RemovalCounter::default(),
			bag_stripe_array:
			{
				let mut array: [BagStripe<B>; BagStripeArrayLength] = unsafe { uninitialized() };
				
				for uninitialized_bag_stripe in array.iter_mut()
				{
					unsafe { write(uninitialized_bag_stripe, BagStripe::default()) }
				}
				
				array
			},
		}
	}
}

impl<B: Block> CtoSafe for Bag<B>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		for bag_stripe in self.bag_stripe_array.iter_mut()
		{
			bag_stripe.cto_pool_opened(cto_pool_arc)
		}
	}
}

impl<B: Block> Bag<B>
{
	// add tries to ensure a round-robin, uniform distribution amongst stripes.
	#[inline(always)]
	pub(crate) fn add(&self, chain_length: ChainLength, add_block: BlockPointer<B>, block_meta_data_items: &BlockMetaDataItems<B>)
	{
		debug_assert!(add_block.is_not_null(), "add_block can not be null");
		
		let add_block_meta_data = add_block.expand_to_pointer_to_meta_data_unchecked(block_meta_data_items);
		
		debug_assert!(add_block_meta_data.get_next().is_null(), "add_block `next` can not be non-null");
		debug_assert!(add_block_meta_data.get_previous().is_null(), "add_block `previous` can not be non-null");
		debug_assert!(add_block_meta_data.chain_length_and_bag_stripe_index().bag_stripe_index().is_none(), "add_block should not be in a bag already");
		
		let next_bag_stripe_index = self.obtain_next_bag_stripe_index();
		let bag_stripe = next_bag_stripe_index.get_bag_stripe(&self.bag_stripe_array);
		
		bag_stripe.add(chain_length, add_block, block_meta_data_items, add_block_meta_data, next_bag_stripe_index)
	}
	
	// remove tries to ensure a round-robin, uniform distribution amongst stripes by always trying to remove from the oldest added to stripe.
	#[inline(always)]
	pub(crate) fn remove(&self, chain_length: ChainLength, block_meta_data_items: &BlockMetaDataItems<B>) -> BlockPointer<B>
	{
		let mut added_count = self.number_of_blocks_added_over_all_time();
		let mut removed_count = self.number_of_blocks_removed_over_all_time();
		
		// We loop whilst the bag is not empty
		while
		{
			debug_assert!(added_count >= removed_count, "added_count should never be less than removed_count");
			let bag_has_at_least_one_block_to_remove = added_count != removed_count;
			bag_has_at_least_one_block_to_remove
		}
		{
			// We try to remove from bag stripes.
			// When we've exhausted our budget of tries, we loop the outer loop and check if the bag has something
			// If we can't remove from it (eg acquire a spinlock or it's empty when we get there), we try the next one.
			// We start with what is probably the oldest bag stripe added to.
			// We stop after a minimum `BagStripeArrayLength` attempts, and a maximum of the different between removed_count and added_count.
			// The minimum must be `BagStripeArrayLength` in order to try every BagStripe at least once.
			let end_at_index_counter_exclusive = max(added_count, removed_count + BagStripeArrayLength as u64);
			let mut index_counter = removed_count;
			while index_counter < end_at_index_counter_exclusive
			{
				let bag_stripe_index = BagStripeIndexCounter::to_bag_strip_index(index_counter);
				
				let bag_stripe = bag_stripe_index.get_bag_stripe(&self.bag_stripe_array);
				
				let removed = bag_stripe.remove(chain_length, block_meta_data_items);
				if removed.is_not_null()
				{
					self.increment_number_of_blocks_removed_over_all_time();
					return removed
				}
				
				spin_loop_hint();
				index_counter += 1;
			}
			
			spin_loop_hint();
			added_count = self.number_of_blocks_added_over_all_time();
			removed_count = self.number_of_blocks_removed_over_all_time();
		}
		
		BlockPointer::Null
	}
	
	#[inline(always)]
	pub(crate) fn try_to_cut(&self, chain_length: ChainLength, probably_in_bag_block: BlockPointer<B>, probably_in_bag_block_meta_data: &BlockMetaData<B>, block_meta_data_items: &BlockMetaDataItems<B>, bag_stripe_index: BagStripeIndex) -> bool
	{
		let bag_stripe = bag_stripe_index.get_bag_stripe(&self.bag_stripe_array);
		bag_stripe.try_to_cut(chain_length, probably_in_bag_block, probably_in_bag_block_meta_data, block_meta_data_items)
	}
	
	#[inline(always)]
	fn number_of_blocks_added_over_all_time(&self) -> u64
	{
		self.bag_stripe_index_counter.current_count()
	}
	
	#[inline(always)]
	fn obtain_next_bag_stripe_index(&self) -> BagStripeIndex
	{
		self.bag_stripe_index_counter.next()
	}
	
	#[inline(always)]
	fn number_of_blocks_removed_over_all_time(&self) -> u64
	{
		self.removal_counter.current_count()
	}
	
	#[inline(always)]
	fn increment_number_of_blocks_removed_over_all_time(&self)
	{
		self.removal_counter.increment()
	}
}
