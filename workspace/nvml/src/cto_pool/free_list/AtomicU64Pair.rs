// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Inspired by <https://github.com/kokkos/kokkos/blob/da314444da72d67831e3e71495583378dac9dbb3/core/src/impl/Kokkos_Atomic_Assembly.hpp>
// However, we've added a memory clobber, as:-
// * it seems that we're referencing memory as an output;
// * that is what the HLE intrinsic assembly does for cmpxchg
// Does not use the `u128` type as:-
// * it is unstable
// * it is troublesome to use with cmpxchg16b.
// * there are not other atomic 128-bit operations on x86_64 (eg store, load).
// * must use cases actually want to use a pointer with a counter.
// `cmpxchg16b` requires that the destination (memory) operand [ie `+m`] is 16-byte aligned.
#[repr(C, align(16))]
struct AtomicU64Pair(UnsafeCell<(u64, u64)>);

impl AtomicU64Pair
{
	#[inline(always)]
	fn compare_and_swap_strong(&self, mut compare: (u64, u64), swap: (u64, u64)) -> (bool, (u64, u64))
	{
		// Compares the 128-bit value in RDX:RAX with the operand (destination operand) `m`.
		// If the values are equal, the 128-bit value in RCX:RBX is stored in the destination operand.
		// Otherwise, the value in the destination operand is loaded into RDX:RAX.
		
		// For the RDX:RAX and RCX:RBX register pairs, RDX and RCX contain the high-order 64 bits and RAX and RBX contain the low-order 64bits of a 128-bit value.
		// We assume of a (u64, u64) that high is .1 and low is .0
		
		let mut swapped: bool = false;
		let ptr = self.0.get();
		unsafe
		{
			asm!
			(
				"lock; cmpxchg16b %1; setz %0"
				:
					  "=q" ( swapped )
					, "+m" ( *ptr )
					, "+d" ( compare.1 )
					, "+a" ( compare.0 )
				:
					  "c" ( swap.1 )
					, "b" ( swap.0 )
					, "q" ( swapped )
				:
					"memory"
				:
					"volatile"
			);
		}
		(swapped, compare)
	}
	
	// A good impossible value might be (0, 0) if a counter starts from 1, or (!0, !0), as a counter and pointer combination will never be this, particularly as on x86_64 pointers do not exceed 2^48 - 1 and counters should never reach 2^64 - 1.
	#[inline(always)]
	fn simulate_load(&self, impossible_value: (u64, u64)) -> (u64, u64)
	{
		// NOTE: It is impossible on x86_64 to atomically load a u128 value without a spinlock or at least one CAS.
		// This design ensures that we have a known, good value:-
		// * by specifying an 'old' value that it is impossible for for u128 value to have ever been
		// * by using a 'new' value that is the same as the 'old' (actually this doesn't matter, but using constants makes things easier)
		match self.compare_and_swap_strong(impossible_value, impossible_value)
		{
			(true, _) => panic!("simulate_load failed, because the impossible_value was already present"),
			(false, loaded_value) => loaded_value,
		}
	}
}
