// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct CtoConditionVariableInner
{
	#[cfg(unix)] cond: UnsafeCell<pthread_cond_t>,
}

impl Drop for CtoConditionVariableInner
{
	#[inline(always)]
	fn drop(&mut self)
	{
		#[cfg(unix)]
		{
			let result = unsafe { pthread_cond_destroy(self.cond.get()) };
			
			#[cfg(not(target_os = "dragonfly"))]
			{
				debug_assert_pthread_result_ok!(result);
			}
			
			#[cfg(target_os = "dragonfly")]
			{
				// On DragonFly `pthread_cond_destroy()` returns `EINVAL` if called on a condition variable that was just initialized with `PTHREAD_COND_INITIALIZER`.
				// Once it is used or `pthread_cond_init()` is called, this behaviour no longer occurs.
				debug_assert_pthread_result_ok_dragonfly!(result);
			}
		}
	}
}

impl CtoSafe for CtoConditionVariableInner
{
}

unsafe impl Send for CtoConditionVariableInner
{
}

unsafe impl Sync for CtoConditionVariableInner
{
}

impl CtoConditionVariableInner
{
	#[inline(always)]
	fn new() -> Box<Self>
	{
		#[cfg(unix)]
		let this =
		{
			Box::new
			(
				Self
				{
					cond: UnsafeCell::new(PTHREAD_COND_INITIALIZER),
				}
			)
		};
		
		// Must happen after we are at a consistent memory location.
		#[cfg(unix)]
		{
			// See notes for `timed_wait()`.
			#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "l4re", target_os = "android")))]
			unsafe
			{
				use ::libc::CLOCK_MONOTONIC;
				use ::libc::pthread_condattr_init;
				use ::libc::pthread_condattr_setclock;
				use ::libc::pthread_cond_init;
				use ::libc::pthread_condattr_destroy;
				use ::libc::pthread_condattr_t;
				
				let mut attr: pthread_condattr_t = uninitialized();
				
				let result = pthread_condattr_init(&mut attr);
				debug_assert_pthread_result_ok!(result);
				
				let result = pthread_condattr_setclock(&mut attr, CLOCK_MONOTONIC);
				debug_assert_pthread_result_ok!(result);
				
				let result = pthread_cond_init(this.cond.get(), &attr);
				debug_assert_pthread_result_ok!(result);
				
				let result = pthread_condattr_destroy(&mut attr);
				debug_assert_pthread_result_ok!(result);
			}
			
			// See notes for `timed_wait()`.
			#[cfg(any(target_os = "macos", target_os = "ios", target_os = "l4re", target_os = "android"))]
			{
			}
		}
		
		this
	}
	
	// The mutex must be locked before calling this function, otherwise the behavior is undefined.
	#[cfg(unix)]
	#[inline(always)]
	fn wait(&self, mutex: *mut pthread_mutex_t)
	{
		let result = unsafe { pthread_cond_wait(self.cond.get(), mutex) };
		debug_assert_pthread_result_ok!(result);
	}
	
	// This implementation is used on systems that support `pthread_condattr_setclock` where we configure the condition variable to use a monotonic clock (instead of the default system clock).
	// This approach avoids all problems that result from changes made to the system time.
	#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "android")))]
	fn wait_timeout(&self, mutex: *mut pthread_mutex_t, duration: Duration) -> TimedOut
	{
		#[inline(always)]
		fn now() -> timespec
		{
			use ::libc::clock_gettime;
			use ::libc::CLOCK_MONOTONIC;
			
			let mut now = uninitialized();
			let result = unsafe { clock_gettime(CLOCK_MONOTONIC, &mut now) };
			debug_assert_pthread_result_ok!(result);
			now
		}
		
		let now = now();
		
		// Nanosecond calculations can't overflow because both values are below 1e9.
		let nanoseconds = duration.subsec_nanos() + now.tv_nsec as u32;
		
		let seconds = Self::saturating_cast_to_time_t(duration.as_secs()).checked_add((nanoseconds / 1_000_000_000) as time_t).and_then(|s| s.checked_add(now.tv_sec));
		
		let timeout = seconds.map(|s| timespec { tv_sec: s, tv_nsec: (nanoseconds % 1_000_000_000) as _}).unwrap_or(Self::MaximumTimeSpec);
		
		match unsafe { pthread_cond_timedwait(self.inner.get(), mutex, &timeout) }
		{
			ResultIsOk => TimedOut::Succeeded,
			
			ETIMEDOUT => TimedOut::TimedOut,
			
			illegal @ _ => panic!("Was not 0 or ETIMEDOUT but was '{}'", illegal),
		}
	}
	
	// This implementation is modeled after libcxx's condition_variable.
	// https://github.com/llvm-mirror/libcxx/blob/release_35/src/condition_variable.cpp#L46
	// https://github.com/llvm-mirror/libcxx/blob/release_35/include/__mutex_base#L367
	#[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
	fn wait_timeout(&self, mutex: *mut pthread_mutex_t, duration: Duration) -> TimedOut
	{
		use ::libc::c_long;
		use ::libc::gettimeofday;
		use ::libc::timeval;
		use ::std::ptr::null_mut;
		use ::std::time::Instant;
		
		// The OSX implementation of `pthread_cond_timedwait` is buggy with very long durations.
		// When the duration is greater than 0x100_0000_0000_0000 seconds, `pthread_cond_timedwait` in macOS Sierra returns error number 316.
		//
		// This program demonstrates the issue:
		// https://gist.github.com/stepancheg/198db4623a20aad2ad7cddb8fda4a63c
		//
		// To work around this issue, and possible bugs of other OSes, the timeout is clamped to 1000 years, which is allowed by the API of `wait_timeout` because of spurious wakeups.
		const DaysInAYear: u64 = 365;
		const SecondsInOneDay: u64 = 86_400;
		let maximum_duration_of_1000_years_to_work_around_os_x_bug = Duration::from_secs(1000 * DaysInAYear * SecondsInOneDay);
		let duration = min(duration, maximum_duration_of_1000_years_to_work_around_os_x_bug);
		
		// First, figure out what time it currently is, in both system and stable time.
		// `pthread_cond_timedwait` uses system time, but we want to report timeout based on stable time.
		let mut system_now = timeval { tv_sec: 0, tv_usec: 0 };
		let stable_now = Instant::now();
		let result = unsafe { gettimeofday(&mut system_now, null_mut()) };
		debug_assert_pthread_result_ok!(result);
		
		let seconds = duration.subsec_nanos() as c_long + (system_now.tv_usec * 1000) as c_long;
		let extra = (seconds / 1_000_000_000) as time_t;
		let nsec = seconds % 1_000_000_000;
		let seconds = Self::saturating_cast_to_time_t(duration.as_secs());
		
		let timeout = system_now.tv_sec.checked_add(extra).and_then(|s| s.checked_add(seconds)).map(|s| timespec { tv_sec: s, tv_nsec: nsec }).unwrap_or(Self::MaximumTimeSpec);
		
		let result = unsafe { pthread_cond_timedwait(self.cond.get(), mutex, &timeout) };
		debug_assert!(result == ETIMEDOUT || result == ResultIsOk, "Was not 0 or ETIMEDOUT but was '{}'", result);
		
		// ETIMEDOUT is not a totally reliable method of determining timeout due to clock shifts, so do the check ourselves.
		if stable_now.elapsed() < duration
		{
			TimedOut::Succeeded
		}
		else
		{
			TimedOut::TimedOut
		}
	}
	
	#[cfg(unix)]
	#[inline(always)]
	fn notify_one(&self)
	{
		let result = unsafe { pthread_cond_signal(self.cond.get()) };
		debug_assert_pthread_result_ok!(result);
	}
	
	#[cfg(unix)]
	#[inline(always)]
	fn notify_all(&self)
	{
		let result = unsafe { pthread_cond_broadcast(self.cond.get()) };
		debug_assert_pthread_result_ok!(result);
	}
	
	#[cfg(unix)]
	const MaximumTimeSpec: timespec = timespec
	{
		tv_sec: <time_t>::max_value(),
		tv_nsec: 1_000_000_000 - 1,
	};
	
	#[cfg(unix)]
	#[inline(always)]
	fn saturating_cast_to_time_t(value: u64) -> time_t
	{
		if value > <time_t>::max_value() as u64
		{
			<time_t>::max_value()
		}
		else
		{
			value as time_t
		}
	}
}
