// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub trait PersistentMemoryCtoPoolPathExt
{
	#[inline(always)]
	fn validatePersistentMemoryCtoPoolIsConsistent(&self, layout_name: &CStr) -> Result<bool, GenericError>;
	
	/// blockSize can be zero, in which case it is not explicitly checked for a match; EINVAL occurs in this case
	#[inline(always)]
	fn openPersistentMemoryCtoPool(&self, layout_name: &CStr) -> Result<*mut PMEMctopool, GenericError>;
	
	#[inline(always)]
	fn createPersistentMemoryCtoPool(&self, layout_name: &CStr, pool_size: usize, mode: mode_t) -> Result<*mut PMEMctopool, GenericError>;
}

impl PersistentMemoryCtoPoolPathExt for Path
{
	#[inline(always)]
	fn validatePersistentMemoryCtoPoolIsConsistent(&self, layout_name: &CStr) -> Result<bool, GenericError>
	{
		let result = usePath!(self, pmemcto_check, layout_name.as_ptr());
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => handleError!(pmemcto_check),
			illegal @ _ => panic!("pmemcto_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn openPersistentMemoryCtoPool(&self, layout_name: &CStr) -> Result<*mut PMEMctopool, GenericError>
	{
		let result = usePath!(self, pmemcto_open, layout_name.as_ptr());
		
		if unlikely(result.is_null())
		{
			handleError!(pmemcto_open)
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn createPersistentMemoryCtoPool(&self, layout_name: &CStr, pool_size: usize, mode: mode_t) -> Result<*mut PMEMctopool, GenericError>
	{
		let result = usePath!(self, pmemcto_create, layout_name.as_ptr(), pool_size, mode);
		
		if unlikely(result.is_null())
		{
			handleError!(pmemcto_create)
		}
		else
		{
			Ok(result)
		}
	}
}
