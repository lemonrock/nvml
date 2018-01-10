// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) struct CtoMutexLockInner<Value: CtoSafe>
{
	#[cfg(unix)] mutex: UnsafeCell<pthread_mutex_t>,
	cto_pool_arc: CtoPoolArc,
	value: UnsafeCell<Value>,
}

impl<Value: CtoSafe> Drop for CtoMutexLockInner<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		#[cfg(unix)]
		{
			let result = unsafe { pthread_mutex_destroy(self.mutex.get()) };
			
			#[cfg(not(target_os = "dragonfly"))]
			{
				debug_assert_pthread_result_ok!(result);
			}
			
			#[cfg(target_os = "dragonfly")]
			{
				// On DragonFly `pthread_mutex_destroy()` returns `EINVAL` if called on a mutex that was just initialized with `PTHREAD_MUTEX_INITIALIZER`.
				// Once it is used (locked or unlocked) or `pthread_mutex_init()` is called, this behaviour no longer occurs.
				debug_assert_pthread_result_ok_dragonfly!(result);
			}
		}
	}
}

unsafe impl<Value: CtoSafe> Send for CtoMutexLockInner<Value>
{
}

unsafe impl<Value: CtoSafe> Sync for CtoMutexLockInner<Value>
{
}

impl<Value: CtoSafe> UnwindSafe for CtoMutexLockInner<Value>
{
}

impl<Value: CtoSafe> RefUnwindSafe for CtoMutexLockInner<Value>
{
}

impl<Value: CtoSafe> Deref for CtoMutexLockInner<Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { &*self.value.get() }
	}
}

impl<Value: CtoSafe> DerefMut for CtoMutexLockInner<Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut *self.value.get() }
	}
}

impl<Value: CtoSafe> CtoMutexLockInner<Value>
{
	#[inline(always)]
	fn common_initialization(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		#[cfg(unix)]
		unsafe
		{
			write(&mut self.mutex, UnsafeCell::new(PTHREAD_MUTEX_INITIALIZER));
			
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
		
		cto_pool_arc.write(&mut self.cto_pool_arc);
	}
	
	#[inline(always)]
	fn allocated<InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&mut self, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<(), InitializationError>
	{
		self.common_initialization(cto_pool_arc);
		
		initializer(self.value.get(), cto_pool_arc)
	}
	
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.common_initialization(cto_pool_arc);
		
		self.deref_mut().cto_pool_opened(cto_pool_arc);
	}
	
	#[inline(always)]
	pub(crate) fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuard<'mutex, Value>
	{
		#[cfg(unix)]
		{
			let result = unsafe { pthread_mutex_lock(self.mutex.get()) };
			debug_assert_pthread_result_ok!(result);
		}
		
		CtoMutexLockGuard(self)
	}
	
	#[inline(always)]
	pub(crate) fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuard<'mutex, Value>>
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
	
	// Behavior is undefined if the current thread does not actually hold the mutex.
	#[cfg(unix)]
	#[inline(always)]
	unsafe fn unlock_mutex(&self)
	{
		let result = pthread_mutex_unlock(self.mutex.get());
		debug_assert_pthread_result_ok!(result);
	}
}
