// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) trait IsNotNull
{
	#[inline(always)]
	fn is_not_null(self) -> bool;
}

impl<T> IsNotNull for *const T
{
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}

impl<T> IsNotNull for *mut T
{
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		!self.is_null()
	}
}
