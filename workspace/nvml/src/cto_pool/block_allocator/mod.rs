// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
use super::super::persistent_memory::persistence::Persistence;
use super::arc::CtoStrongArc;
use super::arc::CtoStrongArcInner;
use self::bags::*;
use ::libc::c_void;
use ::std::cell::Cell;
use ::std::marker::PhantomData;
use ::std::mem::size_of;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::drop_in_place;
use ::std::ptr::NonNull;
use ::std::ptr::write;
use ::std::sync::atomic::*;
use ::std::sync::atomic::Ordering::*;


mod bags;


include!("AtomicBlockPointer.rs");
include!("Block.rs");
include!("BlockAllocator.rs");
include!("BlockMetaData.rs");
include!("BlockMetaDataItems.rs");
include!("BlockPointer.rs");
include!("Chain.rs");
include!("ChainLength.rs");
include!("Chains.rs");
include!("NonNullExt.rs");
include!("PowerOfTwo.rs");
include!("RestartCopyFromAt.rs");
include!("RestartCopyIntoAt.rs");
