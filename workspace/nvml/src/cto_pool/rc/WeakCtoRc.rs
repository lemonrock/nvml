// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Very similar to Rust's Rc Weak.
pub struct WeakCtoRc<Value: CtoSafe>
{
	persistent_memory_pointer: Option<NonNull<CtoRcInner<Value>>>,
}

impl<Value: CtoSafe> Drop for WeakCtoRc<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(persistent_memory_pointer) = self.persistent_memory_pointer
		{
			let cto_rc_inner = unsafe { persistent_memory_pointer.as_ref() };
			
			cto_rc_inner.weak_count_decrement();
			
			if cto_rc_inner.strong_count() == 0 && cto_rc_inner.weak_count() == 0
			{
				let pool_pointer = cto_rc_inner.cto_pool_arc.pool_pointer();
				
				pool_pointer.free(persistent_memory_pointer.as_ptr());
			}
		}
	}
}

impl<Value: CtoSafe> Clone for WeakCtoRc<Value>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		if let Some(shared_cto_rc_inner) = self.persistent_memory_pointer
		{
			(unsafe { shared_cto_rc_inner.as_ref() }).weak_count_increment();
		}
		
		WeakCtoRc
		{
			persistent_memory_pointer: self.persistent_memory_pointer
		}
	}
}

impl<Value: CtoSafe + Debug> Debug for WeakCtoRc<Value>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(WeakCtoRc)")
	}
}

impl<Value: CtoSafe> Default for WeakCtoRc<Value>
{
	/// Constructs a new `WeakCtoRc<Value>`, allocating memory for `Value` without initializing it. Calling `upgrade()` on the return value always gives `None`.
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl<Value: CtoSafe> WeakCtoRc<Value>
{
	/// Constructs a new `WeakCtoRc<Value>`, allocating memory for `Value` without initializing it. Calling `upgrade()` on the return value always gives `None`.
	#[inline(always)]
	pub fn new() -> Self
	{
		WeakCtoRc
		{
			persistent_memory_pointer: None,
		}
	}
	
	/// Upgrades a weak reference to a strong one.
	/// Returns None if only weak references remain, or this instance was created with `new()` or `default()`.
	#[inline(always)]
	pub fn upgrade(&self) -> Option<CtoRc<Value>>
	{
		self.persistent_memory_pointer.map(|persistent_memory_pointer|
		{
			let cto_rc_inner = unsafe { persistent_memory_pointer.as_ref() };
			cto_rc_inner.strong_count_increment();
			CtoRc
			{
				persistent_memory_pointer,
			}
		})
	}
	
	/// How many strong references are there?
	/// Returns None if this instance was created with `new()` or `default()`.
	#[inline(always)]
	pub fn strong_count(&self) -> Option<usize>
	{
		self.persistent_memory_pointer.map(|persistent_memory_pointer| unsafe { persistent_memory_pointer.as_ref() }.strong_count())
	}
	
	/// How many weak references are there?
	/// Returns None if this instance was created with `new()` or `default()`.
	/// If Some is returned then the value will be a minimum of 1.
	#[inline(always)]
	pub fn weak_count(&self) -> Option<usize>
	{
		self.persistent_memory_pointer.map(|persistent_memory_pointer| unsafe { persistent_memory_pointer.as_ref() }.weak_count())
	}
}
