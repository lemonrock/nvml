// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Adds additional methods to Path to make it easier to open persistent memory object pools.
pub trait PersistentMemoryObjectPoolPathExt
{
	/// Not supported if the path is a /dev/daxN ('Device DAX') device file path
	#[inline(always)]
	fn validate_object_pool_is_consistent(&self, layoutName: Option<&str>) -> Result<bool, PmdkError>;
	
	#[inline(always)]
	fn open_object_pool(&self, layoutName: Option<&str>) -> Result<*mut PMEMobjpool, PmdkError>;
	
	#[inline(always)]
	fn create_object_pool(&self, layoutName: Option<&str>, poolSize: usize, mode: mode_t) -> Result<*mut PMEMobjpool, PmdkError>;
}

impl PersistentMemoryObjectPoolPathExt for Path
{
	#[inline(always)]
	fn validate_object_pool_is_consistent(&self, layoutName: Option<&str>) -> Result<bool, PmdkError>
	{
		let layout = layoutAsRawPointer(layoutName);
		let result = use_path!(self, pmemobj_check, layout);
		unsafe { CString::from_raw(layout as *mut _) };
		
		match result
		{
			1 => Ok(false),
			0 => Ok(true),
			-1 => PmdkError::obj("pmemobj_check"),
			illegal @ _ => panic!("pmemobj_check() returned illegal value '{}'", illegal)
		}
	}
	
	#[inline(always)]
	fn open_object_pool(&self, layoutName: Option<&str>) -> Result<*mut PMEMobjpool, PmdkError>
	{
		let layout = layoutAsRawPointer(layoutName);
		let result = use_path!(self, pmemobj_open, layout);
		unsafe { CString::from_raw(layout as *mut _) };
		
		if unlikely(result.is_null())
		{
			PmdkError::obj("pmemobj_open")
		}
		else
		{
			Ok(result)
		}
	}
	
	#[inline(always)]
	fn create_object_pool(&self, layoutName: Option<&str>, poolSize: usize, mode: mode_t) -> Result<*mut PMEMobjpool, PmdkError>
	{
		let layout = layoutAsRawPointer(layoutName);
		let result = use_path!(self, pmemobj_create, layout, poolSize, mode);
		unsafe { CString::from_raw(layout as *mut _) };
		
		if unlikely(result.is_null())
		{
			PmdkError::obj("pmemobj_create")
		}
		else
		{
			Ok(result)
		}
	}
}

#[inline(always)]
fn layoutAsRawPointer(layoutName: Option<&str>) -> *const c_char
{
	if let Some(layoutName) = layoutName
	{
		debug_assert!(layoutName.len() + 1 <= PMEMOBJ_MAX_ALLOC_SIZE as usize, "layoutName length '{}' + 1 is greater than PMEMOBJ_MAX_ALLOC_SIZE '{}'", layoutName.len(), PMEMOBJ_MAX_ALLOC_SIZE);
		
		let cString = CString::new(layoutName).expect("Invalid layout name");
		
		cString.into_raw()
	}
	else
	{
		null()
	}
}
