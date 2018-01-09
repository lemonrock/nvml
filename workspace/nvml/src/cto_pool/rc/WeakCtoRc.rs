// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Very similar to Rust's Rc Weak.
pub struct WeakCtoRc<Value: CtoSafe>(Option<Shared<CtoRcInner<Value>>>);

impl<Value: CtoSafe> Drop for WeakCtoRc<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(shared_cto_rc_inner) = self.0
		{
			(unsafe { shared_cto_rc_inner.as_ref() }).weak_count_decrement();
		}
	}
}

impl<Value: CtoSafe> Clone for WeakCtoRc<Value>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		if let Some(shared_cto_rc_inner) = self.0
		{
			(unsafe { shared_cto_rc_inner.as_ref() }).weak_count_increment();
		}
		
		WeakCtoRc(self.0)
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
		WeakCtoRc(None)
	}
	
	/// Upgrades a weak reference to a strong one.
	/// Returns None if only weak references remain, or this instance was created with `new()` or `default()`.
	#[inline(always)]
	pub fn upgrade(&self) -> Option<CtoRc<Value>>
	{
		self.0.map(|shared_cto_rc_inner|
		{
			let cto_rc_inner = unsafe { shared_cto_rc_inner.as_ref() };
			cto_rc_inner.strong_count_increment();
			CtoRc
			{
				persistent_memory_pointer: shared_cto_rc_inner,
			}
		})
	}
	
	/// How many strong references are there?
	/// Returns None if this instance was created with `new()` or `default()`.
	#[inline(always)]
	pub fn strong_count(&self) -> Option<usize>
	{
		self.0.map(|shared_cto_rc_inner| unsafe { shared_cto_rc_inner.as_ref() }.strong_count())
	}
	
	/// How many weak references are there?
	/// Returns None if this instance was created with `new()` or `default()`.
	/// If Some is returned then the value will be a minimum of 1.
	#[inline(always)]
	pub fn weak_count(&self) -> Option<usize>
	{
		self.0.map(|shared_cto_rc_inner| unsafe { shared_cto_rc_inner.as_ref() }.weak_count())
	}
}
