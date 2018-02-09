// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
use ::std::isize;
use ::std::usize;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::transmute;
use ::std::process::abort;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::Ordering::SeqCst;


include!("CtoArc.rs");
include!("CtoArcCell.rs");
include!("CtoArcInner.rs");
include!("CtoStrongArc.rs");
include!("CtoStrongArcInner.rs");
include!("WeakCtoArc.rs");

