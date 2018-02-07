// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.

// See https://github.com/rust-lang/rust/tree/master/src/etc/platform-intrinsics/x86

#[cfg(target_arch = "x86")]
#[inline(always)]
fn generate_thread_safe_random_usize() -> usize
{
	#[cfg(feature = "rdrand")]
	extern "platform-intrinsic"
	{
		#[inline(always)]
		fn x86_rdrand32_step() -> (u32, i32);
	}
	
	#[cfg(feature = "rdrand")]
	#[target_feature(enable = "rdrnd")]
	loop
	{
		let (random_value, success) = unsafe { x86_rdrand32_step() };
		if success != 0
		{
			return random_value as usize
		}
	}
	
	#[cfg(not(feature = "rdrand"))]
	{
		thread_rng().next_u32() as usize
	}
}

// #[cfg(target_feature)]
#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn generate_thread_safe_random_usize() -> usize
{
	#[cfg(feature = "rdrand")]
	extern "platform-intrinsic"
	{
		#[inline(always)]
		fn x86_rdrand64_step() -> (u64, i32);
	}
	
	#[cfg(feature = "rdrand")]
	#[target_feature(enable = "rdrnd")]
	loop
	{
		let (random_value, success) = unsafe { x86_rdrand64_step() };
		if success != 0
		{
			return random_value as usize
		}
	}
	
	#[cfg(not(feature = "rdrand"))]
	{
		thread_rng().next_u64() as usize
	}
}

#[cfg(all(target_pointer_width = "32", not(any(target_arch = "x86_64", target_arch = "x86"))))]
fn generate_thread_safe_random_usize() -> usize
{
	thread_rng().next_u32() as usize
}

#[cfg(all(target_pointer_width = "64", not(any(target_arch = "x86_64", target_arch = "x86"))))]
fn generate_thread_safe_random_usize() -> usize
{
	thread_rng().next_u64() as usize
}
