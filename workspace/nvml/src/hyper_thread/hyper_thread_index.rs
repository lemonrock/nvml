// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// NOTE: Mac OS X does not support fixed thread affinity.
// NOTE: FreeBSD, NetBSD, OpenBSD and BitRig seem to have no equivalent of `sched_getcpu()`.
// NOTE: Fuschia, Haiku and Redox haven't been investigated.
// NOTE: For all modern x86_64 / x86 CPUs, it is probably possible to use CPUID but it's not straightforward. For example, RocksDb does it but there is not certainty that it handles CPUs newer than Westmere correctly: [See PhysiclCoreID](https://github.com/facebook/rocksdb/blob/aba34097405f076529072fc4cffcba27dd41e73a/port/port_posix.cc) or <https://stackoverflow.com/questions/33745364/sched-getcpu-equivalent-for-os-x>
// NOTE: Also <https://trac.wildfiregames.com/browser/ps/trunk/source/lib/sysdep/arch/x86_x64/topology.cpp>
/// Attempts to return a hyper thread's index.
/// Efficient, as it caches the value in a thread-local variable.
/// Useful only for algorithms requiring a hyper thread index which starts at zero.
/// Does not necessarily map to a CPU number, eg Linux's `sched_getcpu()`.
/// Once assigned for a thread, never changes.
/// Thread death will cause all sorts of problems...
#[inline(always)]
pub fn hyper_thread_index() -> usize
{
	const UninitializedHyperThreadIndex: usize = ::std::usize::MAX;
	
	static NextHyperThreadIndex: AtomicUsize = AtomicUsize::new(0);
	#[thread_local] static mut HyperThreadIndex: usize = UninitializedHyperThreadIndex;
	
	let hyper_thread_index = unsafe { HyperThreadIndex };
	
	if hyper_thread_index == UninitializedHyperThreadIndex
	{
		let current_hyper_thread_index = NextHyperThreadIndex.fetch_add(1, Relaxed);;
		debug_assert_ne!(current_hyper_thread_index, UninitializedHyperThreadIndex, "Too many hyper threads");
		assert!(current_hyper_thread_index < MaximumSupportedHyperThreads, "The current_hyper_thread_index '{}' equals or exceeds the compiled maximum MaximumSupportedHyperThreads '{}'", current_hyper_thread_index, MaximumSupportedHyperThreads);
		assert!(current_hyper_thread_index < MaximumSupportedHyperThreads, "The current_hyper_thread_index '{}' equals or exceeds the maximum_number_of_hyper_threads '{}'", current_hyper_thread_index, maximum_number_of_hyper_threads());
		unsafe { HyperThreadIndex = current_hyper_thread_index };
		current_hyper_thread_index
	}
	else
	{
		hyper_thread_index
	}
}
