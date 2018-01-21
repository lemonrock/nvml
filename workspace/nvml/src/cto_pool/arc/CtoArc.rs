// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A thread-safe reference-counting pointer.
/// 'Arc' stands for 'Atomically Reference Counted'.
/// See Rust stdlib documentation.
pub struct CtoArc<Value: CtoSafe>
{
	persistent_memory_pointer: NonNull<CtoArcInner<Value>>,
}

impl<Value: CtoSafe> PersistentMemoryWrapper for CtoArc<Value>
{
	type PersistentMemory = CtoArcInner<Value>;
	
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

impl<Value: CtoSafe> CtoSafe for CtoArc<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_arc)
	}
}

unsafe impl<Value: CtoSafe + Sync + Send> Send for CtoArc<Value>
{
}

unsafe impl<Value: CtoSafe + Sync + Send> Sync for CtoArc<Value>
{
}

impl<Value: CtoSafe> Drop for CtoArc<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Because `fetch_sub` is already atomic, we do not need to synchronize with other threads unless we are going to delete the object.
		// This same logic applies to the below `fetch_sub` to the `weak` count.
		if self.persistent_memory().decrement_strong_reference_count() != 1
		{
			return;
		}
		
		// This fence is needed to prevent reordering of use of the data and deletion of the data.
		// Because it is marked `Release`, the decreasing of the reference count synchronizes with this `Acquire` fence.
		// This means that use of the data happens before decreasing the reference count, which happens before this fence, which happens before the deletion of the data.
		//
		// As explained in the [Boost documentation][1],
		//
		// > It is important to enforce any possible access to the object in one thread (through an existing reference) to *happen before* deleting the object in a different thread.
		// > This is achieved by a "release" operation after dropping a reference (any access to the object through this reference must obviously happened before), and an "acquire" operation before deleting the object.
		//
		// In particular, while the contents of an Arc are usually immutable, it is possible to have interior writes to something like a `CtoMutex<Value>`.
		// Since a `CtoMutex` is not acquired when it is deleted, we can't rely on its synchronization logic to make writes in thread A visible to a destructor running in thread B.
		//
		// Also note that the Acquire fence here could probably be replaced with an Acquire load, which could improve performance in highly-contended situations. See [2].
		//
		// [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
		// [2]: (https://github.com/rust-lang/rust/pull/41714)
		fence(Acquire);
		
		self.drop_slow();
	}
}

impl<Value: CtoSafe + PartialEq> PartialEq for CtoArc<Value>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		*(*self) == *(*other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		*(*self) != *(*other)
	}
}

impl<Value: CtoSafe + PartialOrd> PartialOrd for CtoArc<Value>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		(**self).partial_cmp(&**other)
	}
	
	#[inline(always)]
	fn lt(&self, other: &Self) -> bool
	{
		*(*self) < *(*other)
	}
	
	#[inline(always)]
	fn le(&self, other: &Self) -> bool
	{
		*(*self) <= *(*other)
	}
	
	#[inline(always)]
	fn gt(&self, other: &Self) -> bool
	{
		*(*self) > *(*other)
	}
	
	#[inline(always)]
	fn ge(&self, other: &Self) -> bool
	{
		*(*self) >= *(*other)
	}
}

impl<Value: CtoSafe + Ord> Ord for CtoArc<Value>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		(**self).cmp(&**other)
	}
}

impl<Value: CtoSafe + Eq> Eq for CtoArc<Value>
{
}

impl<Value: CtoSafe + Hash> Hash for CtoArc<Value>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state)
	}
}

impl<Value: CtoSafe + Display> Display for CtoArc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&**self, f)
	}
}

impl<Value: CtoSafe + Debug> Debug for CtoArc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(&**self, f)
	}
}

impl<Value: CtoSafe> Pointer for CtoArc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&(&**self as *const Value), f)
	}
}

impl<Value: CtoSafe> Deref for CtoArc<Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.persistent_memory().deref()
	}
}

impl<Value: CtoSafe> Borrow<Value> for CtoArc<Value>
{
	#[inline(always)]
	fn borrow(&self) -> &Value
	{
		&**self
	}
}

impl<Value: CtoSafe> AsRef<Value> for CtoArc<Value>
{
	#[inline(always)]
	fn as_ref(&self) -> &Value
	{
		&**self
	}
}

impl<Value: CtoSafe> Clone for CtoArc<Value>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		// Using a relaxed ordering is alright here, as knowledge of the original reference prevents other threads from erroneously deleting the object.
		//
		// As explained in the [Boost documentation][1], increasing the reference counter can always be done with memory_order_relaxed: new references to an object can only be formed from an existing reference, and passing an existing reference from one thread to another must already provide any required synchronization.
		//
		// [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
		let previous_strong_reference_count = self.persistent_memory().increment_strong_reference_count();
		
		CtoArcInner::<Value>::abort_if_maximum_number_of_references_exceeded(previous_strong_reference_count);
		
		Self
		{
			persistent_memory_pointer: self.persistent_memory_pointer,
		}
	}
}

impl<Value: CtoSafe + Clone> CtoArc<Value>
{
	/// Produces a clone of the data
	#[inline(always)]
	pub(crate) fn deep_clone(&self) -> Self
	{
		let cto_arc_inner = self.persistent_memory();
		let cto_pool_arc = &cto_arc_inner.cto_pool_arc;
		let persistent_memory_pointer = cto_pool_arc.pool_pointer().aligned_allocate().unwrap();
		
		unsafe
		{
			Self::initialize_persistent_memory::<(), _>(persistent_memory_pointer, cto_pool_arc, |value_mut_pointer, _cto_pool_arc|
			{
				write(value_mut_pointer, cto_arc_inner.value.clone());
				Ok(())
			}).unwrap()
		}
	}
}

impl<Value: CtoSafe> CtoArc<Value>
{
	/// Produces a clone of the data, customized.
	#[inline(always)]
	pub(crate) fn deep_clone_customized<CallbackError, DeepCloneCallback: FnOnce(*mut Value, &CtoPoolArc, &Value) -> Result<(), CallbackError>>(&self, deep_clone_initializer: DeepCloneCallback) -> Result<Self, CallbackError>
	{
		let cto_arc_inner = self.persistent_memory();
		let cto_pool_arc = &cto_arc_inner.cto_pool_arc;
		let persistent_memory_pointer = cto_pool_arc.pool_pointer().aligned_allocate().unwrap();
		
		unsafe
		{
			Self::initialize_persistent_memory(persistent_memory_pointer, cto_pool_arc, |value_mut_pointer, cto_pool_arc|
			{
				deep_clone_initializer(value_mut_pointer, cto_pool_arc, &cto_arc_inner.value)
			})
		}
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
			persistent_memory_pointer: NonNull::new_unchecked(CtoArcInner::from_raw_value_pointer(raw_value_pointer)),
		}
	}
	
	/// Creates a new [`WeakCtoArc`][weak] pointer to this value.
	///
	/// [weak]: struct.WeakCtoArc.html
	pub fn downgrade(this: &Self) -> WeakCtoArc<Value>
	{
		// This Relaxed is OK because we're checking the value in the CAS below.
		let mut weak_reference_count_or_lock = this.persistent_memory().weak_count_relaxed();
		
		loop
		{
			// check if the weak counter is currently "locked"; if so, spin.
			if CtoArcInner::<Value>::weak_count_is_locked(weak_reference_count_or_lock)
			{
				weak_reference_count_or_lock = this.persistent_memory().weak_count_relaxed();
				continue;
			}
			let weak_reference_count = weak_reference_count_or_lock;
			
			// Unlike with `clone()`, we need this to be an Acquire read to synchronize with the write coming from `is_unique`, so that the events prior to that write happen before this read.
			match this.persistent_memory().increment_weak_count_cas_acquire_relaxed(weak_reference_count)
			{
				Ok(_) => return WeakCtoArc
				{
					persistent_memory_pointer: Some(this.persistent_memory_pointer),
				},
				
				Err(was_actually_weak_reference_count_or_lock) => weak_reference_count_or_lock = was_actually_weak_reference_count_or_lock,
			}
		}
	}
	
	/// Gets the number of [`WeakCtoArc`][weak] pointers to this value.
	///
	/// [weak]: struct.WeakCtoArc.html
	///
	/// # Safety
	///
	/// This method by itself is safe, but using it correctly requires extra care.
	/// Another thread can change the weak count at any time, including potentially between calling this method and acting on the result.
	#[inline(always)]
	pub fn weak_count(this: &Self) -> usize
	{
		let weak_reference_count_or_lock = this.persistent_memory().weak_count_seqcst();
		
		let weak_reference_count = if CtoArcInner::<Value>::weak_count_is_locked(weak_reference_count_or_lock)
		{
			CtoArcInner::<Value>::WeakCountJustBeforeLock
		}
		else
		{
			weak_reference_count_or_lock
		};
		
		weak_reference_count - CtoArcInner::<Value>::WeakCountJustBeforeLock
	}
	
	/// Gets the number of strong (`CtoArc`) pointers to this value.
	///
	/// # Safety
	///
	/// This method by itself is safe, but using it correctly requires extra care.
	/// Another thread can change the strong count at any time, including potentially between calling this method and acting on the result.
	#[inline(always)]
	pub fn strong_count(this: &Self) -> usize
	{
		this.persistent_memory().strong_count_seqcst()
	}
	
	/// Returns a mutable reference to the inner value, if there are no other `CtoArc` or [`WeakCtoArc`][weak] pointers to the same value.
	///
	/// Returns `None` otherwise, because it is not safe to mutate a shared value.
	///
	/// [weak]: struct.WeakCtoArc.html
	#[inline(always)]
	pub fn get_mut(this: &mut Self) -> Option<&mut Value>
	{
		if this.is_unique()
		{
			Some(this.persistent_memory_mut().deref_mut())
		}
		else
		{
			None
		}
	}
	
	/// Returns true if the two `CtoArc`s point to the same value (not just values that compare as equal).
	#[inline(always)]
	pub fn ptr_eq(this: &Self, other: &Self) -> bool
	{
		this.persistent_memory_pointer() == other.persistent_memory_pointer()
	}
	
	/// Determine whether this is the unique reference (including weak refs) to the underlying data.
	///
	/// Note that this requires locking the weak ref count.
	#[inline(always)]
	fn is_unique(&mut self) -> bool
	{
		// Lock the weak pointer count if we appear to be the sole weak pointer holder.
		//
		// The acquire label here ensures a happens-before relationship with any writes to `strong` prior to decrements of the `weak` count (via drop, which uses Release).
		if self.persistent_memory().try_to_lock_weak_count()
		{
			// Due to the previous acquire read, this will observe any writes to `strong` that were due to upgrading weak pointers; only strong clones remain, which require that the strong count is > 1 anyway.
			let is_unique = self.persistent_memory().strong_count_relaxed() == 1;
			
			// The release write here synchronizes with a read in `downgrade`, effectively preventing the above read of `strong` from happening after the write.
			self.persistent_memory().unlock_weak_count();
			
			is_unique
		}
		else
		{
			false
		}
	}
	
	// Non-inlined part of `drop`.
	#[inline(never)]
	fn drop_slow(&mut self)
	{
		let ptr = self.persistent_memory_pointer();
		
		// Destroy the value at this time, even though we may not free the allocation itself (there may still be weak pointers lying around).
		unsafe { drop_in_place(self.persistent_memory_mut().deref_mut()) };
		
		if self.persistent_memory().decrement_weak_reference_count() == CtoArcInner::<Value>::WeakCountJustBeforeLock
		{
			fence(Acquire);
			
			let pool_pointer = self.persistent_memory().cto_pool_arc.pool_pointer();
			
			pool_pointer.free(ptr);
		}
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoArcInner<Value>
	{
		// This unsafety is ok because while this arc is alive we're guaranteed that the inner pointer is valid.
		// Furthermore, we know that the `CtoArcInner` structure itself is `Sync` because the inner data is `Sync` as well, so we're ok loaning out an immutable pointer to these contents.
		unsafe { self.persistent_memory_pointer.as_ref() }
	}
	
	// Only valid when `self.is_unique()` is true or inside `drop_slow()`, ie when there is only one strong reference.
	#[inline(always)]
	fn persistent_memory_mut(&mut self) -> &mut CtoArcInner<Value>
	{
		// This unsafety is ok because we're guaranteed that the pointer returned is the *only* pointer that will ever be returned to Value.
		// Our reference count is guaranteed to be 1 at this point, and we required the CtoArc itself to be `mut`, so we're returning the only possible reference to the inner data.
		unsafe { self.persistent_memory_pointer.as_mut() }
	}
	
	#[inline(always)]
	fn persistent_memory_pointer(&self) -> *mut CtoArcInner<Value>
	{
		self.persistent_memory_pointer.as_ptr()
	}
}
