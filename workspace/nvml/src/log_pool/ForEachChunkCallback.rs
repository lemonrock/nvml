// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Type of callback function pointer used when walking the log.
/// Called for each chunk.
pub type ForEachChunkCallback = unsafe extern "C" fn(chunk: *const c_void, length: usize, callback_argument: *mut c_void) -> WalkCallbackResult;
