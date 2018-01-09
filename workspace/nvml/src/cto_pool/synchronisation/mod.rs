// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
#[cfg(unix)] use ::libc::
{
	EAGAIN,
	EDEADLK,
	ETIMEDOUT,
	pthread_cond_t,
	pthread_cond_broadcast,
	pthread_cond_destroy,
	pthread_cond_signal,
	pthread_cond_timedwait,
	pthread_cond_wait,
	PTHREAD_COND_INITIALIZER,
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
	pthread_rwlock_destroy,
	pthread_rwlock_rdlock,
	pthread_rwlock_t,
	pthread_rwlock_tryrdlock,
	pthread_rwlock_trywrlock,
	pthread_rwlock_unlock,
	pthread_rwlock_wrlock,
	PTHREAD_RWLOCK_INITIALIZER,
	timespec,
	time_t,
};
#[cfg(target_os = "dragonfly")] use ::libc::EINVAL;
use ::std::cell::UnsafeCell;
use ::std::cmp::min;
use ::std::mem::uninitialized;
use ::std::panic::UnwindSafe;
use ::std::panic::RefUnwindSafe;
use ::std::sync::atomic::AtomicUsize;
//use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering;
use ::std::sync::atomic::Ordering::Relaxed;
//use ::std::sync::atomic::Ordering::Release;
//use ::std::sync::atomic::Ordering::SeqCst;
use ::std::time::Duration;


include!("debug_assert_pthread_result_ok.rs");
include!("debug_assert_pthread_result_ok_dragonfly.rs");


//include!("CtoArc.rs");
//include!("CtoArcInner.rs");
include!("CtoConditionVariable.rs");
include!("CtoConditionVariableInner.rs");
include!("CtoMutexLock.rs");
include!("CtoMutexLockAndConditionVariable.rs");
include!("CtoMutexLockInner.rs");
include!("CtoMutexLockGuard.rs");
include!("CtoMutexLockGuardWithConditionVariable.rs");
include!("CtoReadWriteLock.rs");
include!("CtoReadWriteLockInner.rs");
include!("CtoReadWriteLockReadGuard.rs");
include!("CtoReadWriteLockWriteGuard.rs");
include!("ResultIsOk.rs");
include!("TimedOut.rs");
