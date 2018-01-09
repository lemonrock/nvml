// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Similar to a Rust Rc but allocated in a persistent memory CTO Pool.
pub struct CtoRc<Value: CtoSafe>
{
	persistent_memory_pointer: Shared<CtoRcInner<Value>>
}

impl<Value: CtoSafe> PersistentMemoryWrapper for CtoRc<Value>
{
	type PersistentMemory = CtoRcInner<Value>;
	
	type Value = Value;
	
	#[inline(always)]
	unsafe fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(&mut Self::Value) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_alloc_guard_reference: &CtoPoolArc, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let mut persistent_memory_pointer = Shared::new_unchecked(persistent_memory_pointer);
		{
			let cto_rc_inner = persistent_memory_pointer.as_mut();
			cto_rc_inner.cto_pool_alloc_guard_reference = cto_pool_alloc_guard_reference.clone();
			cto_rc_inner.strong_counter = CtoRcCounter::default();
			cto_rc_inner.weak_counter = CtoRcCounter::default();
			initializer(cto_rc_inner)?;
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

impl<Value: CtoSafe> CtoSafe for CtoRc<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_alloc_guard_reference: &CtoPoolArc)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_alloc_guard_reference)
	}
}

impl<Value: CtoSafe> Drop for CtoRc<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let should_be_dropped_because_there_are_no_strong_references =
		{
			let cto_rc_inner = self.persistent_memory_mut();
			cto_rc_inner.strong_count_decrement();
			cto_rc_inner.strong_count() == 0
		};
		
		if should_be_dropped_because_there_are_no_strong_references
		{
			let pool_pointer = self.persistent_memory().cto_pool_alloc_guard_reference.pool_pointer();
			
			let persistent_memory_pointer = self.persistent_memory_pointer.as_ptr();
			
			if needs_drop::<CtoRcInner<Value>>()
			{
				unsafe { drop_in_place(persistent_memory_pointer) }
			}
			
			pool_pointer.free(persistent_memory_pointer);
		}
	}
}

impl<Value: CtoSafe> Clone for CtoRc<Value>
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

impl<Value: CtoSafe> !Send for CtoRc<Value>
{
}

impl<Value: CtoSafe> !Sync for CtoRc<Value>
{
}

impl<Value: CtoSafe + PartialEq> PartialEq for CtoRc<Value>
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

impl<Value: CtoSafe + Eq> Eq for CtoRc<Value>
{
}

impl<Value: CtoSafe + PartialOrd> PartialOrd for CtoRc<Value>
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

impl<Value: CtoSafe + Ord> Ord for CtoRc<Value>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

impl<Value: CtoSafe + Hash> Hash for CtoRc<Value>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state);
	}
}

impl<Value: CtoSafe + Display> Display for CtoRc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(self.deref(), f)
	}
}

impl<Value: CtoSafe + Debug> Debug for CtoRc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.deref(), f)
	}
}

impl<Value: CtoSafe> Pointer for CtoRc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&self.deref(), f)
	}
}

impl<Value: CtoSafe> Deref for CtoRc<Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.persistent_memory().deref()
	}
}

impl<Value: CtoSafe> Borrow<Value> for CtoRc<Value>
{
	#[inline(always)]
	fn borrow(&self) -> &Value
	{
		self.deref()
	}
}

impl<Value: CtoSafe> AsRef<Value> for CtoRc<Value>
{
	#[inline(always)]
	fn as_ref(&self) -> &Value
	{
		self.deref()
	}
}

impl<Value: CtoSafe> CtoRc<Value>
{
	/// Downgrades this strong reference to a weak reference.
	#[inline(always)]
	pub fn downgrade(this: &Self) -> WeakCtoRc<Value>
	{
		this.persistent_memory().weak_count_increment();
		WeakCtoRc(Some(this.persistent_memory_pointer))
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
	pub fn get_mut(this: &mut Self) -> Option<&mut Value>
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
		this.persistent_memory_pointer.as_ptr() == other.persistent_memory_pointer.as_ptr()
	}
	
	/// A pointer to use with C. Use wisely; dropping this object may cause the pointer to go out of scope.
	#[inline(always)]
	pub fn as_ptr(this: Self) -> *const Value
	{
		this.deref() as *const Value
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoRcInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_ref() }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&mut self) -> &mut CtoRcInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_mut() }
	}
}
