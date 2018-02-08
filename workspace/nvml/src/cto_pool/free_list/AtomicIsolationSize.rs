// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Architecture names based on https://github.com/rust-lang/rust/blob/90eb44a5897c39e3dff9c7e48e3973671dcd9496/src/librustc_trans/abi.rs#L925
// If changing this code, all places that use architecture alignment must be changes to, as rust does not allow the use of constants in #[repr()] statements.
// Currently this is `AlignedVariableLengthArray` and `FreeList`
// Note that the following can not be supported:-
// * msp430
// * asmjs

// nvptx, wasm32 and hexagon are guesses.
#[cfg(any(target_arch = "x86", target_arch = "mips", target_arch = "sparc", target_arch = "nvptx", target_arch = "wasm32", target_arch = "hexagon"))] const AtomicIsolationSize: usize = 32;

// s390x and nvptx64 are guesses.
#[cfg(any(target_arch = "mips64", target_arch = "sparc64", target_arch = "s390x"))] const AtomicIsolationSize: usize = 64;

// Brings over two cache lines at once.
// powerpc (32-bit) and powerpc64 are not particularly certain.
#[cfg(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "powerpc64"))] const AtomicIsolationSize: usize = 128;

// Terrible.
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))] const AtomicIsolationSize: usize = 2048;
