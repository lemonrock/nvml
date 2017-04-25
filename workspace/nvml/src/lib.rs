// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of nvml, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#![allow(non_snake_case)]

extern crate errno;
extern crate libc;
extern crate nvml_sys;
#[macro_use] extern crate quick_error;
extern crate rust_extra;
extern crate syscall_alt;


pub mod logPool;
