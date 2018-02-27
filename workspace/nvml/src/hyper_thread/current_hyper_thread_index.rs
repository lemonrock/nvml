// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// NOTE: AIX: `mycpu()`.
// NOTE: BlueGene/Q: `Kernel_ProcessorID()`.

//noinspection SpellCheckingInspection
/// Returns the current hyper thread index (also known as logical CPU number).
/// No reliable implementation exists for Mac OS X as it does not support pinned thread affinity.
/// No currently known solution exists for the BSDs except for DragonFly BSD.
/// Redox is currently treated as single-hyper-threaded.
#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn current_hyper_thread_index() -> usize
{
	use ::libc::sched_getcpu;

	let result = unsafe { sched_getcpu() };
	debug_assert!(result >= 0, "sched_getcpu() was negative");
	result as usize
}

// NOTE: Not present in Rust libc as of this time
//noinspection SpellCheckingInspection
/// Returns the current hyper thread index (also known as logical CPU number).
/// No reliable implementation exists for Mac OS X as it does not support pinned thread affinity.
/// No currently known solution exists for the BSDs except for DragonFly BSD.
/// Redox is currently treated as single-hyper-threaded.
#[cfg(any(target_os = "dragonfly"))]
fn current_hyper_thread_index() -> usize
{
	extern "C"
	{
		fn sched_getcpu() -> ::libc::c_int;
	}

	let result = unsafe { sched_getcpu() };
	debug_assert!(result >= 0, "sched_getcpu() was negative");
	result as usize
}

//noinspection SpellCheckingInspection
/// Returns the current hyper thread index (also known as logical CPU number).
/// No reliable implementation exists for Mac OS X as it does not support pinned thread affinity.
/// No currently known solution exists for the BSDs except for DragonFly BSD.
/// Redox is currently treated as single-hyper-threaded.
#[cfg(target_os = "solaris")]
fn current_hyper_thread_index() -> usize
{
	// sys/processor.h
	type processorid_t = ::libc::c_int;
	extern "C"
	{
		fn getcpuid() -> processorid_t;
	}

	let result = unsafe { getcpuid() };
	debug_assert!(result >= 0, "getcpuid() was negative");
	result as usize
}

/// Returns the current hyper thread index (also known as logical CPU number).
/// No reliable implementation exists for Mac OS X as it does not support pinned thread affinity.
/// No currently known solution exists for the BSDs except for DragonFly BSD.
/// Redox is currently treated as single-hyper-threaded.
#[cfg(target_os = "windows")]
fn current_hyper_thread_index() -> usize
{
	use ::kernel32::GetCurrentProcessorNumberEx;
	use ::winapi::winnt::PROCESSOR_NUMBER;

	let mut processor_number: PROCESSOR_NUMBER = unsafe { uninitialized() };
	unsafe { GetCurrentProcessorNumberEx(&mut processor_number) };

	(processor_number.GROUP * 64 + (processor_number.Number as u16)) as usize
}

/// Returns the current hyper thread index (also known as logical CPU number).
/// No reliable implementation exists for Mac OS X as it does not support pinned thread affinity.
/// No currently known solution exists for the BSDs except for DragonFly BSD.
/// Redox is currently treated as single-hyper-threaded.
#[cfg(any(target_os = "emscripten", target_os = "haiku", target_os = "redox"))]
fn current_hyper_thread_index() -> usize
{
	0
}

/// Returns the current hyper thread index (also known as logical CPU number).
/// No reliable implementation exists for Mac OS X as it does not support pinned thread affinity.
/// No currently known solution exists for the BSDs except for DragonFly BSD.
/// Redox is currently treated as single-hyper-threaded.
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
fn current_hyper_thread_index() -> usize
{
	0
}
