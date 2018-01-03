// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PersistentMemoryBlockPoolPathExt
{
	/// Not supported if the path is a /dev/daxN ('Device DAX') device file path
	/// blockSize can be zero, in which case it is not explicitly checked for a match
	#[inline(always)]
	fn validatePersistentMemoryBlockPoolIsConsistent(&self, blockSize: usize) -> Result<bool, GenericError>;
	
	/// blockSize can be zero, in which case it is not explicitly checked for a match; EINVAL occurs in this case
	#[inline(always)]
	fn openPersistentMemoryBlockPool(&self, blockSize: usize) -> Result<*mut PMEMblkpool, GenericError>;
	
	#[inline(always)]
	fn createPersistentMemoryBlockPool(&self, blockSize: usize, poolSize: usize, mode: mode_t) -> Result<*mut PMEMblkpool, GenericError>;
}

macro_rules! handleError
{
	($function: path) =>
	{
		{
			let osErrorNumber = errno().0;
			const functionName: &'static str = stringify!($function);
			Err(GenericError::new(osErrorNumber, pmempool_errormsg, functionName))
		}
	}
}

impl PersistentMemoryBlockPoolPathExt for Path
{
	#[inline(always)]
	fn validatePersistentMemoryBlockPoolIsConsistent(&self, blockSize: usize) -> Result<bool, GenericError>
	{
		let result = usePath!(self, pmemblk_check, blockSize);
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => handleError!(pmemblk_check),
			illegal @ _ => panic!("pmemblk_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn openPersistentMemoryBlockPool(&self, blockSize: usize) -> Result<*mut PMEMblkpool, GenericError>
	{
		let result = usePath!(self, pmemblk_open, blockSize);
		
		if unlikely(result.is_null())
		{
			handleError!(pmemblk_open)
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn createPersistentMemoryBlockPool(&self, blockSize: usize, poolSize: usize, mode: mode_t) -> Result<*mut PMEMblkpool, GenericError>
	{
		let result = usePath!(self, pmemblk_create, blockSize, poolSize, mode);
		
		if unlikely(result.is_null())
		{
			handleError!(pmemblk_create)
		}
		else
		{
			Ok(result)
		}
	}
}
