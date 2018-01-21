// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// `WeakCtoArc` is a version of [`CtoArc`] that holds a non-owning reference to the managed value.
/// The value is accessed by calling [`upgrade`] on the `WeakCtoArc` pointer, which returns an `Option``<`[`CtoArc`]`<Value>>`.
///
/// Since a `WeakCtoArc` reference does not count towards ownership, it will not prevent the inner value from being dropped, and `WeakCtoArc` itself makes no guarantees about the value still being present and may return `None` when [`upgrade`]d.
///
/// A `WeakCtoArc` pointer is useful for keeping a temporary reference to the value within [`CtoArc`] without extending its lifetime.
/// It is also used to prevent circular references between [`CtoArc`] pointers, since mutual owning references would never allow either [`CtoArc`] to be dropped.
/// For example, a tree could have strong [`CtoArc`] pointers from parent nodes to children, and `WeakCtoArc` pointers from children back to their parents.
///
/// The typical way to obtain a `Weak` pointer is to call [`CtoArc::downgrade`].
///
/// [`CtoArc`]: struct.Arc.html
/// [`CtoArc::downgrade`]: struct.Arc.html#method.downgrade
/// [`upgrade`]: struct.Weak.html#method.upgrade
pub struct WeakCtoArc<Value: CtoSafe>
{
	persistent_memory_pointer: Option<NonNull<CtoArcInner<Value>>>,
}

unsafe impl<Value: CtoSafe + Sync + Send> Send for WeakCtoArc<Value>
{
}

unsafe impl<Value: CtoSafe + Sync + Send> Sync for WeakCtoArc<Value>
{
}

impl<Value: CtoSafe + Debug> Debug for WeakCtoArc<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(Weak)")
	}
}

impl<Value: CtoSafe> Clone for WeakCtoArc<Value>
{
	/// Makes a clone of the `WeakCtoArc` pointer that points to the same value.
	#[inline(always)]
	fn clone(&self) -> Self
	{
		match self.persistent_memory_pointer
		{
			None => Self
			{
				persistent_memory_pointer: None
			},
			
			Some(persistent_memory_pointer) =>
			{
				let cto_arc_inner = unsafe { persistent_memory_pointer.as_ref() };
				
				// See comments in CtoArc::clone() for why this is relaxed.
				// This can use a `fetch_add` (ignoring the lock) because the weak count is only locked when there are *no other* weak pointers in existence.
				// (So we can't be running this code in that case).
				let previous_weak_reference_count = cto_arc_inner.increment_weak_reference_count();
				
				CtoArcInner::<Value>::abort_if_maximum_number_of_references_exceeded(previous_weak_reference_count);
				
				Self
				{
					persistent_memory_pointer: Some(persistent_memory_pointer),
				}
			}
		}
	}
}

impl<Value: CtoSafe> Drop for WeakCtoArc<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(persistent_memory_pointer) = self.persistent_memory_pointer
		{
			let ptr = persistent_memory_pointer.as_ptr();
			
			let cto_arc_inner = unsafe { persistent_memory_pointer.as_ref() };
			
			// If we find out that we were the last weak pointer, then its time to deallocate the data entirely.
			// See the discussion in `CtoArc::drop()` about the memory orderings.
			//
			// It's not necessary to check for the locked state here, because the weak count can only be locked if there was precisely one weak ref, meaning that drop could only subsequently run ON that remaining weak ref, which can only happen after the lock is released.
			if cto_arc_inner.decrement_weak_reference_count() == CtoArcInner::<Value>::WeakCountJustBeforeLock
			{
				fence(Acquire);
				
				let pool_pointer = cto_arc_inner.cto_pool_arc.pool_pointer();
				
				pool_pointer.free(ptr);
			}
		}
	}
}

impl<Value: CtoSafe> Default for WeakCtoArc<Value>
{
	/// Constructs a new `WeakCtoArc<Value>`.
	/// Calling [`upgrade`] on the return value always gives `None`.
	///
	/// [`upgrade`]: struct.Weak.html#method.upgrade
	#[inline(always)]
	fn default() -> Self
	{
		WeakCtoArc::new()
	}
}

impl<Value: CtoSafe> WeakCtoArc<Value>
{
	/// Constructs a new `WeakCtoArc<Value>`.
	/// Calling [`upgrade`] on the return value always gives `None`.
	///
	/// [`upgrade`]: struct.WeakCtoArc.html#method.upgrade
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			persistent_memory_pointer: None,
		}
	}
	
	/// Attempts to upgrade the `WeakCtoArc` pointer to an [`CtoArc`], extending the lifetime of the value if successful.
	///
	/// Returns `None` if the value has since been dropped.
	///
	/// [`CtoArc`]: struct.Arc.html
	#[inline(always)]
	pub fn upgrade(&self) -> Option<CtoArc<Value>>
	{
		match self.persistent_memory_pointer
		{
			None => None,
			
			Some(persistent_memory_pointer) =>
			{
				let cto_arc_inner = unsafe { persistent_memory_pointer.as_ref() };
				
				// Relaxed load because any write of 0 that we can observe leaves the field in a permanently zero state (so a "stale" read of 0 is fine), and any other value is confirmed via the CAS below.
				let mut strong_reference_count = cto_arc_inner.strong_count_relaxed();
				
				// We use a CAS loop to increment the strong count instead of a `fetch_add` because once the count hits 0 it must never be above 0.
				loop
				{
					if strong_reference_count == 0
					{
						return None;
					}
					
					CtoArcInner::<Value>::abort_if_maximum_number_of_references_exceeded(strong_reference_count);
					
					// Relaxed is valid for the same reason it is on `CtoArc`'s Clone impl.
					match cto_arc_inner.increment_strong_count_cas_relaxed_relaxed(strong_reference_count)
					{
						Ok(_) => return Some
						(
							CtoArc
							{
								persistent_memory_pointer,
							}
						),
						
						Err(was_actually_strong_reference_count) => strong_reference_count = was_actually_strong_reference_count,
					}
				}
			}
		}
	}
}
