// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use ::std::marker::PhantomData;
use ::std::ptr::null;
use ::std::ptr::write;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;


include!("BackOff.rs");
include!("hint_core_should_pause.rs");
include!("IsNotNull.rs");


/// Implementation based on the paper: "Lock-free deques and doubly linked lists", by Håkan Sundell and Philippas Tsigas, 2008
x

