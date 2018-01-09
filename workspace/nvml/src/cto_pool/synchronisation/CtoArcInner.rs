// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug)]
struct CtoArcInner<T: CtoSafe>
{
	strong_counter: AtomicUsize,
	weak_counter: AtomicUsize,
	cto_pool_inner: Arc<CtoPoolInner>,
	value: T,
}

impl<T: CtoSafe> CtoSafe for CtoArcInner<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.cto_pool_inner = cto_pool_inner.clone();
		
		self.value.reinitialize(cto_pool_inner)
	}
}

unsafe impl<T: CtoSafe + Sync + Send> Send for CtoArcInner<T>
{
}

unsafe impl<T: CtoSafe + Sync + Send> Sync for CtoArcInner<T>
{
}

impl<T: CtoSafe> Deref for CtoArcInner<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.value
	}
}

impl<T: CtoSafe> DerefMut for CtoArcInner<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.value
	}
}



impl<T: CtoSafe> CtoArcInner<T>
{
	
	#[inline(always)]
	pub(crate) fn is_unique(&self) -> bool
	{
		// lock the weak pointer count if we appear to be the sole weak pointer holder.
		//
		// The acquire label here ensures a happens-before relationship with any writes to `strong` prior to decrements of the `weak` count (via drop, which uses Release).
		if self.weak_counter.compare_exchange(1, usize::MAX, Acquire, Relaxed).is_ok()
		{
			// Due to the previous acquire read, this will observe any writes to `strong` that were due to upgrading weak pointers; only strong clones remain, which require that the strong count is > 1 anyway.
			let is_unique = self.strong_counter.load(Relaxed) == 1;
			
			// The release write here synchronizes with a read in `downgrade`, effectively preventing the above read of `strong` from happening after the write.
			
			// Release the lock.
			self.inner().weak.store(1, Release);
			
			is_unique
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(&mut T) -> Result<(), InitializationError>>(&mut self, cto_pool_inner: &Arc<CtoPoolInner>, initializer: Initializer) -> Result<(), InitializationError>
	{
		self.strong_counter = CtoArcCounter::default();
		self.weak_counter = CtoArcCounter::default();
		
		self.cto_pool_inner = cto_pool_inner.clone();
		initializer(&mut self.value)
	}
}
