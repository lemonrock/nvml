// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use ::std::marker::PhantomData;
use ::std::mem::replace;
use ::std::mem::transmute;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ptr::NonNull;
use ::std::ptr::null;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering::AcqRel;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Release;


include!("Back_Off.rs");
include!("hint_core_should_pause.rs");
include!("IsNotNull.rs");
include!("LockFreeDoublyLinkedListAndDeque.rs");
include!("LockFreeDoublyLinkedListAndDequeCursor.rs");
include!("Node.rs");
include!("TaggedPointerToNode.rs");
