// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::errno::errno;
use ::errors::PmdkError;
use ::libc::c_char;
use ::libc::c_longlong;
use ::libc::c_void;
use ::libc::mode_t;
use ::libc::size_t;
use ::Configuration;
use ::nvml_sys::*;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::collections::HashMap;
use ::std::cmp::min;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
use ::std::path::Path;
use ::std::sync::Arc;
use ::syscall_alt::constants::E;


include!("BlockPool.rs");
include!("BlockPoolConfiguration.rs");
include!("BlockPoolDropWrapper.rs");
include!("BlockPoolsConfiguration.rs");
include!("initialiseMemoryFunctions.rs");
include!("PersistentMemoryBlockPoolPathExt.rs");
include!("PMEMblkpoolEx.rs");
