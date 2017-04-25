// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


quick_error!
{
	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum LastErrorMessageOnThisThreadIsInvalidError
	{
		Null
		{
			description("Error message pointer is null")
		}
		
		InvalidUtf8
		{
			description("Error message is not valid ASCII NUL terminated UTF-8")
		}
	}
}

impl LastErrorMessageOnThisThreadIsInvalidError
{
	#[inline(always)]
	pub fn lastErrorMessageOnThisThread() -> Result<String, LastErrorMessageOnThisThreadIsInvalidError>
	{
		let pointer = unsafe { pmempool_errormsg() };
		if pointer.is_null()
		{
			return Err(LastErrorMessageOnThisThreadIsInvalidError::Null)
		}
		
		let threadSafeCStringPointer = unsafe { CStr::from_ptr(pointer) };
		let x: CString = threadSafeCStringPointer.into();
		match x.into_string()
		{
			Ok(value) => Ok(value),
			Err(_) => Err(LastErrorMessageOnThisThreadIsInvalidError::InvalidUtf8)
		}
	}
	
}
