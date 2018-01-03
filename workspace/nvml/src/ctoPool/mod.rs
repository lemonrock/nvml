// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use ::either::Either;
use ::either::Either::*;
use ::errno::errno;
use ::errors::GenericError;
use ::libc::c_char;
use ::libc::c_void;
use ::libc::mode_t;
use ::libc::size_t;
use ::libc::wchar_t;
use ::nvml_sys::*;
use ::rust_extra::unlikely;
use ::std::borrow::Borrow;
use ::std::borrow::BorrowMut;
use ::std::cmp::Ordering;
use ::std::ffi::CStr;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Pointer;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::forget;
use ::std::mem::align_of;
use ::std::mem::size_of;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::null;
use ::std::path::Path;
use ::std::sync::Arc;


include!("handleError.rs");


include!("CtoBox.rs");
include!("CtoPool.rs");
include!("CtoPoolDropWrapper.rs");
include!("CtoSafe.rs");
include!("initialiseMemoryFunctions.rs");
include!("PersistentMemoryCtoPoolPathExt.rs");
include!("PMEMctopoolEx.rs");