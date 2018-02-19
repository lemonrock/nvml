// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Persistent Memory
pub struct Chains<B: Block>
{
	block_allocator: CtoArc<BlockAllocator<B>>,
	head_of_chains_linked_list: BlockPointer<B>,
}

impl<B: Block> Drop for Chains<B>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.head_of_chains_linked_list.is_not_null()
		{
			let head = self.block_allocator.block_meta_data_unchecked(self.head_of_chains_linked_list);
			head.recycle_chains_into_block_allocator(self.block_allocator.as_ref(), self.head_of_chains_linked_list);
		}
		
		let cto_pool_arc = self.block_allocator.cto_pool_arc.clone();
		cto_pool_arc.free_pointer(self)
	}
}

impl<B: Block> CtoSafe for Chains<B>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.block_allocator.cto_pool_opened(cto_pool_arc)
	}
}

impl<B: Block> Chains<B>
{
	#[inline(always)]
	fn new(block_allocator: &CtoArc<BlockAllocator<B>>) -> Result<NonNull<Self>, ()>
	{
		match block_allocator.cto_pool_arc.pool_pointer().aligned_alloc(size_of::<Self>(), size_of::<Self>())
		{
			Err(_) => Err(()),
			Ok(void_pointer) =>
			{
				let mut this = unsafe { NonNull::new_unchecked(void_pointer as *mut Self) };
				
				unsafe
				{
					write(&mut this.as_mut().block_allocator, block_allocator.clone());
					write(&mut this.as_mut().head_of_chains_linked_list, BlockPointer::Null);
				}
				
				Ok(this)
			}
		}
	}
	
	/// Stored in Volatile Memory
	#[inline(always)]
	pub fn copy_bytes_into_chains_start<'block_meta_data>(&'block_meta_data self) -> RestartCopyIntoAt<'block_meta_data, B>
	{
		RestartCopyIntoAt::new(self.block_allocator.memory_base_pointer, self.head_of_chains_linked_list, &self.block_allocator.block_meta_data_items)
	}
	
	/// Stored in Volatile Memory
	#[inline(always)]
	pub fn copy_bytes_from_chains_start<'block_meta_data>(&'block_meta_data self) -> RestartCopyFromAt<'block_meta_data, B>
	{
		RestartCopyFromAt::new(self.block_allocator.memory_base_pointer, self.head_of_chains_linked_list, &self.block_allocator.block_meta_data_items)
	}
}
