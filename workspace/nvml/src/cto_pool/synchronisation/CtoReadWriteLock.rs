// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Mutex, similar to that in Rust, but lacking the concept of Poison.
pub struct CtoReadWriteLock<Value: CtoSafe>
{
	persistent_memory_pointer: Shared<CtoReadWriteLockInner<Value>>,
}

impl<Value: CtoSafe> PersistentMemoryWrapper for CtoReadWriteLock<Value>
{
	type PersistentMemory = CtoReadWriteLockInner<Value>;
	
	type Value = Value;
	
	#[inline(always)]
	unsafe fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(*mut Self::Value) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let mut persistent_memory_pointer = Shared::new_unchecked(persistent_memory_pointer);
		{
			let cto_read_write_lock_inner = persistent_memory_pointer.as_mut();
			
			cto_read_write_lock_inner.common_initialization(cto_pool_arc);
			
			initializer(cto_read_write_lock_inner.value.get())?;
		}
		Ok
		(
			Self
			{
				persistent_memory_pointer,
			}
		)
	}
}

impl<Value: CtoSafe> CtoSafe for CtoReadWriteLock<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<Value: CtoSafe> Drop for CtoReadWriteLock<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pool_pointer = self.persistent_memory().cto_pool_arc.pool_pointer();
		
		let persistent_memory_pointer = self.persistent_memory_pointer.as_ptr();
		
		unsafe { drop_in_place(persistent_memory_pointer) }
		
		pool_pointer.free(persistent_memory_pointer);
	}
}

unsafe impl<Value: CtoSafe> Send for CtoReadWriteLock<Value>
{
}

unsafe impl<Value: CtoSafe> Sync for CtoReadWriteLock<Value>
{
}

impl<Value: CtoSafe> UnwindSafe for CtoReadWriteLock<Value>
{
}

impl<Value: CtoSafe> RefUnwindSafe for CtoReadWriteLock<Value>
{
}

impl<Value: CtoSafe + Debug> Debug for CtoReadWriteLock<Value>
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

impl<Value: CtoSafe> CtoReadWriteLock<Value>
{
	/// Obtain a read lock.
	/// Panics if write-locked.
	#[inline(always)]
	pub fn read<'read_write_lock>(&'read_write_lock self) -> CtoReadWriteLockReadGuard<'read_write_lock, Value>
	{
		self.persistent_memory().read()
	}
	
	/// Try to obtain a read lock.
	/// Does not panic.
	#[inline(always)]
	pub fn try_read<'read_write_lock>(&'read_write_lock self) -> Option<CtoReadWriteLockReadGuard<'read_write_lock, Value>>
	{
		self.persistent_memory().try_read()
	}
	
	/// Obtains a write lock.
	/// Panics if already write-locked or there are extant read-locks.
	#[inline(always)]
	pub fn write<'read_write_lock>(&'read_write_lock self) -> CtoReadWriteLockWriteGuard<'read_write_lock, Value>
	{
		self.persistent_memory().write()
	}
	
	/// Tries to obtain a write lock.
	/// Does not panic.
	#[inline(always)]
	pub fn try_write<'read_write_lock>(&'read_write_lock self) -> Option<CtoReadWriteLockWriteGuard<'read_write_lock, Value>>
	{
		self.persistent_memory().try_write()
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoReadWriteLockInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_ref() }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&mut self) -> &mut CtoReadWriteLockInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_mut() }
	}
}
