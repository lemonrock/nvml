// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// An atomic option which wraps CtoBox<Value>. Useful for fields initially empty, or as a rendezvous between two threads.
#[derive(Debug)]
pub struct CtoAtomicOption<Value: CtoSafe>
{
	inner_cto_box: AtomicPtr<Value>,
}

impl<Value: CtoSafe> CtoSafe for CtoAtomicOption<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let value = self.inner_cto_box.load(SeqCst);
		if value.is_not_null()
		{
			let mut cto_box = unsafe { CtoBox::from_raw(value) };
			cto_box.cto_pool_opened(cto_pool_arc);
			forget(cto_box);
		}
	}
}

unsafe impl<Value: CtoSafe + Send> Send for CtoAtomicOption<Value>
{
}

unsafe impl<Value: CtoSafe + Send> Sync for CtoAtomicOption<Value>
{
}

impl<Value: CtoSafe> Drop for CtoAtomicOption<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let value = self.inner_cto_box.load(Relaxed);
		if value.is_not_null()
		{
			drop(unsafe { CtoBox::from_raw(value) });
		}
	}
}

impl<Value: CtoSafe> Default for CtoAtomicOption<Value>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::none()
	}
}

impl<Value: CtoSafe> CtoAtomicOption<Value>
{
	/// Creates an empty CtoAtomicOption.
	#[inline(always)]
	pub fn none() -> Self
	{
		Self
		{
			inner_cto_box: AtomicPtr::new(null_mut())
		}
	}
	
	/// Creates an CtoAtomicOption with an initial CtoBox.
	#[inline(always)]
	pub fn some(value: CtoBox<Value>) -> Self
	{
		let this = Self::none();
		this.swap(value, Relaxed);
		this
	}
	
	/// Swaps with the replacement value, returning the previous value.
	#[inline(always)]
	pub fn swap(&self, replacement: CtoBox<Value>, ordering: atomic::Ordering) -> Option<CtoBox<Value>>
	{
		self.swap_inner(CtoBox::into_raw(replacement), ordering)
	}
	
	/// Takes the CtoBox, replacing with None (null) behind.
	#[inline(always)]
	pub fn take(&self, ordering: atomic::Ordering) -> Option<CtoBox<Value>>
	{
		self.swap_inner(null_mut(), ordering)
	}
	
	#[inline(always)]
	fn swap_inner(&self, replacement_pointer: *mut Value, ordering: atomic::Ordering) -> Option<CtoBox<Value>>
	{
		let previous = self.inner_cto_box.swap(replacement_pointer, ordering);
		
		if previous.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CtoBox::from_raw(previous) })
		}
	}
}
