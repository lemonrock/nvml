// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PersistentMemoryLogPoolPathExt
{
	/// Not supported if the path is a /dev/daxN ('Device DAX') device file path
	#[inline(always)]
	fn validatePersistentMemoryLogPoolIsConsistent(&self) -> Result<bool, GenericError>;
	
	#[inline(always)]
	fn openPersistentMemoryLogPool(&self) -> Result<*mut PMEMlogpool, GenericError>;
}

macro_rules! usePath
{
	($self: ident, $function: path) =>
	{
		{
			let osPath = $self.as_os_str();
			let bytes = osPath.as_bytes();
			let pointer = bytes.as_ptr() as *const c_char;

			unsafe { $function(pointer) }
		}
	}
}

macro_rules! handleError
{
	($function: path) =>
	{
		{
			let errorNumber = errno().0;
			if likely(errorNumber > 0)
			{
				Err(GenericError::new(errorNumber))
			}
			else
			{
				const functionName: &'static str = stringify!($function);
				let reason = LastErrorMessageOnThisThreadIsInvalidError::lastErrorMessageOnThisThread();
				if likely(reason.is_ok())
				{
					panic!("Invalid errno value '{}' from {} (last error message was unavailable because '{}')", errorNumber, functionName, reason.unwrap());
				}
				else
				{
					panic!("Invalid errno value '{}' from {} (last error message was unavailable because '{}')", errorNumber, functionName, reason.unwrap_err());
				}
			}
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
}
