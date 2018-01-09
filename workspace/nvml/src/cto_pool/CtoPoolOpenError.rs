// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Represents a failure to open a CTO pool.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CtoPoolOpenError<InitializationError: error::Error>
{
	/// Could not create a new CTO pool (can not occur if an existing CTO pool exists).
	CreateFailed(PmdkError),
	
	/// An existing CTO pool failed validation with an error.
	ValidationFailed(PmdkError),
	
	/// An existing CTO pool could not be opened.
	OpenFailed(PmdkError),
	
	/// An existing CTO pool is invalid or inconsistent.
	Invalid,
	
	/// After creating or opening a CTO pool, a root object was missing and creation of it was tried. Creation then failed.
	RootCreation(CtoPoolAllocationError<InitializationError>),
}

impl<InitializationError: error::Error> Display for CtoPoolOpenError<InitializationError>
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error>
	{
		use self::CtoPoolOpenError::*;
		
		match *self
		{
			CreateFailed(ref pmdk_error) => Display::fmt(pmdk_error, formatter),
			
			ValidationFailed(ref pmdk_error) => Display::fmt(pmdk_error, formatter),
			
			OpenFailed(ref pmdk_error) => Display::fmt(pmdk_error, formatter),
			
			Invalid => write!(formatter, "Invalid"),
			
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
			CreateFailed(ref pmdk_error) => Some(pmdk_error),
			
			ValidationFailed(ref pmdk_error) => Some(pmdk_error),
			
			OpenFailed(ref pmdk_error) => Some(pmdk_error),
			
			Invalid => None,
			
			RootCreation(ref cto_pool_allocation_error) => Some(cto_pool_allocation_error),
		}
	}
}
