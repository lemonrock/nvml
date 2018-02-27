// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.



#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
pub fn CLWB()
{
	asm!();
}


/// Executions of the `CLFLUSHOPT` instruction are ordered with respect to fence instructions and to locked read-modify-write instructions; they are also ordered with respect to the following accesses to the cache line being invalidated: writes, executions of `CLFLUSH`, and executions of `CLFLUSHOPT`.
/// They are not ordered with respect to writes, executions of `CLFLUSH`, or executions of `CLFLUSHOPT` that access other cache lines; to enforce ordering with such an operation, software can insert an `SFENCE` instruction between `CFLUSHOPT` and that operation.
///
/// Executing `CLFLUSHOPT` will usually cause a TSX abort.
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
pub fn CLFLUSHOPT()
{
	asm!();
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[inline(always)]
pub fn CLFLUSH()
{
	asm!();
}
