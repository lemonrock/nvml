// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Adds additional methods to Path to make it easier to open persistent memory object pools.
pub trait ObjectPoolPathExt
{
	/// Validate an object pool.
	/// Not supported if the path is a `/dev/daxN` ('Device DAX') device file path.
	#[inline(always)]
	fn validate_object_pool_is_consistent(&self, layout_name: Option<&str>) -> Result<bool, PmdkError>;
	
	/// Open an existing object pool.
	#[inline(always)]
	fn open_object_pool(&self, layout_name: Option<&str>) -> Result<*mut PMEMobjpool, PmdkError>;
	
	/// Create (and implicitly open) a new object pool.
	#[inline(always)]
	fn create_object_pool(&self, layout_name: Option<&str>, pool_size: usize, mode: mode_t) -> Result<*mut PMEMobjpool, PmdkError>;
}

impl ObjectPoolPathExt for Path
{
	#[inline(always)]
	fn validate_object_pool_is_consistent(&self, layout_name: Option<&str>) -> Result<bool, PmdkError>
	{
		let layout = layout_as_raw_pointer(layout_name);
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
	fn open_object_pool(&self, layout_name: Option<&str>) -> Result<*mut PMEMobjpool, PmdkError>
	{
		let layout = layout_as_raw_pointer(layout_name);
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
	fn create_object_pool(&self, layout_name: Option<&str>, pool_size: usize, mode: mode_t) -> Result<*mut PMEMobjpool, PmdkError>
	{
		let layout = layout_as_raw_pointer(layout_name);
		let result = use_path!(self, pmemobj_create, layout, pool_size, mode);
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
fn layout_as_raw_pointer(layout_name: Option<&str>) -> *const c_char
{
	if let Some(layout_name) = layout_name
	{
		debug_assert!(layout_name.len() + 1 <= PMEMOBJ_MAX_ALLOC_SIZE as usize, "layout_name length '{}' + 1 is greater than PMEMOBJ_MAX_ALLOC_SIZE '{}'", layout_name.len(), PMEMOBJ_MAX_ALLOC_SIZE);
		
		let c_string = CString::new(layout_name).expect("Invalid layout name");
		
		c_string.into_raw()
	}
	else
	{
		null()
	}
}
