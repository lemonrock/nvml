// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CtoPoolOpenError<InitializationError>
{
	Open(GenericError),
	
	RootCreation(CtoPoolAllocationError<InitializationError>),
}

impl<InitializationError: Display> Display for CtoPoolOpenError<InitializationError>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>
	{
		use self::CtoPoolOpenError::*;
		
		match *self
		{
			Open(ref generic_error) => Display::fmt(generic_error, formatter),
			
			RootCreation(ref cto_pool_allocation_error) => Display::fmt(cto_pool_allocation_error, formatter),
		}
	}
}

impl<InitializationError: error::Error> error::Error for CtoPoolOpenError<InitializationError>
{
	#[inline(always)]
	fn description(&self) -> &str
	{
		"Cto Pool Open Error"
	}
	
	#[inline(always)]
	fn cause(&self) -> Option<&error::Error>
	{
		use self::CtoPoolOpenError::*;
		
		match *self
		{
			Open(ref generic_error) => Some(generic_error),
			
			RootCreation(ref cto_pool_allocation_error) => Some(cto_pool_allocation_error),
		}
	}
}
