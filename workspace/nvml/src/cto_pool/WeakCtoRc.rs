// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Very similar to Rust's Rc Weak.
pub struct WeakCtoRc<T: CtoSafe>(*mut CtoRcInner<T>);

impl<T: CtoSafe> Drop for WeakCtoRc<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if let Some(extant) = self.dereference()
		{
			extant.weak_count_decrement();
		}
	}
}

impl<T: CtoSafe> Clone for WeakCtoRc<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		if let Some(extant) = self.dereference()
		{
			extant.weak_count_increment();
		}
		
		WeakCtoRc(self.0)
	}
}

impl<T: CtoSafe + Debug> Debug for WeakCtoRc<T>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "(WeakCtoRc)")
	}
}

impl<T: CtoSafe> Default for WeakCtoRc<T>
{
	/// Constructs a new `WeakCtoRc<T>`, allocating memory for `T` without initializing it. Calling `upgrade()` on the return value always gives `None`.
	#[inline(always)]
	fn default() -> Self
	{
		Self::new()
	}
}

impl<T: CtoSafe> WeakCtoRc<T>
{
	/// Constructs a new `WeakCtoRc<T>`, allocating memory for `T` without initializing it. Calling `upgrade()` on the return value always gives `None`.
	#[inline(always)]
	pub fn new() -> Self
	{
		WeakCtoRc(null_mut())
	}
	
	/// Upgrades a weak reference to a strong one.
	/// Returns None if only weak references remain, or this instance was created with `new()` or `default()`.
	#[inline(always)]
	pub fn upgrade(&self) -> Option<CtoRc<T>>
	{
		self.dereference().map(|cto_rc_inner|
		{
			cto_rc_inner.strong_count_increment();
			CtoRc
			{
				persistent_memory_pointer: self.0,
			}
		})
	}
	
	/// How many strong references are there?
	/// Returns None if this instance was created with `new()` or `default()`.
	#[inline(always)]
	pub fn strong_count(&self) -> Option<usize>
	{
		self.dereference().map(|cto_rc_inner| cto_rc_inner.strong_count())
	}
	
	/// How many weak references are there?
	/// Returns None if this instance was created with `new()` or `default()`.
	/// If Some is returned then the value will be a minimum of 1.
	#[inline(always)]
	pub fn weak_count(&self) -> Option<usize>
	{
		self.dereference().map(|cto_rc_inner| cto_rc_inner.weak_count())
	}
	
	#[inline(always)]
	fn dereference(&self) -> Option<&CtoRcInner<T>>
	{
		if self.0.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { &*self.0 })
		}
	}
}
