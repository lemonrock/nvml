// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::super::persistent_memory::persistence::Persistence;
use ::std::cmp::max;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::sync::atomic::*;
use ::std::sync::atomic::Ordering::*;


include!("AtomicBlockPointer.rs");
include!("AtomicChainLengthAndBagStripeIndex.rs");
include!("Block.rs");
include!("BlockMetaData.rs");
include!("BlockPointer.rs");
include!("Bag.rs");
include!("Bags.rs");
include!("BagStripe.rs");
include!("BagStripeIndex.rs");
include!("BagStripeIndexCounter.rs");
include!("ChainLength.rs");
include!("ChainLengthAndBagStripeIndex.rs");
include!("RemovalCounter.rs");
include!("u10.rs");
include!("u5.rs");


const InclusiveMaximumChainLength: usize = 1024;

const BagStripeArrayLength: usize = 32;
