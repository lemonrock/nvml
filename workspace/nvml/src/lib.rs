// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of nvml, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(tyvar_behind_raw_pointer)]
#![deny(missing_docs)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(asm)]
#![feature(attr_literals)]
#![feature(box_into_raw_non_null)]
#![feature(cfg_target_feature)]
#![feature(collections_range)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(exact_size_is_empty)]
#![feature(fused)]
#![feature(i128_type)]
#![feature(inclusive_range)]
#![feature(integer_atomics)]
#![feature(offset_to)]
#![feature(optin_builtin_traits)]
#![feature(pattern)]
#![feature(placement_new_protocol)]
#![feature(platform_intrinsics)]
#![feature(pointer_methods)]
#![feature(repr_align)]
#![feature(shared)]
#![feature(specialization)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(target_feature)]
#![feature(thread_local)]
#![feature(trusted_len)]
#![feature(unicode)]
#![feature(unique)]
#![feature(untagged_unions)]


//! # nvml
//!
//! This crate provides mid-level Rust wrappers for working with persistent memory by wrapping the PMDK, Persistent Memory Development Kit (formerly NVML, and also known as pmem).
//! For more documentation, check [pmem.io](https://pmem.io).
//!
//! Persistent memory is available as 'pools', each of which can be used in a particular way.
//!
//! The most interesting modules are `cto_pool`, `block_pool`  and `log_pool`.
//! The module `persistent_memory` provides low-level support.
//! The module `object_pool` can not work well with Rust.
//!
//! The struct `Configuration` can be used to create and manage several different pools of persistent memory.
//!


extern crate alloc;
#[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate nvml_sys;
pub extern crate parking_lot;
#[macro_use] extern crate quick_error;
#[cfg(not(all(target_feature = "rdrnd", any(target_arch = "x86", target_arch = "x86_64"))))] extern crate rand;
extern crate rust_extra;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate spin_locks;
extern crate std_unicode;
extern crate syscall_alt;


include!("use_path.rs");


/// Block pools are similar to persistent arrays.
pub mod block_pool;

/// CTO pools are equivalent to multiple `malloc`-like (Heap) allocators.
/// Get started with the struct `CtoPool`.
/// Use the method `CtoPool::new()` to create a new instance, and `self.allocator()` to access an allocator that can construct CTO equivalents of Box, Rc, Arc, Mutex, etc.
pub mod cto_pool;

mod errors;

/// Log pools are similar to database or filesystem write logs.
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
include!("IsNotNull.rs");
include!("initialise_memory_functions.rs");
include!("Pools.rs");
