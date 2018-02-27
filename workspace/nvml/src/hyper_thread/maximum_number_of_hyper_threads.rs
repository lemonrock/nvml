// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// NOTE: We do not use the `num_cpus` crate, because it gets the number of CPUs from thread affinity - and our thread may already have had its affinity set.
// On Linux glibc & musl, there are also the functions `get_nprocs()` and `get_nprocs_conf()` but these either use `/sys` (glibc) or delegate to `sysconf()` (musl).
/// Maximum number of hyper threads.
/// Stays constant throughout program execution.
#[inline(always)]
pub fn maximum_number_of_hyper_threads() -> usize
{
	#[cfg(target_os = "windows")]
	fn current_number_of_hyper_threads() -> usize
	{
		use ::winapi::um::sysinfoapi::GetSystemInfo;
		use ::winapi::um::sysinfoapi::SYSTEM_INFO;
		
		let mut lpSystemInfo: SYSTEM_INFO = unsafe { uninitialized() };
		unsafe { GetSystemInfo(&mut lpSystemInfo) };
		
		lpSystemInfo.dwNumberOfProcessors as usize
	}
	
	#[cfg(any(target_os = "android", target_os = "fuchsia", target_os = "ios", target_os = "linux", target_os = "macos", target_os = "nacl", target_os = "solaris"))]
	fn current_number_of_hyper_threads() -> usize
	{
		let result = sysconf_current_number_of_hyper_threads();
		if result > 0
		{
			return result as usize;
		}
		else
		{
			1
		}
	}
	
	#[cfg(any(target_os = "bitrig", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd"))]
	fn current_number_of_hyper_threads() -> usize
	{
		let result = sysconf_maximum_number_of_hyper_threads();
		if result > 0
		{
			return result as usize;
		}
		else
		{
			sysctl_current_number_of_hyper_threads()
		}
	}
	
	#[cfg(target_os = "openbsd")]
	fn current_number_of_hyper_threads() -> usize
	{
		sysctl_maximum_number_of_hyper_threads()
	}
	
	#[cfg(any(target_os = "emscripten", target_os = "haiku", target_os = "redox"))]
	fn current_number_of_hyper_threads() -> usize
	{
		1
	}
	
	#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
	fn current_number_of_hyper_threads() -> usize
	{
		1
	}
	
	#[cfg(any(target_os = "android", target_os = "bitrig", target_os = "dragonfly", target_os = "freebsd", target_os = "fuchsia", target_os = "ios", target_os = "linux", target_os = "macos",target_os = "nacl", target_os = "netbsd", target_os = "openbsd", target_os = "solaris"))]
	fn sysconf_current_number_of_hyper_threads() -> ::libc::c_long
	{
		use ::libc::c_int;
		use ::libc::sysconf;
		
		// On ARM targets, processors can be temporarily turned off to save power.
		// On other platforms, this is unlikely to be the case.
		#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
		const SysConfKey: c_int = ::libc::_SC_NPROCESSORS_CONF;
		
		#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
		const SysConfKey: c_int = ::libc::_SC_NPROCESSORS_ONLN;
		
		unsafe { sysconf(SysConfKey) }
	}
	
	#[cfg(any(target_os = "bitrig", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))]
	fn sysctl_current_number_of_hyper_threads() -> usize
	{
		use ::libc::c_uint;
		use ::libc::CTL_HW;
		use ::libc::HW_NCPU;
		use ::libc::sysctl;
		use ::std::size_of;
		
		let mut mib = [CTL_HW, HW_NCPU, 0, 0];
		let mut cpus: c_uint = unsafe { unintialized() };
		let mut cpus_size = size_of::<c_uint>();
		unsafe { sysctl(mib.as_mut_ptr(), 2, &mut cpus as *mut _ as *mut _, &mut cpus_size as *mut _ as *mut _, 0 as *mut _, 0) };
		if cpus > 0
		{
			cpus as usize
		}
		else
		{
			1
		}
	}
	
	const UninitializedMaximumNumberOfHyperThreads: usize = ::std::usize::MAX;
	
	static MaximumNumberOfHyperThreads: AtomicUsize = AtomicUsize::new(UninitializedMaximumNumberOfHyperThreads);
	
	let maximum_number_of_hyper_threads = MaximumNumberOfHyperThreads.load(Relaxed);
	if maximum_number_of_hyper_threads == UninitializedMaximumNumberOfHyperThreads
	{
		let current_number_of_hyper_threads = current_number_of_hyper_threads();
		debug_assert_ne!(current_number_of_hyper_threads, 0, "The current_number_of_hyper_threads is zero");
		debug_assert_ne!(current_number_of_hyper_threads, UninitializedMaximumNumberOfHyperThreads, "The current_number_of_hyper_threads is bonkers");
		assert!(current_number_of_hyper_threads <= MaximumSupportedHyperThreads, "The current_number_of_hyper_threads '{}' exceeds the compiled maximum MaximumSupportedHyperThreads '{}'", current_number_of_hyper_threads, MaximumSupportedHyperThreads);
		MaximumNumberOfHyperThreads.compare_and_swap(UninitializedMaximumNumberOfHyperThreads, current_number_of_hyper_threads, Relaxed)
	}
	else
	{
		maximum_number_of_hyper_threads
	}
}
