// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct CtoMutexLockInner<T: CtoSafe>
{
	#[cfg(unix)] mutex: UnsafeCell<pthread_mutex_t>,
	cto_pool_inner: Arc<CtoPoolInner>,
	value: UnsafeCell<T>,
}

impl<T: CtoSafe> Drop for CtoMutexLockInner<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			self.destroy_mutex();
		}
	}
}

impl<T: CtoSafe> CtoSafe for CtoMutexLockInner<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.cto_pool_inner = cto_pool_inner.clone();
		
		self.deref_mut().reinitialize(cto_pool_inner);
		
		#[cfg(unix)]
		unsafe
		{
			self.mutex = UnsafeCell::new(PTHREAD_MUTEX_INITIALIZER);
			
			// self.mutex must be at a stable memory address.
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
	}
}

unsafe impl<T: CtoSafe> Send for CtoMutexLockInner<T>
{
}

unsafe impl<T: CtoSafe> Sync for CtoMutexLockInner<T>
{
}

impl<T: CtoSafe> UnwindSafe for CtoMutexLockInner<T>
{
}

impl<T: CtoSafe> RefUnwindSafe for CtoMutexLockInner<T>
{
}

impl<T: CtoSafe> Deref for CtoMutexLockInner<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { &*self.value.get() }
	}
}

impl<T: CtoSafe> DerefMut for CtoMutexLockInner<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut *self.value.get() }
	}
}

impl<T: CtoSafe> CtoMutexLockInner<T>
{
	#[inline(always)]
	pub(crate) fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuard<'mutex, T>
	{
		#[cfg(unix)]
		{
			let result = unsafe { pthread_mutex_lock(self.mutex.get()) };
			debug_assert_pthread_result_ok!(result);
		}
		
		CtoMutexLockGuard(self)
	}
	
	#[inline(always)]
	pub(crate) fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuard<'mutex, T>>
	{
		#[cfg(unix)]
		{
			// Error codes are EBUSY (lock in use) and EINVAL (which should not occur).
			if unsafe { pthread_mutex_trylock(self.mutex.get()) } == ResultIsOk
			{
				Some(CtoMutexLockGuard(self))
			}
			else
			{
				None
			}
		}
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
			debug_assert_pthread_result_ok_dragonfly!(result);
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
