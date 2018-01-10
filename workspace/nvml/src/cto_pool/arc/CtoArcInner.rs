// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// #[repr(C)] is required otherwise `from_raw_value_pointer()` will be very broken indeed.
#[repr(C)]
pub(crate) struct CtoArcInner<Value: CtoSafe>
{
	// Field order matters. `value: Value` must be first otherwise `from_raw_value_pointer()` will be very broken indeed.
	value: Value,
	strong_counter: AtomicUsize,
	weak_counter: AtomicUsize,
	cto_pool_arc: CtoPoolArc,
}

unsafe impl<Value: CtoSafe + Sync + Send> Send for CtoArcInner<Value>
{
}

unsafe impl<Value: CtoSafe + Sync + Send> Sync for CtoArcInner<Value>
{
}

impl<Value: CtoSafe> Deref for CtoArcInner<Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.value
	}
}

impl<Value: CtoSafe> DerefMut for CtoArcInner<Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.value
	}
}

impl<Value: CtoSafe> CtoArcInner<Value>
{
	#[inline(always)]
	fn common_initialization(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		cto_pool_arc.write(&mut self.cto_pool_arc);
	}
	
	#[inline(always)]
	fn allocated<InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&mut self, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<(), InitializationError>
	{
		unsafe { write(&mut self.strong_counter, AtomicUsize::new(1)) };
		
		// Start the weak pointer count as 1 which is the weak pointer that's held by all the strong pointers.
		unsafe { write(&mut self.weak_counter, AtomicUsize::new(Self::WeakCountJustBeforeLock)) };
		
		self.common_initialization(cto_pool_arc);
		
		initializer(&mut self.value, cto_pool_arc)
	}
	
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.common_initialization(cto_pool_arc);
		
		self.value.cto_pool_opened(cto_pool_arc)
	}
	
	#[inline(always)]
	fn into_raw_value_pointer(&mut self) -> *mut Value
	{
		self.deref_mut()
	}
	
	#[inline(always)]
	fn from_raw_value_pointer(raw_value_pointer: *mut Value) -> *mut Self
	{
		// Works because Value is the first field and we use #[repr(C)]
		raw_value_pointer as *mut Self
	}
	
	// A soft limit on the amount of references that may be made to a `CtoArc`.
	// Going above this limit will abort your program (although not necessarily) at _exactly_ `MaximumNumberOfReferences + 1` references.
	// `MaximumNumberOfReferences` is less than `WeakCounterLockSentinel` to allow `WeakCounterLockSentinel` to be used as a lock sentinel.
	// `MaximumNumberOfReferences` must also be at least one less than `WeakCounterLockSentinel` so that `fetch_add`, when used for incrementing reference counts, has enough `headroom`, without overflowing.
	const MaximumNumberOfReferences: usize = (isize::MAX) as usize;
	
	/// This value is used as a sentinel to `lock` the weak count.
	/// It must be greater than `MaximumNumberOfReferences`.
	const WeakCounterLockSentinel: usize = usize::MAX;
	
	const WeakCountJustBeforeLock: usize = 1;
	
	// We need to guard against massive ref-counts in case someone is `mem::forget`ing CtoArc or WeakCtoArc instances.
	// If we don't do this the count can overflow and users will use-after free.
	// We racily saturate to `MaximumNumberOfReferences` on the assumption that there aren't ~2 billion threads incrementing the reference count at once.
	// `MaximumNumberOfReferences` is less than `WeakCounterLockSentinel` to allow `WeakCounterLockSentinel` to be used as a lock sentinel.
	// This branch will never be taken in any realistic program.
	//
	// We abort because such a program is incredibly degenerate, and we don't care to support it.
	#[inline(always)]
	fn abort_if_maximum_number_of_references_exceeded(reference_count: usize)
	{
		if reference_count > Self::MaximumNumberOfReferences
		{
			abort();
		}
	}
	
	#[inline(always)]
	fn weak_count_is_locked(locked_or_weak_count: usize) -> bool
	{
		Self::WeakCounterLockSentinel == locked_or_weak_count
	}
	
	// Returns previous reference count
	#[inline(always)]
	fn increment_weak_reference_count(&self) -> usize
	{
		self.weak_counter.fetch_add(1, Relaxed)
	}
	
	// Returns previous reference count
	#[inline(always)]
	fn decrement_weak_reference_count(&self) -> usize
	{
		self.weak_counter.fetch_sub(1, Relaxed)
	}
	
	#[inline(always)]
	fn weak_count_relaxed(&self) -> usize
	{
		self.weak_counter.load(Relaxed)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn weak_count_seqcst(&self) -> usize
	{
		self.weak_counter.load(SeqCst)
	}
	
	#[inline(always)]
	fn strong_count_relaxed(&self) -> usize
	{
		self.strong_counter.load(Relaxed)
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn strong_count_seqcst(&self) -> usize
	{
		self.strong_counter.load(SeqCst)
	}
	
	#[inline(always)]
	fn increment_weak_count_cas_acquire_relaxed(&self, current_reference_count: usize) -> Result<usize, usize>
	{
		self.weak_counter.compare_exchange_weak(current_reference_count, current_reference_count + 1, Acquire, Relaxed)
	}
	
	#[inline(always)]
	fn try_to_lock_weak_count(&self) -> bool
	{
		self.weak_counter.compare_exchange(Self::WeakCountJustBeforeLock, Self::WeakCounterLockSentinel, Acquire, Relaxed).is_ok()
	}
	
	#[inline(always)]
	fn unlock_weak_count(&self)
	{
		self.weak_counter.store(Self::WeakCountJustBeforeLock, Release);
	}
	
	#[inline(always)]
	fn increment_strong_count_cas_relaxed_relaxed(&self, current_reference_count: usize) -> Result<usize, usize>
	{
		self.strong_counter.compare_exchange_weak(current_reference_count, current_reference_count + 1, Relaxed, Relaxed)
	}
	
	// Returns previous reference count
	#[inline(always)]
	fn increment_strong_reference_count(&self) -> usize
	{
		self.strong_counter.fetch_add(1, Relaxed)
	}
	
	// Returns previous reference count
	#[inline(always)]
	fn decrement_strong_reference_count(&self) -> usize
	{
		self.strong_counter.fetch_sub(1, Release)
	}
}
