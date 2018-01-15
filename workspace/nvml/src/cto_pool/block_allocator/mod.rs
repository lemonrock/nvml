// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
use super::super::persistent_memory::persistence::Persistence;
use ::libc::c_void;
use ::std::collections::BTreeMap;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::ptr::write_bytes;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;
use ::std::sync::atomic::AtomicPtr;
use ::std::sync::atomic::AtomicU8;
use ::std::sync::atomic::Ordering;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Acquire;


include!("Block.rs");
include!("BlockAllocator.rs");
include!("Chain.rs");
include!("ChainMetadata.rs");
include!("Chains.rs");
include!("Chain.rs");
include!("PointerToNextChainOrLock.rs");
include!("RestartCopyAt.rs");
include!("VariableLengthArray.rs");
