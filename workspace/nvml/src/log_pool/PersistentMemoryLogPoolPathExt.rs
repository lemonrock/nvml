// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An extension trait to make it easier to use a Path to access a log pool.
pub trait PersistentMemoryLogPoolPathExt
{
	/// Validate a log pool.
	/// Not supported if the path is a `/dev/daxN` ('Device DAX') device file path.
	#[inline(always)]
	fn validate_log_pool_is_consistent(&self) -> Result<bool, PmdkError>;
	
	/// Open an existing log pool.
	#[inline(always)]
	fn open_log_pool(&self) -> Result<*mut PMEMlogpool, PmdkError>;
	
	/// Create (and implicitly open) a new log pool.
	#[inline(always)]
	fn create_log_pool(&self, poolSize: usize, mode: mode_t) -> Result<*mut PMEMlogpool, PmdkError>;
}

impl PersistentMemoryLogPoolPathExt for Path
{
	#[inline(always)]
	fn validate_log_pool_is_consistent(&self) -> Result<bool, PmdkError>
	{
		let result = use_path!(self, pmemlog_check);
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => PmdkError::log("pmemlog_check"),
			illegal @ _ => panic!("pmemlog_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn open_log_pool(&self) -> Result<*mut PMEMlogpool, PmdkError>
	{
		let result = use_path!(self, pmemlog_open);
		
		if unlikely(result.is_null())
		{
			PmdkError::log("pmemlog_open")
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn create_log_pool(&self, poolSize: usize, mode: mode_t) -> Result<*mut PMEMlogpool, PmdkError>
	{
		let result = use_path!(self, pmemlog_create, poolSize, mode);
		
		if unlikely(result.is_null())
		{
			PmdkError::log("pmemlog_create")
		}
		else
		{
			Ok(result)
		}
	}
}
