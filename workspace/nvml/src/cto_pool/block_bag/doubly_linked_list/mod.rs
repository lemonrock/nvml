// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use self::rust_extra::*;
use self::tagged_pointers::*;
use ::std::cell::Cell;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ptr::NonNull;
use ::std::ptr::write;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering::AcqRel;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::spin_loop_hint;


mod rust_extra;
mod tagged_pointers;


include!("AtomicLink.rs");
include!("Back_Off.rs");
include!("Cursor.rs");
include!("DereferencedLink.rs");
include!("LockFreeDoublyLinkedListAndDeque.rs");
include!("Node.rs");
include!("StackLink.rs");
