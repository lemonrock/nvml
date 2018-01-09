// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A guard to ensure the CTO pool is not dropped too soon.
/// Similar to a Rust Arc, but altered to address the fact that it will end-up in persistent storage.
/// Also provides allocation methods.
pub struct CtoPoolArc
{
	cto_pool_arc_inner: Shared<CtoPoolArcInner>,
}

impl Drop for CtoPoolArc
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unsafe { self.cto_pool_arc_inner.as_mut() }.release()
		{
			drop(unsafe { Box::from_unique(Unique::from(self.cto_pool_arc_inner.as_mut())) });
		}
	}
}

impl Clone for CtoPoolArc
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		let mut guard = self.cto_pool_arc_inner;
		
		unsafe { guard.as_mut() }.acquire();
		
		Self
		{
			cto_pool_arc_inner: self.cto_pool_arc_inner,
		}
	}
}

impl CtoPoolArc
{
	#[inline(always)]
	fn new(pool_pointer: *mut PMEMctopool) -> Self
	{
		let cto_pool_alloc_arc = Box::new
		(
			CtoPoolArcInner
			{
				pool_pointer,
				counter: AtomicUsize::new(1),
			}
		);
		
		Self
		{
			cto_pool_arc_inner: Box::into_unique(cto_pool_alloc_arc).into(),
		}
	}
	
	/// Pointer to CTO pool from FFI `libpmemcto`.
	#[inline(always)]
	pub fn pool_pointer(&self) -> *mut PMEMctopool
	{
		unsafe { self.cto_pool_arc_inner.as_ref() }.pool_pointer
	}
	
//	/// Allocate a CtoVec, which is similar to a Rust Vec but uses the persistent memory pool instead of the system allocator.
//	/// Returns on success a CtoVec.
//	#[inline(always)]
//	pub fn allocate_cto_vec<Value: CtoSafe>(&self) -> CtoVec<Value>
//	{
//		CtoVec::new(CtoPool(self.0.clone()))
//	}
//
//	/// Allocate a CtoVec with capacity, which is similar to a Rust Vec but uses the persistent memory pool instead of the system allocator.
//	/// Returns on success a CtoVec.
//	#[inline(always)]
//	pub fn allocate_cto_vec_with_capacity<Value: CtoSafe>(&self, capacity: usize) -> CtoVec<Value>
//	{
//		CtoVec::with_capacity(capacity, CtoPool(self.0.clone()))
//	}
//
//	/// Allocate a CtoReadWriteLock, which is similar to a Rust Mutex but uses the persistent memory pool instead of the system allocator.
//	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
//	/// Returns on success a CtoReadWriteLock.
//	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
//	#[inline(always)]
//	pub fn allocate_cto_read_write_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoReadWriteLock<Value>, CtoPoolAllocationError<InitializationError>>
//	{
//		self.allocate::<CtoReadWriteLock<Value>, InitializationError, Initializer>(initializer)
//	}
//
//	/// Allocate a CtoMutexLock, which is similar to a Rust Mutex but uses the persistent memory pool instead of the system allocator.
//	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
//	/// Returns on success a CtoMutexLock.
//	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
//	#[inline(always)]
//	pub fn allocate_cto_mutex_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoMutexLock<Value>, CtoPoolAllocationError<InitializationError>>
//	{
//		self.allocate::<CtoMutexLock<Value>, InitializationError, Initializer>(initializer)
//	}

	/// Allocate a CtoRc, which is similar to a Rust Rc but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoRc.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_rc<Value: CtoSafe, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoRc<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoRc<Value>, InitializationError, Initializer>(initializer)
	}
	
	/// Allocate a CtoBox, which is similar to a Rust Box but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoBox.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_box<Value: CtoSafe, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoBox<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoBox<Value>, InitializationError, Initializer>(initializer)
	}
	
	#[inline(always)]
	fn allocate<P: PersistentMemoryWrapper, InitializationError, Initializer: FnOnce(&mut P::Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<P, CtoPoolAllocationError<InitializationError>>
	{
		self.pool_pointer().allocate(initializer, self)
	}
}
