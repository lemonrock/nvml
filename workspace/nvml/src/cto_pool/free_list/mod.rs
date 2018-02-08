// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use IsNotNull;
use super::CtoPoolArc;
use super::CtoSafe;
use super::PMEMctopoolExt;
#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] use ::rand::Rng;
#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] use ::rand::thread_rng;
use ::spin_locks::BestSpinLockForCompilationTarget;
use ::spin_locks::SpinLock;
use ::std::cell::UnsafeCell;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::ptr::replace;
use ::std::ptr::write;
use ::std::sync::atomic::AtomicPtr;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;


include!("AlignedVariableLengthArray.rs");
include!("AtomicIsolationSize.rs");
include!("AtomicPointerAndCounter.rs");
include!("AtomicU64Pair.rs");
include!("BackOffState.rs");
include!("EliminationArray.rs");
include!("EliminationArrayCacheLine.rs");
include!("EliminationArrayEntry.rs");
include!("EliminationArrayLength.rs");
include!("ExponentialBackOffState.rs");
include!("FreeList.rs");
include!("FreeListElement.rs");
include!("generate_thread_safe_random_usize.rs");
include!("InitializedFreeListElement.rs");
include!("MaximumNumberOfFreeListElementPointersThatFitInACacheLine.rs");
include!("OwnedFreeListElement.rs");
include!("PointerAndCounter.rs");
