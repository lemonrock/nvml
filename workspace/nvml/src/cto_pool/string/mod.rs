// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


//! A UTF-8 encoded, growable string.
//!
//! This module contains the [`String`] type, a trait for converting
//! [`ToString`]s, and several error types that may result from working with
//! [`String`]s.
//!
//! [`ToString`]: trait.ToString.html
//!
//! # Examples
//!
//! There are multiple ways to create a new [`String`] from a string literal:
//!
//! ```
//! let s = "Hello".to_string();
//!
//! let s = String::from("world");
//! let s: String = "also this".into();
//! ```
//!
//! You can create a new [`String`] from an existing one by concatenating with
//! `+`:
//!
//! [`String`]: struct.String.html
//!
//! ```
//! let s = "Hello".to_string();
//!
//! let message = s + " world!";
//! ```
//!
//! If you have a vector of valid UTF-8 bytes, you can make a [`String`] out of
//! it. You can do the reverse too.
//!
//! ```
//! let sparkle_heart = vec![240, 159, 146, 150];
//!
//! // We know these bytes are valid, so we'll use `unwrap()`.
//! let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
//!
//! assert_eq!("💖", sparkle_heart);
//!
//! let bytes = sparkle_heart.into_bytes();
//!
//! assert_eq!(bytes, [240, 159, 146, 150]);
//! ```


use super::*;
use ::std::borrow::Cow;
use ::std::collections::Bound::Excluded;
use ::std::collections::Bound::Included;
use ::std::collections::Bound::Unbounded;
use ::std::collections::range::RangeArgument;
use ::std::iter::FusedIterator;
use ::std::ops::*;
use ::std::str::Chars;
use ::std::ptr::copy;
use ::std::str::from_utf8_unchecked;
use ::std::str::from_utf8_unchecked_mut;
use ::std::str::pattern::Pattern;


include!("CtoStringDrain.rs");
include!("CtoString.rs");
