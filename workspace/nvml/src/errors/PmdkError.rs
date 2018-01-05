// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a generic error
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PmdkError
{
	/// OS error number, eg EINVAL. Processor architecture and Operating System specific.
	pub os_error_number: i32,
	
	/// Error message from PMDK, if available.
	pub last_error_message_on_this_thread: Result<String, LastErrorMessageOnThisThreadIsInvalidError>,
	
	/// PMDK function called that produced this error.
	pub function_name: &'static str,
}

impl Display for PmdkError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>
	{
		let lastErrorMessageOnThisThread = self.last_error_message_on_this_thread.as_ref();
		if likely(lastErrorMessageOnThisThread.is_ok())
		{
			write!(formatter, "Generic Error Number '{}': '{}'", self.os_error_number, lastErrorMessageOnThisThread.unwrap())
		}
		else
		{
			write!(formatter, "Generic Error Number '{}': No message ({})", self.os_error_number, lastErrorMessageOnThisThread.unwrap_err())
		}
	}
}

impl error::Error for PmdkError
{
	#[inline(always)]
	fn description(&self) -> &str
	{
		"Generic Error"
	}
	
	#[inline(always)]
	fn cause(&self) -> Option<&error::Error>
	{
		None
	}
}

impl PmdkError
{
	#[inline(always)]
	pub(crate) fn block<T>(function_name: &'static str) -> Result<T, Self>
	{
		Self::err(pmemblk_errormsg, function_name)
	}
	
	#[inline(always)]
	pub(crate) fn block_panic(function_name: &'static str) -> !
	{
		Self::new(pmemblk_errormsg, function_name).panic()
	}
	
	#[inline(always)]
	pub(crate) fn cto<T>(function_name: &'static str) -> Result<T, Self>
	{
		Self::err(pmemcto_errormsg, function_name)
	}
	
	#[inline(always)]
	pub(crate) fn log<T>(function_name: &'static str) -> Result<T, Self>
	{
		Self::err(pmemlog_errormsg, function_name)
	}
	
	#[inline(always)]
	pub(crate) fn obj<T>(function_name: &'static str) -> Result<T, Self>
	{
		Self::err(pmemobj_errormsg, function_name)
	}
	
	#[inline(always)]
	pub(crate) fn pmem<T>(function_name: &'static str) -> Result<T, Self>
	{
		Self::err(pmem_errormsg, function_name)
	}
	
	#[inline(always)]
	pub(crate) fn err<T>(error_function: ErrorFunction, function_name: &'static str) -> Result<T, Self>
	{
		Err(Self::new(error_function, function_name))
	}
	
	#[inline(always)]
	pub(crate) fn new(error_function: ErrorFunction, function_name: &'static str) -> Self
	{
		let os_error_number = errno().0;
		let last_error_message_on_this_thread = LastErrorMessageOnThisThreadIsInvalidError::last_error_message_on_this_thread(error_function);
		
		if likely(os_error_number > 0)
		{
			Self
			{
				os_error_number,
				last_error_message_on_this_thread,
				function_name,
			}
		}
		else
		{
			if likely(last_error_message_on_this_thread.is_ok())
			{
				panic!("Invalid errno value '{}' from {} (last error message was '{}')", os_error_number, function_name, last_error_message_on_this_thread.unwrap());
			}
			else
			{
				panic!("Invalid errno value '{}' from {} (last error message was unavailable because '{}')", os_error_number, function_name, last_error_message_on_this_thread.unwrap_err());
			}
		}
	}
	
	/// Raise a panic with this error as the cause.
	#[inline(always)]
	pub fn panic(self) -> !
	{
		panic!("Unexpected or fatal error; {}", self)
	}
	
	/// is this error EINVAL?, eg due to invalid arguments?
	#[inline(always)]
	pub fn is_EINVAL(&self) -> bool
	{
		self.os_error_number == EINVAL
	}
	
	/// is this error ENOMEM?, eg due to memory exhaustion?
	#[inline(always)]
	pub fn is_ENOMEM(&self) -> bool
	{
		self.os_error_number == ENOMEM
	}
}
