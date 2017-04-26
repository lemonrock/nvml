// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PersistentMemoryLogPoolPathExt
{
	/// Not supported if the path is a /dev/daxN ('Device DAX') device file path
	#[inline(always)]
	fn validatePersistentMemoryLogPoolIsConsistent(&self) -> Result<bool, GenericError>;
	
	#[inline(always)]
	fn openPersistentMemoryLogPool(&self) -> Result<*mut PMEMlogpool, GenericError>;
	
	#[inline(always)]
	fn createPersistentMemoryLogPool(&self, poolSize: usize, mode: mode_t) -> Result<*mut PMEMlogpool, GenericError>;
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

impl PersistentMemoryLogPoolPathExt for Path
{
	#[inline(always)]
	fn validatePersistentMemoryLogPoolIsConsistent(&self) -> Result<bool, GenericError>
	{
		let result = usePath!(self, pmemlog_check);
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => handleError!(pmemlog_check),
			illegal @ _ => panic!("pmemlog_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn openPersistentMemoryLogPool(&self) -> Result<*mut PMEMlogpool, GenericError>
	{
		let result = usePath!(self, pmemlog_open);
		
		if unlikely(result.is_null())
		{
			handleError!(pmemlog_open)
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn createPersistentMemoryLogPool(&self, poolSize: usize, mode: mode_t) -> Result<*mut PMEMlogpool, GenericError>
	{
		let result = usePath!(self, pmemlog_create, poolSize, mode);
		
		if unlikely(result.is_null())
		{
			handleError!(pmemlog_create)
		}
		else
		{
			Ok(result)
		}
	}
}
