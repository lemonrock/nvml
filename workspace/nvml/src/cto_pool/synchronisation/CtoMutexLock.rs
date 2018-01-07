// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Mutex, similar to that in Rust, but lacking the concept of Poison.
pub struct CtoMutexLock<T: CtoSafe>
{
	#[cfg(unix)] mutex: UnsafeCell<pthread_mutex_t>,
	cto_pool_inner: Arc<CtoPoolInner>,
	value: UnsafeCell<T>,
}

unsafe impl<T: CtoSafe> Send for CtoMutexLock<T>
{
}

unsafe impl<T: CtoSafe> Sync for CtoMutexLock<T>
{
}

impl<T: CtoSafe> UnwindSafe for CtoMutexLock<T>
{
}

impl<T: CtoSafe> RefUnwindSafe for CtoMutexLock<T>
{
}

impl<T: CtoSafe + Debug> Debug for CtoMutexLock<T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		const Name: &'static str = "CtoMutexLock";
		const Field: &'static str = "value";
		
		match self.try_lock()
		{
			Some(cto_mutex_lock_guard) => f.debug_struct(Name).field(Field, &&*cto_mutex_lock_guard).finish(),
			
			None =>
			{
				struct LockedPlaceholder;
				
				impl Debug for LockedPlaceholder
				{
					fn fmt(&self, f: &mut Formatter) -> fmt::Result { f.write_str("<locked>") }
				}

				f.debug_struct(Name).field(Field, &LockedPlaceholder).finish()
			}
		}
	}
}

impl<T: CtoSafe> Drop for CtoMutexLock<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.destroy_mutex() }
		CtoPoolInner::free(&self.cto_pool_inner, self.value.get())
	}
}


impl<T: CtoSafe> CtoSafe for CtoMutexLock<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.mutex = UnsafeCell::new(PTHREAD_MUTEX_INITIALIZER);
		self.cto_pool_inner = cto_pool_inner.clone();
		
		unsafe { self.initialize_mutex() }
	}
}

macro_rules! debug_assert_pthread_result_ok
{
	($result: ident) =>
	{
		debug_assert_eq!($result, CtoMutexLock::<T>::ResultIsOk, "result from pthread function was not OK");
	}
}

impl<T: CtoSafe> CtoMutexLock<T>
{
	/// Locks a mutex.
	#[inline(always)]
	pub fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuard<'mutex, T>
	{
		#[cfg(unix)]
		{
			let result = unsafe { pthread_mutex_lock(self.mutex.get()) };
			debug_assert_pthread_result_ok!(result);
		}
		
		CtoMutexLockGuard
		{
			cto_mutex_lock: self
		}
	}
	
	/// Returns Some(lock_guard) if could be locked.
	/// Returns None if the lock is held by another.
	#[inline(always)]
	pub fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuard<'mutex, T>>
	{
		#[cfg(unix)]
		{
			// Error codes are EBUSY (lock in use) and EINVAL (which should not occur).
			if unsafe { pthread_mutex_trylock(self.mutex.get()) } == CtoMutexLock::<T>::ResultIsOk
			{
				Some
				(
					CtoMutexLockGuard
					{
						cto_mutex_lock: self
					}
				)
			}
			else
			{
				None
			}
		}
	}
	
	const ResultIsOk: i32 = 0;
	
	// This should be called once the mutex is at a stable memory address.
	//
	// A pthread mutex initialized with PTHREAD_MUTEX_INITIALIZER will have
	// a type of PTHREAD_MUTEX_DEFAULT, which has undefined behavior if you
	// try to re-lock it from the same thread when you already hold a lock.
	//
	// In practice, glibc takes advantage of this undefined behavior to
	// implement hardware lock elision, which uses hardware transactional
	// memory to avoid acquiring the lock. While a transaction is in
	// progress, the lock appears to be unlocked. This isn't a problem for
	// other threads since the transactional memory will abort if a conflict
	// is detected, however no abort is generated if re-locking from the
	// same thread.
	//
	// Since locking the same mutex twice will result in two aliasing &mut
	// references, we instead create the mutex with type
	// PTHREAD_MUTEX_NORMAL which is guaranteed to deadlock if we try to
	// re-lock it from the same thread, thus avoiding undefined behavior.
	#[cfg(unix)]
	#[inline(always)]
	unsafe fn initialize_mutex(&mut self)
	{
		let mut mutex_options: pthread_mutexattr_t = uninitialized();
		
		let result = pthread_mutexattr_init(&mut mutex_options);
		debug_assert_pthread_result_ok!(result);
		
		let result = pthread_mutexattr_settype(&mut mutex_options, PTHREAD_MUTEX_NORMAL);
		debug_assert_pthread_result_ok!(result);
		
		let result = pthread_mutex_init(self.mutex.get(), &mutex_options);
		debug_assert_pthread_result_ok!(result);
		
		let result = pthread_mutexattr_destroy(&mut mutex_options);
		debug_assert_pthread_result_ok!(result);
	}
	
	// Behavior is undefined if there are current or will be future users of this mutex.
	#[cfg(unix)]
	#[inline(always)]
	unsafe fn destroy_mutex(&self)
	{
		let result = pthread_mutex_destroy(self.mutex.get());
		
		#[cfg(not(target_os = "dragonfly"))]
		{
			debug_assert_pthread_result_ok!(result);
		}
		
		#[cfg(target_os = "dragonfly")]
		{
			// On DragonFly pthread_mutex_destroy() returns EINVAL if called on a mutex that was just initialized with libc::PTHREAD_MUTEX_INITIALIZER.
			// Once it is used (locked/unlocked) or pthread_mutex_init() is called, this behaviour no longer occurs.
			debug_assert!(result == CtoMutexLock::ResultIsOk || result == ::libc::EINVAL);
		}
	}
	
	/// Unlocks the mutex.
	///
	/// Behavior is undefined if the current thread does not actually hold the mutex.
	#[cfg(unix)]
	#[inline(always)]
	unsafe fn unlock_mutex(&self)
	{
		let result = pthread_mutex_unlock(self.mutex.get());
		debug_assert_pthread_result_ok!(result);
	}
}
