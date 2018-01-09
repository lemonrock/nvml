// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Mutex, similar to that in Rust, but lacking the concept of Poison.
pub struct CtoMutexLock<T: CtoSafe>
{
	persistent_memory_pointer: *mut CtoMutexLockInner<T>,
}

impl<T: CtoSafe> CtoSafe for CtoMutexLock<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_inner: *mut PMEMctopool)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_inner)
	}
}

impl<T: CtoSafe> PersistentMemoryWrapper for CtoMutexLock<T>
{
	type PersistentMemory = CtoMutexLockInner<T>;
	
	type Value = T;
	
	#[inline(always)]
	fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(&mut Self::Value) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_inner: &Arc<CtoPoolInner>, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let inner = unsafe { &mut * persistent_memory_pointer };
		initializer(inner.deref_mut())?;
		inner.cto_pool_opened(cto_pool_inner);
		Ok
		(
			Self
			{
				persistent_memory_pointer,
			}
		)
	}
}

impl<T: CtoSafe> Drop for CtoMutexLock<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let cto_pool_inner = self.persistent_memory().cto_pool_inner.clone();
		CtoPoolInner::free_persistent_memory(&cto_pool_inner, self.persistent_memory_pointer)
	}
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

impl<T: CtoSafe> CtoMutexLock<T>
{
	/// Locks a mutex.
	#[inline(always)]
	pub fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuard<'mutex, T>
	{
		self.persistent_memory().lock()
	}
	
	/// Returns Some(lock_guard) if could be locked.
	/// Returns None if the lock is held by another.
	#[inline(always)]
	pub fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuard<'mutex, T>>
	{
		self.persistent_memory().try_lock()
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoMutexLockInner<T>
	{
		unsafe { &*self.persistent_memory_pointer }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&self) -> &mut CtoMutexLockInner<T>
	{
		unsafe { &mut *self.persistent_memory_pointer }
	}
}
