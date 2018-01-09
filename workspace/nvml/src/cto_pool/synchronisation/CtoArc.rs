// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Similar to a Rust Arc but allocated in a persistent memory CTO Pool.
pub struct CtoArc<T: CtoSafe>
{
	persistent_memory_pointer: *mut CtoArcInner<T>
}

impl<T: CtoSafe> Clone for CtoRc<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		// Using a relaxed ordering is alright here, as knowledge of the original reference prevents other threads from erroneously deleting the object.
		//
		// As explained in the [Boost documentation][1], increasing the reference counter can always be done with memory_order_relaxed. New references to an object can only be formed from an existing/ reference, and passing an existing reference from one thread to another must already provide any required synchronization.
		//
		// [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
		let old_size = self.persistent_memory().strong_counter.fetch_add(1, Relaxed);
		
		// However we need to guard against massive refcounts in case someone is `mem::forget`ing Arcs.
		// If we don't do this the count can overflow and users will use-after free.
		// We racily saturate to `isize::MAX` on the assumption that there aren't ~2 billion threads incrementing the reference count at once.
		// This branch will never be taken in any realistic program.
		//
		// We abort because such a program is incredibly degenerate, and we don't care to support it.
		if old_size > Self::MaximumNumberOfReferences
		{
			unsafe
			{
				abort();
			}
		}
		
		Self
		{
			persistent_memory_pointer: self.persistent_memory_pointer,
		}
	}
}

unsafe impl<T: CtoSafe + Sync + Send> Send for CtoArc<T>
{
}

unsafe impl<T: CtoSafe + Sync + Send> Sync for CtoArc<T>
{
}

impl<T: CtoSafe + PartialEq> PartialEq for CtoArc<T>
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

impl<T: CtoSafe + Eq> Eq for CtoArc<T>
{
}

impl<T: CtoSafe + PartialOrd> PartialOrd for CtoArc<T>
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

impl<T: CtoSafe + Ord> Ord for CtoArc<T>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

impl<T: CtoSafe + Hash> Hash for CtoArc<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state);
	}
}

impl<T: CtoSafe + Display> Display for CtoArc<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe + Debug> Debug for CtoArc<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe> Pointer for CtoArc<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&self.deref(), f)
	}
}

impl<T: CtoSafe> Deref for CtoArc<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.persistent_memory().deref()
	}
}

impl<T: CtoSafe> Borrow<T> for CtoArc<T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe> AsRef<T> for CtoArc<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe> CtoArc<T>
{
	// A soft limit on the amount of references that may be made to an `Arc`.
	//
	// Going above this limit will abort your program (although not necessarily) at _exactly_ `MAX_REFCOUNT + 1` references.
	const MaximumNumberOfReferences: usize = (isize::MAX) as usize;
	
	/// Is this object unique, ie there is only one strong reference and no weak references?
	#[inline(always)]
	pub fn is_unique(this: &Self) -> bool
	{
		this.persistent_memory().is_unique()
	}
	
	/// Downgrades this strong reference to a weak reference.
	#[inline(always)]
	pub fn downgrade(this: &Self) -> WeakCtoArc<T>
	{
		// This Relaxed is OK because we're checking the value in the CAS below.
		let mut cur = this.persistent_memory().weak_counter.load(Relaxed);
		
		loop
		{
			// check if the weak counter is currently "locked"; if so, spin.
			if cur == usize::MAX
			{
				cur = this.persistent_memory().weak_counter.load(Relaxed);
				continue;
			}
			
			// Unlike with Clone(), we need this to be an Acquire read to synchronize with the write coming from `is_unique`, so that the events prior to that write happen before this read.
			match this.persistent_memory().weak_counter.compare_exchange_weak(cur, cur + 1, Acquire, Relaxed)
			{
				Ok(_) => return WeakCtoArc(this.persistent_memory),
				Err(old) => cur = old,
			}
		}
	}
	
	/// If this object is unique (see `is_unique()`) then will return Some otherwise None.
	#[inline(always)]
	pub fn get_mut(this: &mut Self) -> Option<&mut T>
	{
		if this.is_unique()
		{
			// This unsafety is ok because we're guaranteed that the pointer returned is the *only* pointer that will ever be returned to T.
			// Our reference count is guaranteed to be 1 at this point, and we required the Arc itself to be `mut`, so we're returning the only possible reference to the inner data.
			unsafe
			{
				Some(&mut this.ptr.as_mut().data)
			}
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
	fn persistent_memory(&self) -> &CtoArcInner<T>
	{
		unsafe { &*self.persistent_memory_pointer }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&self) -> &mut CtoArcInner<T>
	{
		unsafe { &mut *self.persistent_memory_pointer }
	}
}
