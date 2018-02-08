// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Uses align of AtomicIsolationSize.
#[cfg_attr(any(target_arch = "x86", target_arch = "mips", target_arch = "sparc", target_arch = "nvptx", target_arch = "wasm32", target_arch = "hexagon"), repr(C, align(32)))]
#[cfg_attr(any(target_arch = "mips64", target_arch = "sparc64", target_arch = "s390x"), repr(C, align(64)))]
#[cfg_attr(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "powerpc64"), repr(C, align(128)))]
#[cfg_attr(any(target_arch = "arm", target_arch = "aarch64"), repr(C, align(2048)))]
#[derive(Debug)]
struct AlignedVariableLengthArray<T>(PhantomData<EliminationArrayCacheLine<T>>);
