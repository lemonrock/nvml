// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[cfg(all(target_feature = "rdrnd", target_arch = "x86"))]
#[inline(always)]
fn generate_thread_safe_random_usize() -> usize
{
	// See https://github.com/rust-lang/rust/tree/master/src/etc/platform-intrinsics/x86
	extern "platform-intrinsic"
	{
		#[inline(always)]
		fn x86_rdrand32_step() -> (u32, i32);
	}

	#[target_feature(enable = "rdrnd")]
	unsafe fn generate_thread_safe_random_usize_target_feature() -> usize
	{
		loop
		{
			let (random_value, success) = x86_rdrand32_step();
			if success != 0
			{
				return random_value as usize
			}
		}
	}

	unsafe { generate_thread_safe_random_usize_target_feature() }
}

#[cfg(all(target_feature = "rdrnd", target_arch = "x86_64"))]
#[inline(always)]
fn generate_thread_safe_random_usize() -> usize
{
	// See https://github.com/rust-lang/rust/tree/master/src/etc/platform-intrinsics/x86
	extern "platform-intrinsic"
	{
		#[inline(always)]
		fn x86_rdrand64_step() -> (u64, i32);
	}

	#[target_feature(enable = "rdrnd")]
	unsafe fn generate_thread_safe_random_usize_target_feature() -> usize
	{
		loop
		{
			let (random_value, success) = x86_rdrand64_step();
			if success != 0
			{
				return random_value as usize
			}
		}
	}

	unsafe { generate_thread_safe_random_usize_target_feature() }
}

#[cfg(all(target_pointer_width = "32", not(all(target_feature = "rdrnd", target_arch = "x86"))))]
fn generate_thread_safe_random_usize() -> usize
{
	thread_rng().next_u32() as usize
}

#[cfg(all(target_pointer_width = "64", not(all(target_feature = "rdrnd", target_arch = "x86_64"))))]
fn generate_thread_safe_random_usize() -> usize
{
	thread_rng().next_u64() as usize
}
