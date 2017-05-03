// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::errno::Errno;
use ::errno::errno;
use ::errno::set_errno;
use ::errors::GenericError;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::mode_t;
use ::libc::size_t;
use ::libc::timespec;
use ::nvml_sys::*;
use ::nvml_sys::pobj_tx_param::TX_PARAM_NONE;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::any::Any;
use ::std::ffi::CString;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::mem::zeroed;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::panic::resume_unwind;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
use ::std::path::Path;
use ::std::ptr::null;
use ::std::sync::Arc;
use ::syscall_alt::constants::E;


include!("ConditionVariable.rs");
include!("ConditionVariableMutexLockablePersistentObjectMemory.rs");
include!("initialiseMemoryFunctions.rs");
include!("MutexLock.rs");
include!("MutexLockablePersistentObjectMemory.rs");
include!("ReadWriteLockablePersistentObjectMemory.rs");
include!("MutexUnlock.rs");
include!("ObjectPool.rs");
include!("ObjectPoolDropWrapper.rs");
include!("ObjectPoolPersistOnDrop.rs");
include!("OID.rs");
include!("OidWrapper.rs");
include!("Persistable.rs");
include!("PersistentMemoryObjectPoolPathExt.rs");
include!("PersistentObjectMemory.rs");
include!("PMEMobjpoolEx.rs");
include!("ReadWriteLock.rs");
include!("ReadLockUnlock.rs");
include!("TypeNumber.rs");
include!("Transaction.rs");
include!("WriteLockUnlock.rs");