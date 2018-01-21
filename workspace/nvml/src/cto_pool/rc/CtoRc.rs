// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Similar to a Rust Rc but allocated in a persistent memory CTO Pool.
pub struct CtoRc<Value: CtoSafe>
{
	persistent_memory_pointer: NonNull<CtoRcInner<Value>>
}

impl<Value: CtoSafe> PersistentMemoryWrapper for CtoRc<Value>
{
	type PersistentMemory = CtoRcInner<Value>;
	
	type Value = Value;
	
	#[inline(always)]
	unsafe fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(*mut Self::Value, &CtoPoolArc) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let mut persistent_memory_pointer = NonNull::new_unchecked(persistent_memory_pointer);
		
		{
			persistent_memory_pointer.as_mut().allocated(cto_pool_arc, initializer)?;
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
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<Value: CtoSafe> Drop for CtoRc<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let has_no_more_strong_references =
		{
			let cto_rc_inner = self.persistent_memory();
			
			cto_rc_inner.strong_count_decrement();
			
			cto_rc_inner.strong_count() == 0
		};
		
		if has_no_more_strong_references
		{
			let ptr = self.persistent_memory_pointer();
			
			// Destroy the value at this time, even though we may not free the allocation itself (there may still be weak pointers lying around).
			unsafe { drop_in_place(self.persistent_memory_mut().deref_mut()) };
			
			let cto_rc_inner = self.persistent_memory();
			
			cto_rc_inner.weak_count_decrement();
			
			if cto_rc_inner.weak_count() == 0
			{
				let pool_pointer = cto_rc_inner.cto_pool_arc.pool_pointer();
				
				pool_pointer.free(ptr);
			}
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
		
		WeakCtoRc
		{
			persistent_memory_pointer: Some(this.persistent_memory_pointer),
		}
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
	pub fn as_ptr(this: &Self) -> *const Value
	{
		this.deref() as *const Value
	}
	
	/// Gets a raw pointer to Value, suitable for use with FFI.
	/// Must be eventually passed to `from_raw()`, or a very serious (possibly irrecoverable even with reboots) memory leak will occur.
	#[inline(always)]
	pub fn into_raw(mut this: Self) -> *mut Value
	{
		this.persistent_memory_mut().into_raw_value_pointer()
	}
	
	/// Gets a CtoRc from a raw pointer to Value, typically passed back from FFI.
	/// Must be a pointer originally created using `into_raw()`.
	#[inline(always)]
	pub unsafe fn from_raw(raw_value_pointer: *mut Value) -> Self
	{
		Self
		{
			persistent_memory_pointer: NonNull::new_unchecked(CtoRcInner::from_raw_value_pointer(raw_value_pointer)),
		}
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
	
	#[inline(always)]
	fn persistent_memory_pointer(&self) -> *mut CtoRcInner<Value>
	{
		self.persistent_memory_pointer.as_ptr()
	}
}
