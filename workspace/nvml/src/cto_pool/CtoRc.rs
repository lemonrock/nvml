// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Similar to a Rust Rc but allocated in a persistent memory CTO Pool.
pub struct CtoRc<T: CtoSafe>
{
	persistent_memory_pointer: *mut CtoRcInner<T>
}

impl<T: CtoSafe> CtoSafe for CtoRc<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_inner: *mut PMEMctopool)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_inner)
	}
}

impl<T: CtoSafe> PersistentMemoryWrapper for CtoRc<T>
{
	type PersistentMemory = CtoRcInner<T>;
	
	type Value = T;
	
	#[inline(always)]
	fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(&mut Self::Value) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_inner: &Arc<CtoPoolInner>, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let inner = unsafe { &mut * persistent_memory_pointer };
		inner.initialize_persistent_memory(cto_pool_inner, initializer)?;
		Ok
		(
			Self
			{
				persistent_memory_pointer,
			}
		)
	}
}

impl<T: CtoSafe> Drop for CtoRc<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let cto_rc_inner = self.persistent_memory_mut();
		cto_rc_inner.strong_count_decrement();
		if cto_rc_inner.strong_count() == 0
		{
			let cto_pool_inner = cto_rc_inner.cto_pool_inner.clone();
			CtoPoolInner::free_persistent_memory(&cto_pool_inner, self.persistent_memory_pointer)
		}
	}
}

impl<T: CtoSafe> Clone for CtoRc<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		self.persistent_memory().strong_count_increment();
		
		Self
		{
			persistent_memory_pointer: self.persistent_memory_pointer,
		}
	}
}

impl<T: CtoSafe> !Send for CtoRc<T>
{
}

impl<T: CtoSafe> !Sync for CtoRc<T>
{
}

impl<T: CtoSafe + PartialEq> PartialEq for CtoRc<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		PartialEq::eq(&**self, &**other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		PartialEq::ne(&**self, &**other)
	}
}

impl<T: CtoSafe + Eq> Eq for CtoRc<T>
{
}

impl<T: CtoSafe + PartialOrd> PartialOrd for CtoRc<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&**self, &**other)
	}
	
	#[inline(always)]
	fn lt(&self, other: &Self) -> bool
	{
		PartialOrd::lt(&**self, &**other)
	}
	
	#[inline(always)]
	fn le(&self, other: &Self) -> bool
	{
		PartialOrd::le(&**self, &**other)
	}
	
	#[inline(always)]
	fn ge(&self, other: &Self) -> bool
	{
		PartialOrd::ge(&**self, &**other)
	}
	
	#[inline(always)]
	fn gt(&self, other: &Self) -> bool
	{
		PartialOrd::gt(&**self, &**other)
	}
}

impl<T: CtoSafe + Ord> Ord for CtoRc<T>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

impl<T: CtoSafe + Hash> Hash for CtoRc<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state);
	}
}

impl<T: CtoSafe + Display> Display for CtoRc<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe + Debug> Debug for CtoRc<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe> Pointer for CtoRc<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&self.deref(), f)
	}
}

impl<T: CtoSafe> Deref for CtoRc<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.persistent_memory().deref()
	}
}

impl<T: CtoSafe> Borrow<T> for CtoRc<T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe> AsRef<T> for CtoRc<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe> CtoRc<T>
{
	/// Downgrades this strong reference to a weak reference.
	#[inline(always)]
	pub fn downgrade(this: &Self) -> WeakCtoRc<T>
	{
		this.persistent_memory().weak_count_increment();
		WeakCtoRc(this.persistent_memory_pointer)
	}
	
	/// How many strong references are there (will always be at least one)?
	#[inline(always)]
	pub fn strong_count(this: &Self) -> usize
	{
		this.persistent_memory().strong_count()
	}
	
	/// How many weak references are there? (there may be none).
	#[inline(always)]
	pub fn weak_count(this: &Self) -> usize
	{
		this.persistent_memory().weak_count()
	}
	
	/// Is this object unique, ie there is only one strong reference and no weak references?
	#[inline(always)]
	pub fn is_unique(this: &Self) -> bool
	{
		this.persistent_memory().is_unique()
	}
	
	/// If this object is unique (see `is_unique()`) then will return Some otherwise None.
	#[inline(always)]
	pub fn get_mut(this: &mut Self) -> Option<&mut T>
	{
		if Self::is_unique(this)
		{
			Some(this.persistent_memory_mut().deref_mut())
		}
		else
		{
			None
		}
	}
	
	/// Pointer equality
	#[inline(always)]
	pub fn ptr_eq(this: &Self, other: &Self) -> bool
	{
		this.persistent_memory_pointer == other.persistent_memory_pointer
	}
	
	/// A pointer to use with C. Use wisely; dropping this object may cause the pointer to go out of scope.
	#[inline(always)]
	pub fn as_ptr(this: Self) -> *const T
	{
		this.deref() as *const T
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoRcInner<T>
	{
		unsafe { &*self.persistent_memory_pointer }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&self) -> &mut CtoRcInner<T>
	{
		unsafe { &mut *self.persistent_memory_pointer }
	}
}
