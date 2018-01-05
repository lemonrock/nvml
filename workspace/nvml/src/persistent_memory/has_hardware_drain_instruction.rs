// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Does the hardware have a hardware drain instruction? (Most Intel CPUs from 2015 onwards do).
#[deprecated(note = "Always false, and not useful to know as a fence is still needed, ie calls to drain_after_flush() can not be avoided")]
#[inline(always)]
pub fn has_hardware_drain_instruction() -> bool
{
	let result = unsafe { pmem_has_hw_drain() };
	if likely(result == 0)
	{
		false
	}
	else if likely(result == 1)
	{
		true
	}
	else
	{
		panic!("pmem_has_hw_drain() returned value '{}', not 0 or 1", result);
	}
}
