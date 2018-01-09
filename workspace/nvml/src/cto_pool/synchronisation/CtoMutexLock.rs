// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A Mutex, similar to that in Rust, but lacking the concept of Poison.
pub struct CtoMutexLock<Value: CtoSafe>
{
	persistent_memory_pointer: Unique<CtoMutexLockInner<Value>>,
}

impl<Value: CtoSafe> PersistentMemoryWrapper for CtoMutexLock<Value>
{
	type PersistentMemory = CtoMutexLockInner<Value>;
	
	type Value = Value;
	
	#[inline(always)]
	unsafe fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(*mut Self::Value, &CtoPoolArc) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let mut persistent_memory_pointer = Unique::new_unchecked(persistent_memory_pointer);
		
		{
			persistent_memory_pointer.as_mut().created(cto_pool_arc, initializer)?;
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

impl<Value: CtoSafe> CtoSafe for CtoMutexLock<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<Value: CtoSafe> Drop for CtoMutexLock<Value>
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

unsafe impl<Value: CtoSafe> Send for CtoMutexLock<Value>
{
}

unsafe impl<Value: CtoSafe> Sync for CtoMutexLock<Value>
{
}

impl<Value: CtoSafe> UnwindSafe for CtoMutexLock<Value>
{
}

impl<Value: CtoSafe> RefUnwindSafe for CtoMutexLock<Value>
{
}

impl<Value: CtoSafe + Debug> Debug for CtoMutexLock<Value>
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

impl<Value: CtoSafe> CtoMutexLock<Value>
{
	/// Locks a mutex.
	#[inline(always)]
	pub fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuard<'mutex, Value>
	{
		self.persistent_memory().lock()
	}
	
	/// Returns Some(lock_guard) if could be locked.
	/// Returns None if the lock is held by another.
	#[inline(always)]
	pub fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuard<'mutex, Value>>
	{
		self.persistent_memory().try_lock()
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoMutexLockInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_ref() }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&mut self) -> &mut CtoMutexLockInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_mut() }
	}
}
