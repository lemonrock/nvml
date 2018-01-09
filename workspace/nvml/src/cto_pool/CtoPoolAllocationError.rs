// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Represents a failure to allocate or initialize an object.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CtoPoolAllocationError<InitializationError>
{
	/// Failed to allocate memory for an object.
	Allocation(PmdkError),
	
	/// Failed to initialize an object.
	Initialization(InitializationError),
}

impl<InitializationError: Display> Display for CtoPoolAllocationError<InitializationError>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>
	{
		use self::CtoPoolAllocationError::*;
		
		match *self
		{
			Allocation(ref pmdk_error) => Display::fmt(pmdk_error, formatter),
			
			Initialization(ref initialization_error) => Display::fmt(initialization_error, formatter),
		}
	}
}

impl<InitializationError: error::Error> error::Error for CtoPoolAllocationError<InitializationError>
{
	#[inline(always)]
	fn description(&self) -> &str
	{
		"Cto Pool Allocation Error"
	}
	
	#[inline(always)]
	fn cause(&self) -> Option<&error::Error>
	{
		use self::CtoPoolAllocationError::*;
		
		match *self
		{
			Allocation(ref pmdk_error) => Some(pmdk_error),
			
			Initialization(ref initialization_error) => Some(initialization_error),
		}
	}
}
