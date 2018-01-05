// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait to make it easier to use a Path to access a block pool.
pub trait PersistentMemoryBlockPoolPathExt
{
	/// Validate a block pool.
	/// Not supported if the path is a `/dev/daxN` ('Device DAX') device file path.
	/// block_size can be zero, in which case it is not explicitly checked for a match.
	#[inline(always)]
	fn validate_block_pool_is_consistent(&self, block_size: usize) -> Result<bool, PmdkError>;
	
	/// Open an existing block pool.
	/// block_size can be zero, in which case it is not explicitly checked for a match; EINVAL occurs in this case.
	#[inline(always)]
	fn open_block_pool(&self, block_size: usize) -> Result<*mut PMEMblkpool, PmdkError>;
	
	/// Create (and implicitly open) a new block pool.
	#[inline(always)]
	fn create_block_pool(&self, block_size: usize, pool_size: usize, mode: mode_t) -> Result<*mut PMEMblkpool, PmdkError>;
}

impl PersistentMemoryBlockPoolPathExt for Path
{
	#[inline(always)]
	fn validate_block_pool_is_consistent(&self, block_size: usize) -> Result<bool, PmdkError>
	{
		let result = use_path!(self, pmemblk_check, block_size);
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => PmdkError::block("pmemblk_check"),
			illegal @ _ => panic!("pmemblk_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn open_block_pool(&self, block_size: usize) -> Result<*mut PMEMblkpool, PmdkError>
	{
		let result = use_path!(self, pmemblk_open, block_size);
		
		if unlikely(result.is_null())
		{
			PmdkError::block("pmemblk_open")
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn create_block_pool(&self, block_size: usize, pool_size: usize, mode: mode_t) -> Result<*mut PMEMblkpool, PmdkError>
	{
		let result = use_path!(self, pmemblk_create, block_size, pool_size, mode);
		
		if unlikely(result.is_null())
		{
			PmdkError::block("pmemblk_create")
		}
		else
		{
			Ok(result)
		}
	}
}
