// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A type providing atomic storage and retrieval of a `CtoArc<T>`.
/// This current implementation works because:-
/// 1. The `size_of::<CtoArc<>()` is usize;
/// 2. A valid `CtoArc` has one field, which is a (a) a pointer and (b) a pointer that can never validly be null
/// ie that a CtoArc is in union with AtomicUsize.
#[derive(Debug)]
pub struct CtoArcCell<Value: CtoSafe>(AtomicUsize, PhantomData<Value>);

impl<Value: CtoSafe> Drop for CtoArcCell<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.take();
	}
}

impl<Value: CtoSafe> CtoSafe for CtoArcCell<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		match self.0.load(SeqCst)
		{
			Self::InvalidValueForCtoArc => panic!("InvalidValueForCtoArc was persisted"),
			bytes =>
			{
				let mut cto_arc = Self::usize_to_cto_arc(bytes);
				cto_arc.cto_pool_opened(cto_pool_arc);
				forget(cto_arc);
			}
		}
	}
}

impl<Value: CtoSafe> CtoArcCell<Value>
{
	const InvalidValueForCtoArc: usize = 0;
	
	/// Creates a new `CtoArcCell`.
	#[inline(always)]
	pub fn new(cto_arc: CtoArc<Value>) -> CtoArcCell<Value>
	{
		CtoArcCell
		(
			AtomicUsize::new(Self::cto_arc_to_usize(cto_arc)),
			PhantomData,
		)
	}
	
	/// Returns a cheap copy of the value stored by the `CtoArcCell`.
	#[inline(always)]
	pub fn get(&self) -> CtoArc<Value>
	{
		let cto_arc = self.take();
		let out = cto_arc.clone();
		self.put(cto_arc);
		out
	}
	
	/// Stores a new value in the `CtoArcCell`, returning the previous value.
	#[inline(always)]
	pub fn set(&self, cto_arc: CtoArc<Value>) -> CtoArc<Value>
	{
		let old_cto_arc = self.take();
		self.put(cto_arc);
		old_cto_arc
	}
	
	#[inline(always)]
	fn take(&self) -> CtoArc<Value>
	{
		// Whilst there is nothing to take, spin-loops.
		// When another thread has called `take()`, this will be the case.
		loop
		{
			match self.0.swap(Self::InvalidValueForCtoArc, Acquire)
			{
				Self::InvalidValueForCtoArc => {}
				bytes => return Self::usize_to_cto_arc(bytes)
			}
		}
	}
	
	#[inline(always)]
	fn put(&self, cto_arc: CtoArc<Value>)
	{
		debug_assert_eq!(self.0.load(SeqCst), Self::InvalidValueForCtoArc);
		
		self.0.store(Self::cto_arc_to_usize(cto_arc), Release);
	}
	
	#[inline(always)]
	fn cto_arc_to_usize(cto_arc: CtoArc<Value>) -> usize
	{
		unsafe { transmute(cto_arc) }
	}
	
	#[inline(always)]
	fn usize_to_cto_arc(bytes: usize) -> CtoArc<Value>
	{
		unsafe { transmute(bytes) }
	}
}
