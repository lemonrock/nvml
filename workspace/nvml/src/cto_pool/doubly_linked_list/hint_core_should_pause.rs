// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Provided as ::std::sync::atomic::hint_core_should_pause is very unstable.
#[inline(always)]
fn hint_core_should_pause()
{
	#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
	unsafe
	{
		asm!("pause" ::: "memory" : "volatile");
	}
	
	#[cfg(target_arch = "aarch64")]
	unsafe
	{
		asm!("yield" ::: "memory" : "volatile");
	}
}
