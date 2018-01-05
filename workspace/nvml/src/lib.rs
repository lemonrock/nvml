// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of nvml, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![feature(allocator_api)]
#![feature(const_fn)]
#![feature(optin_builtin_traits)]
#![feature(specialization)]
#![feature(thread_local)]
#![feature(unique)]
#![feature(untagged_unions)]


//! # nvml
//!
//! This crate provides mid-level Rust wrappers for working with persistent memory by wrapping the PMDK, Persistent Memory Development Kit (formerly NVML, and also known as pmem).
//! For more documentation, check [pmem.io](https://pmem.io).
//!


#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate nvml_sys;
#[macro_use] extern crate quick_error;
extern crate rust_extra;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate syscall_alt;


include!("offset_of.rs");
include!("use_path.rs");


/// Block pools are similar to persistent arrays
pub mod block_pool;

/// CTO pools are equivalent to multiple `malloc`-like (Heap) allocators
pub mod cto_pool;

mod errors;

/// Log pools are similar to database or filesystem write logs
pub mod log_pool;

/// Object pools allow the storage of arbitrary objects, arranged in graphs or lists, with support for Mutexes, Read-Write locks, condition variables and transactions.
/// Transactions can not work on Rust currently as they use `setjmp/longjmp`.
pub mod object_pool;

/// Basic abstractions supporting the use of persistent memory.
/// Essential if building alternatives to the block, cto, log or object pools.
pub mod persistent_memory;


use ::block_pool::BlockPool;
use ::block_pool::BlockPoolsConfiguration;
use ::libc::mode_t;
use ::log_pool::LogPool;
use ::log_pool::LogPoolsConfiguration;
use ::object_pool::ObjectPool;
use ::object_pool::ObjectPoolsConfiguration;
use ::rust_extra::unlikely;
use ::std::collections::HashMap;
use ::std::path::Path;


include!("Configuration.rs");
include!("initialise_memory_functions.rs");
include!("Pools.rs");
