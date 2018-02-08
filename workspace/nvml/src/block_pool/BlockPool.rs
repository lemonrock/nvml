// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A block pool of persistent memory.
#[derive(Debug, Clone)]
pub struct BlockPool(*mut PMEMblkpool, Arc<BlockPoolDropWrapper>);

unsafe impl Send for BlockPool
{
}

unsafe impl Sync for BlockPool
{
}

impl BlockPool
{
	/// Validate an existing pool.
	#[inline(always)]
	pub fn validate(pool_set_file_path: &Path, block_size: usize) -> Result<bool, PmdkError>
	{
		pool_set_file_path.validate_block_pool_is_consistent(block_size)
	}
	
	/// Open an existing pool.
	/// Prefer the use of `BlockPoolConfiguration.open_or_create()`.
	#[inline(always)]
	pub fn open(pool_set_file_path: &Path, validate_block_size_is: Option<usize>) -> Result<Self, PmdkError>
	{
		let block_size = if let Some(block_size) = validate_block_size_is
		{
			assert_ne!(block_size, 0, "block_size can not be zero");
			block_size
		}
		else
		{
			0
		};
		
		pool_set_file_path.open_block_pool(block_size).map(Self::from_handle)
	}
	
	/// Create a new pool.
	/// Prefer the use of `BlockPoolConfiguration.open_or_create()`.
	#[inline(always)]
	pub fn create(pool_set_file_path: &Path, block_size: usize, pool_size: usize, mode: mode_t) -> Result<Self, PmdkError>
	{
		pool_set_file_path.create_block_pool(block_size, pool_size, mode).map(Self::from_handle)
	}
	
	/// Size of blocks in the block pool.
	#[inline(always)]
	pub fn block_size(self) -> usize
	{
		self.0.block_size()
	}
	
	/// How many blocks are available (free) in the block pool?
	#[inline(always)]
	pub fn number_of_blocks_available_in_block_pool(self) -> usize
	{
		self.0.number_of_blocks_available_in_block_pool()
	}
	
	/// Read from a block.
	/// Returns false if the block has previously had its error condition set (see `set_error()`).
	#[inline(always)]
	pub fn read(self, to: *mut c_void, zero_based_block_index: usize) -> bool
	{
		self.0.read_from(to, zero_based_block_index)
	}
	
	/// Write to a block.
	#[inline(always)]
	pub fn write(self, from: *const c_void, zero_based_block_index: usize)
	{
		self.0.write_to(from, zero_based_block_index)
	}
	
	/// Set a block to all zeros.
	#[inline(always)]
	pub fn set_zero(self, zero_based_block_index: usize)
	{
		self.0.set_zero(zero_based_block_index)
	}
	
	/// Set a block to being in an error state (ie set its error condition).
	#[inline(always)]
	pub fn set_error(self, zero_based_block_index: usize)
	{
		self.0.set_error(zero_based_block_index)
	}
	
	#[inline(always)]
	fn from_handle(handle: *mut PMEMblkpool) -> Self
	{
		debug_assert!(handle.is_not_null(), "PMEMblkpool handle is null");
		
		BlockPool(handle, BlockPoolDropWrapper::new(handle))
	}
}
