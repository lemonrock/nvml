// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
#[cfg(unix)] use ::libc::
{
	pthread_mutex_destroy,
	pthread_mutex_init,
	pthread_mutex_lock,
	pthread_mutex_trylock,
	pthread_mutex_t,
	pthread_mutex_unlock,
	pthread_mutexattr_destroy,
	pthread_mutexattr_t,
	pthread_mutexattr_init,
	pthread_mutexattr_settype,
	PTHREAD_MUTEX_NORMAL,
	PTHREAD_MUTEX_INITIALIZER,
};
use ::std::cell::UnsafeCell;
use ::std::mem::uninitialized;
use ::std::panic::UnwindSafe;
use ::std::panic::RefUnwindSafe;


include!("CtoMutexLock.rs");
include!("CtoMutexLockGuard.rs");
