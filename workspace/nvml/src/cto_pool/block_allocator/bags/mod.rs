// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use persistent_memory::persistence::Persistence;
use super::AtomicBlockPointer;
use super::Block;
use super::BlockMetaData;
use super::BlockMetaDataItems;
use super::BlockPointer;
use super::ChainLength;
use ::std::cmp::max;
use ::std::mem::uninitialized;
use ::std::ptr::write;
use ::std::sync::atomic::*;
use ::std::sync::atomic::Ordering::*;


include!("AtomicChainLengthAndBagStripeIndex.rs");
include!("Bag.rs");
include!("Bags.rs");
include!("BagStripe.rs");
include!("BagStripeArrayLength.rs");
include!("BagStripeIndex.rs");
include!("BagStripeIndexCounter.rs");
include!("ChainLengthAndBagStripeIndex.rs");
include!("InclusiveMaximumChainLength.rs");
include!("RemovalCounter.rs");
include!("u10.rs");
include!("u5.rs");
