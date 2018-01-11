// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
use super::arc::CtoArcCell;
use ::parking_lot::Condvar;
use ::parking_lot::Mutex;
use ::parking_lot::MutexGuard;
use ::parking_lot::ReentrantMutex;
use ::parking_lot::RwLock;
use ::std::cell::Cell;
use ::std::cell::UnsafeCell;
use ::std::mem::forget;
use ::std::mem::uninitialized;
use ::std::sync::atomic::AtomicU8;
use ::std::sync::atomic::AtomicUsize;


include!("CtoParkingLotConditionVariable.rs");
include!("CtoParkingLotMutexLock.rs");
include!("CtoParkingLotReadWriteLock.rs");
include!("CtoParkingLotReentrantMutexLock.rs");
include!("ReadCopyUpdateLock.rs");
include!("ReadCopyUpdateLockWriteGuard.rs");
