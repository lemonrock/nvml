// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use ::errors::PmdkError;
use ::libc::c_char;
use ::libc::c_void;
use ::libc::mode_t;
use ::libc::size_t;
use ::libc::wchar_t;
use ::nvml_sys::*;
use ::rust_extra::unlikely;
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::cell::Cell;
use ::std::cmp::min;
use ::std::cmp::Ordering;
use ::std::error;
use ::std::ffi::CString;
use ::std::ffi::CStr;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Pointer;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::heap::Alloc;
use ::std::heap::AllocErr;
use ::std::heap::Layout;
use ::std::mem::align_of;
use ::std::mem::forget;
use ::std::mem::needs_drop;
use ::std::mem::size_of;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
use ::std::process::abort;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::drop_in_place;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::std::path::Path;
use ::std::sync::Arc;
use ::std::sync::RwLock;


include!("CtoBox.rs");
include!("CtoPool.rs");
include!("CtoPoolAllocationError.rs");
include!("CtoPoolAllocator.rs");
include!("CtoPoolInner.rs");
include!("CtoPoolOpenError.rs");
include!("CtoPoolPathExt.rs");
include!("CtoRc.rs");
include!("CtoRcCounter.rs");
include!("CtoRcInner.rs");
include!("CtoRootBox.rs");
include!("CtoSafe.rs");
include!("initialise_memory_functions.rs");
include!("PMEMctopoolExt.rs");
include!("WeakCtoRc.rs");
