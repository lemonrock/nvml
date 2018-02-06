// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Effectively operates like a stack.
#[derive(Debug)]
pub(crate) struct BagStripe<B: Block>
{
	head: AtomicBlockPointer<B>,
	spin_lock: BestSpinLockForCompilationTarget,
}

impl<B: Block> Default for BagStripe<B>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			head: AtomicBlockPointer::default(),
			spin_lock: BestSpinLockForCompilationTarget::default(),
		}
	}
}

impl<B: Block> CtoSafe for BagStripe<B>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, _cto_pool_arc: &CtoPoolArc)
	{
		self.spin_lock.forcibly_unlock_spin_lock()
	}
}

impl<B: Block> BagStripe<B>
{
	#[inline(always)]
	fn add(&self, chain_length: ChainLength, add_block: BlockPointer<B>, block_meta_data_items: &BlockMetaDataItems<B>, add_block_meta_data: &BlockMetaData<B>, next_bag_stripe_index: BagStripeIndex)
	{
		debug_assert!(add_block.is_not_null(), "add_block can not be null");
		debug_assert!(add_block_meta_data.get_next().is_null(), "add_block `next` can not be non-null");
		debug_assert!(add_block_meta_data.get_previous().is_null(), "add_block `previous` can not be non-null");
		debug_assert!(add_block_meta_data.chain_length_and_bag_stripe_index().bag_stripe_index().is_none(), "add_block should not be in a bag already");
		
		self.acquire_spin_lock();
		{
			let old_head = self.get_head_relaxed();
			self.set_head_relaxed(add_block);
			
			if old_head.is_not_null()
			{
				let old_head_block_meta_data = old_head.expand_to_pointer_to_meta_data_unchecked(block_meta_data_items);
				
				add_block_meta_data.set_previous(old_head);
				old_head_block_meta_data.set_next(add_block);
			}
			
			B::P::flush_struct(self);
		}
		
		self.unlock_spin_lock();
		
		add_block_meta_data.release(chain_length, next_bag_stripe_index)
	}
	
	#[inline(always)]
	fn remove(&self, chain_length: ChainLength, block_meta_data_items: &BlockMetaDataItems<B>) -> BlockPointer<B>
	{
		if !self.try_to_acquire_spin_lock()
		{
			return BlockPointer::Null
		}
		
		let result =
		{
			let old_head = self.get_head_relaxed();
			if old_head.is_null()
			{
				BlockPointer::Null
			}
			else
			{
				let old_head_block_meta_data = old_head.expand_to_pointer_to_meta_data_unchecked(block_meta_data_items);
				
				self.set_head_relaxed(old_head_block_meta_data.get_previous());
				
				old_head_block_meta_data.acquire(chain_length);
				
				B::P::flush_struct(self);
				
				old_head
			}
		};
		
		self.unlock_spin_lock();
		
		result
	}
	
	#[inline(always)]
	fn try_to_cut(&self, chain_length: ChainLength, cut_block: BlockPointer<B>, cut_block_meta_data: &BlockMetaData<B>, block_meta_data_items: &BlockMetaDataItems<B>) -> bool
	{
		if !self.try_to_acquire_spin_lock()
		{
			return false
		}
		
		let result =
		{
			let old_head = self.get_head_relaxed();
			if old_head.is_null()
			{
				false
			}
			else if old_head.equals(cut_block)
			{
				debug_assert!(cut_block_meta_data.get_next().is_null(), "next should be null if cut_block is at head");
				
				self.set_head_relaxed(cut_block_meta_data.get_previous());
				
				cut_block_meta_data.acquire(chain_length);
				B::P::flush_struct(self);
				true
			}
			else
			{
				let before_block = cut_block_meta_data.get_previous();
				if let Some(before_block_meta_data) = before_block.expand_to_pointer_to_meta_data(block_meta_data_items)
				{
					before_block_meta_data.set_next(cut_block_meta_data.get_next());
				}
				
				debug_assert!(cut_block_meta_data.get_next().is_not_null(), "next should not be null because cut_block is not at head");
				let after_block_meta_data = cut_block_meta_data.get_next().expand_to_pointer_to_meta_data_unchecked(block_meta_data_items);
				after_block_meta_data.set_previous(before_block);
				
				cut_block_meta_data.acquire(chain_length);
				B::P::flush_struct(self);
				true
			}
		};
		
		self.unlock_spin_lock();
		
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn get_head_relaxed(&self) -> BlockPointer<B>
	{
		self.head.get_relaxed()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn set_head_relaxed(&self, new_head: BlockPointer<B>)
	{
		self.head.set_relaxed(new_head)
	}
	
	// Returns true if acquired spin lock
	#[doc(hidden)]
	#[inline(always)]
	fn acquire_spin_lock(&self)
	{
		self.spin_lock.acquire_spin_lock()
	}
	
	// Returns true if acquired spin lock
	#[doc(hidden)]
	#[inline(always)]
	fn try_to_acquire_spin_lock(&self) -> bool
	{
		self.spin_lock.try_to_acquire_spin_lock()
	}
	
	#[inline(always)]
	#[doc(hidden)]
	fn unlock_spin_lock(&self)
	{
		self.spin_lock.unlock_spin_lock()
	}
}
