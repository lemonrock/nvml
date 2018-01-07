// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Mutex, similar to that in Rust, but lacking the concept of Poison.
pub struct CtoReadWriteLock<T: CtoSafe>
{
	#[cfg(unix)] rwlock: UnsafeCell<pthread_rwlock_t>,
	write_lock: UnsafeCell<bool>,
	number_of_read_locks: AtomicUsize,
	cto_pool_inner: Arc<CtoPoolInner>,
	value: UnsafeCell<T>,
}

unsafe impl<T: CtoSafe> Send for CtoReadWriteLock<T>
{
}

unsafe impl<T: CtoSafe> Sync for CtoReadWriteLock<T>
{
}

impl<T: CtoSafe> UnwindSafe for CtoReadWriteLock<T>
{
}

impl<T: CtoSafe> RefUnwindSafe for CtoReadWriteLock<T>
{
}

impl<T: CtoSafe + Debug> Debug for CtoReadWriteLock<T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		const Name: &'static str = "CtoReadWriteLock";
		const Field: &'static str = "value";
		
		match self.try_read()
		{
			Some(cto_read_lock_guard) => f.debug_struct(Name).field(Field, &&*cto_read_lock_guard).finish(),
			
			None =>
			{
				struct LockedPlaceholder;
				
				impl Debug for LockedPlaceholder
				{
					fn fmt(&self, f: &mut Formatter) -> fmt::Result { f.write_str("<read-locked>") }
				}

				f.debug_struct(Name).field(Field, &LockedPlaceholder).finish()
			}
		}
	}
}

impl<T: CtoSafe> Drop for CtoReadWriteLock<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.destroy() }
		CtoPoolInner::free(&self.cto_pool_inner, self.value.get())
	}
}


impl<T: CtoSafe> CtoSafe for CtoReadWriteLock<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		#[cfg(unix)]
		{
			self.rwlock = UnsafeCell::new(PTHREAD_RWLOCK_INITIALIZER);
		}
		self.write_lock = UnsafeCell::new(false);
		self.number_of_read_locks = AtomicUsize::new(0);
		self.cto_pool_inner = cto_pool_inner.clone();
	}
}

impl<T: CtoSafe> CtoReadWriteLock<T>
{
	/// Obtain a read lock.
	/// Panics if write-locked.
	// This should be called once the mutex is at a stable memory address.
	//
	// According to the pthread_rwlock_rdlock spec, this function **may**
	// fail with EDEADLK if a deadlock is detected. On the other hand
	// pthread rwlocks will *never* return EDEADLK if they are initialized
	// as the "fast" kind (which ours always are). As a result, a deadlock
	// situation may actually return from the call to pthread_rwlock_rdlock
	// instead of blocking forever (as mutexes and Windows rwlocks do). Note
	// that not all unix implementations, however, will return EDEADLK for
	// their rwlocks.
	//
	// We roughly maintain the deadlocking behavior by panicking to ensure
	// that this lock acquisition does not succeed.
	//
	// We also check whether this lock is already write locked. This
	// is only possible if it was write locked by the current thread and
	// the implementation allows recursive locking. The POSIX standard
	// doesn't require recursively locking a rwlock to deadlock, but we can't
	// allow that because it could lead to aliasing issues.
	#[cfg(unix)]
	#[inline(always)]
	pub fn read<'read_write_lock>(&'read_write_lock self) -> CtoReadWriteLockReadGuard<'read_write_lock, T>
	{
		let result = unsafe { pthread_rwlock_rdlock(self.rwlock()) };
		
		if result == EAGAIN
		{
			panic!("rwlock maximum reader count exceeded");
		}
		else if result == EDEADLK || self.is_write_locked()
		{
			if result == ResultIsOk
			{
				self.unlock_pthread_read_or_write_lock();
			}
			panic!("rwlock read lock would result in deadlock");
		}
		else
		{
			debug_assert_pthread_result_ok!(result);
			self.increment_number_of_read_locks();
		}
		
		CtoReadWriteLockReadGuard(self)
	}
	
	/// Try to obtain a read lock.
	/// Does not panic.
	#[cfg(unix)]
	#[inline(always)]
	pub fn try_read<'read_write_lock>(&'read_write_lock self) -> Option<CtoReadWriteLockReadGuard<'read_write_lock, T>>
	{
		let result = unsafe { pthread_rwlock_tryrdlock(self.rwlock()) };
		
		if result == ResultIsOk
		{
			if self.is_write_locked()
			{
				self.unlock_pthread_read_or_write_lock();
				None
			}
			else
			{
				self.increment_number_of_read_locks();
				Some(CtoReadWriteLockReadGuard(self))
			}
		}
		else
		{
			None
		}
	}
	
	/// Obtains a write lock.
	/// Panics if already write-locked or there are extant read-locks.
	#[cfg(unix)]
	#[inline(always)]
	pub fn write<'read_write_lock>(&'read_write_lock self) -> CtoReadWriteLockWriteGuard<'read_write_lock, T>
	{
		let result = unsafe { pthread_rwlock_wrlock(self.rwlock()) };
		
		// See comments for `read()` regarding deadlock.
		if result == EDEADLK || self.is_write_locked() || self.there_are_read_locks()
		{
			if result == ResultIsOk
			{
				self.unlock_pthread_read_or_write_lock();
			}
			panic!("rwlock write lock would result in deadlock");
		}
		else
		{
			debug_assert_pthread_result_ok!(result);
		}
		
		self.set_is_write_locked();
		CtoReadWriteLockWriteGuard(self)
	}
	
	/// Tries to obtain a write lock.
	/// Does not panic.
	#[cfg(unix)]
	#[inline(always)]
	pub fn try_write<'read_write_lock>(&'read_write_lock self) -> Option<CtoReadWriteLockWriteGuard<'read_write_lock, T>>
	{
		let result = unsafe { pthread_rwlock_trywrlock(self.rwlock()) };
		
		if result == ResultIsOk
		{
			if self.is_write_locked() || self.there_are_read_locks()
			{
				self.unlock_pthread_read_or_write_lock();
				None
			}
			else
			{
				self.set_is_write_locked();
				Some(CtoReadWriteLockWriteGuard(self))
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub(crate) unsafe fn read_unlock(&self)
	{
		debug_assert!(!self.is_write_locked(), "We are write locked, but we're unlocking a read lock");
		
		self.decrement_number_of_read_locks();
		self.unlock_pthread_read_or_write_lock();
	}
	
	#[inline(always)]
	pub(crate) unsafe fn write_unlock(&self)
	{
		debug_assert!(self.is_write_locked(), "We are not write locked, but we're unlocking the write lock");
		debug_assert!(!self.there_are_read_locks(), "We are write locked and trying to unlock the write lock, but there are readers");
		
		self.set_is_not_write_locked();
		self.unlock_pthread_read_or_write_lock();
	}
	
	#[cfg(unix)]
	#[inline(always)]
	unsafe fn destroy(&self)
	{
		let result = pthread_rwlock_destroy(self.rwlock());
		
		#[cfg(not(target_os = "dragonfly"))]
		{
			debug_assert_pthread_result_ok!(result);
		}
		
		#[cfg(target_os = "dragonfly")]
		{
			// On DragonFly `pthread_rwlock_destroy()` returns `EINVAL` if called on a rwlock that was just initialized with `PTHREAD_RWLOCK_INITIALIZER`.
			// Once it is used (locked or unlocked) or `pthread_rwlock_init()` is called, this behaviour no longer occurs.
			debug_assert_pthread_result_ok_dragonfly!(result);
		}
	}
	
	#[cfg(unix)]
	#[inline(always)]
	fn unlock_pthread_read_or_write_lock(&self)
	{
		let result = unsafe { pthread_rwlock_unlock(self.rwlock()) };
		
		debug_assert_pthread_result_ok!(result);
	}
	
	#[cfg(unix)]
	#[inline(always)]
	fn rwlock(&self) -> *mut pthread_rwlock_t
	{
		self.rwlock.get()
	}
	
	#[inline(always)]
	fn is_write_locked(&self) -> bool
	{
		unsafe { *self.write_lock() }
	}
	
	#[inline(always)]
	fn set_is_write_locked(&self)
	{
		unsafe { *self.write_lock() = true }
	}
	
	#[inline(always)]
	unsafe fn set_is_not_write_locked(&self)
	{
		*self.write_lock() = false
	}
	
	#[inline(always)]
	fn write_lock(&self) -> *mut bool
	{
		self.write_lock.get()
	}
	
	#[inline(always)]
	fn there_are_read_locks(&self) -> bool
	{
		self.number_of_read_locks() != 0
	}
	
	#[inline(always)]
	fn number_of_read_locks(&self) -> usize
	{
		self.number_of_read_locks.load(NumberOfReadersOrdering)
	}
	
	#[inline(always)]
	fn increment_number_of_read_locks(&self)
	{
		self.number_of_read_locks.fetch_add(1, NumberOfReadersOrdering);
	}
	
	#[inline(always)]
	fn decrement_number_of_read_locks(&self)
	{
		self.number_of_read_locks.fetch_sub(1, NumberOfReadersOrdering);
	}
}

const NumberOfReadersOrdering: Ordering = Ordering::Relaxed;
