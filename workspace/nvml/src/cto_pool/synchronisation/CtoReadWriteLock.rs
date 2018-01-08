// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Mutex, similar to that in Rust, but lacking the concept of Poison.
pub struct CtoReadWriteLock<T: CtoSafe>
{
	persistent_memory_pointer: *mut CtoReadWriteLockInner<T>,
}

impl<T: CtoSafe> CtoSafe for CtoReadWriteLock<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.persistent_memory_mut().reinitialize(cto_pool_inner)
	}
}

impl<T: CtoSafe> PersistentMemoryWrapper for CtoReadWriteLock<T>
{
	type PersistentMemory = CtoReadWriteLockInner<T>;
	
	type Value = T;
	
	#[inline(always)]
	fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(&mut Self::Value) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_inner: &Arc<CtoPoolInner>, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let inner = unsafe { &mut * persistent_memory_pointer };
		initializer(inner.deref_mut())?;
		inner.reinitialize(cto_pool_inner);
		Ok
		(
			Self
			{
				persistent_memory_pointer,
			}
		)
	}
}

impl<T: CtoSafe> Drop for CtoReadWriteLock<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let cto_pool_inner = self.persistent_memory().cto_pool_inner.clone();
		CtoPoolInner::free_persistent_memory(&cto_pool_inner, self.persistent_memory_pointer)
	}
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

impl<T: CtoSafe> CtoReadWriteLock<T>
{
	/// Obtain a read lock.
	/// Panics if write-locked.
	#[inline(always)]
	pub fn read<'read_write_lock>(&'read_write_lock self) -> CtoReadWriteLockReadGuard<'read_write_lock, T>
	{
		self.persistent_memory().read()
	}
	
	/// Try to obtain a read lock.
	/// Does not panic.
	#[inline(always)]
	pub fn try_read<'read_write_lock>(&'read_write_lock self) -> Option<CtoReadWriteLockReadGuard<'read_write_lock, T>>
	{
		self.persistent_memory().try_read()
	}
	
	/// Obtains a write lock.
	/// Panics if already write-locked or there are extant read-locks.
	#[inline(always)]
	pub fn write<'read_write_lock>(&'read_write_lock self) -> CtoReadWriteLockWriteGuard<'read_write_lock, T>
	{
		self.persistent_memory().write()
	}
	
	/// Tries to obtain a write lock.
	/// Does not panic.
	#[inline(always)]
	pub fn try_write<'read_write_lock>(&'read_write_lock self) -> Option<CtoReadWriteLockWriteGuard<'read_write_lock, T>>
	{
		self.persistent_memory().try_write()
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoReadWriteLockInner<T>
	{
		unsafe { &*self.persistent_memory_pointer }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&self) -> &mut CtoReadWriteLockInner<T>
	{
		unsafe { &mut *self.persistent_memory_pointer }
	}
}
