// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
use ::alloc::raw_vec::RawVec;
use ::std::collections::Bound::Included;
use ::std::collections::Bound::Excluded;
use ::std::collections::Bound::Unbounded;
use ::std::collections::range::RangeArgument;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::swap;
use ::std::intrinsics::arith_offset;
use ::std::intrinsics::assume;
use ::std::iter::FusedIterator;
use ::std::iter::TrustedLen;
use ::std::ops::*;
use ::std::ptr::copy;
use ::std::ptr::copy_nonoverlapping;
use ::std::ptr::drop_in_place;
use ::std::ptr::read;
use ::std::ptr::write;
use ::std::slice;
use ::std::slice::from_raw_parts;
use ::std::slice::from_raw_parts_mut;


include!("CtoVec.rs");
include!("CtoVecDrain.rs");
include!("CtoVecDrainFilter.rs");
include!("CtoVecIntoIter.rs");
include!("CtoVecPlaceBack.rs");
include!("CtoVecSplice.rs");
include!("SetLenOnDrop.rs");
include!("SpecExtend.rs");
