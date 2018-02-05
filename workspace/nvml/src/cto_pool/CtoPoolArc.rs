// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A guard to ensure the CTO pool is not dropped too soon.
/// Similar to a Rust Arc, but altered to address the fact that it will end-up in persistent storage and does not have weak references.
/// Also provides allocation methods.
pub struct CtoPoolArc
{
	cto_pool_arc_inner: NonNull<CtoPoolArcInner>,
}

impl Drop for CtoPoolArc
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if unsafe { self.cto_pool_arc_inner.as_mut() }.release()
		{
			drop(unsafe { Box::from_raw(self.cto_pool_arc_inner.as_ptr()) });
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
	/// Used in conjunction with `CtoSafe.cto_pool_opened()` to make sure that old references to persistent objects are discarded.
	#[inline(always)]
	pub fn write(&self, location: &mut Self)
	{
		unsafe { write(location, self.clone()) };
	}
	
	/// Pointer to CTO pool from FFI `libpmemcto`.
	#[inline(always)]
	pub fn pool_pointer(&self) -> *mut PMEMctopool
	{
		unsafe { self.cto_pool_arc_inner.as_ref() }.pool_pointer
	}
	
	/// Allocate a CtoString, which is similar to a Rust String but uses the persistent memory pool instead of the system allocator.
	/// Returns on success a CtoString.
	#[inline(always)]
	pub fn allocate_string(&self) -> CtoString
	{
		CtoString::new(self.alloc())
	}

	/// Allocate a CtoString with capacity, which is similar to a Rust String but uses the persistent memory pool instead of the system allocator.
	/// Returns on success a CtoString.
	#[inline(always)]
	pub fn allocate_string_with_capacity(&self, capacity: usize) -> CtoString
	{
		CtoString::with_capacity(capacity, self.alloc())
	}
	
	/// Allocate a CtoVec, which is similar to a Rust Vec but uses the persistent memory pool instead of the system allocator.
	/// Returns on success a CtoVec.
	#[inline(always)]
	pub fn allocate_vec<Value: CtoSafe>(&self) -> CtoVec<Value>
	{
		CtoVec::new(self.alloc())
	}

	/// Allocate a CtoVec with capacity, which is similar to a Rust Vec but uses the persistent memory pool instead of the system allocator.
	/// Returns on success a CtoVec.
	#[inline(always)]
	pub fn allocate_vec_with_capacity<Value: CtoSafe>(&self, capacity: usize) -> CtoVec<Value>
	{
		CtoVec::with_capacity(capacity, self.alloc())
	}

	/// Allocate a CtoReadWriteLock, which is similar to a Rust Mutex but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoReadWriteLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_read_write_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoReadWriteLock<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoReadWriteLock<Value>, InitializationError, Initializer>(initializer)
	}

	/// Allocate a CtoParkingLotReadWriteLock, which is a CtoSafe wrapper around a parking lot mutex which uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoParkingLotReadWriteLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_parking_lot_read_write_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoParkingLotReadWriteLock<Value>, InitializationError>
	{
		CtoParkingLotReadWriteLock::new(initializer, self)
	}

	/// Allocate a CtoParkingLotReentrantMutexLock, which is a CtoSafe wrapper around a parking lot mutex which uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoParkingLotReentrantMutexLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_parking_lot_reentrant_mutex_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoParkingLotReentrantMutexLock<Value>, InitializationError>
	{
		CtoParkingLotReentrantMutexLock::new(initializer, self)
	}

	/// Allocate a CtoParkingLotMutexLock, which is a CtoSafe wrapper around a parking lot mutex which uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoParkingLotMutexLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_parking_lot_mutex_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoParkingLotMutexLock<Value>, InitializationError>
	{
		CtoParkingLotMutexLock::new(initializer, self)
	}

	/// Allocate a CtoMutexLock, which is similar to a Rust Mutex but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoMutexLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_mutex_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoMutexLock<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoMutexLock<Value>, InitializationError, Initializer>(initializer)
	}
	
	/// Allocate a CtoRc, which is similar to a Rust Rc but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoRc.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_arc<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoArc<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoArc<Value>, InitializationError, Initializer>(initializer)
	}

	/// Allocate a CtoRc, which is similar to a Rust Rc but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoRc.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_rc<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoRc<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoRc<Value>, InitializationError, Initializer>(initializer)
	}
	
	/// Allocate a CtoBox, which is similar to a Rust Box but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoBox.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_box<Value: CtoSafe, InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoBox<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoBox<Value>, InitializationError, Initializer>(initializer)
	}
	
	#[inline(always)]
	fn allocate<P: PersistentMemoryWrapper, InitializationError, Initializer: FnOnce(*mut P::Value, &CtoPoolArc) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<P, CtoPoolAllocationError<InitializationError>>
	{
		self.pool_pointer().allocate(initializer, self)
	}
	
	#[inline(always)]
	fn aligned_allocate_or_panic(&self, alignment: usize, size: usize) -> NonNull<u8>
	{
		unsafe { NonNull::new_unchecked(self.pool_pointer().aligned_alloc(alignment, size).unwrap() as *mut u8) }
	}
	
	#[inline(always)]
	fn alloc(&self) -> CtoPoolAlloc
	{
		CtoPoolAlloc(self.clone())
	}
	
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
			cto_pool_arc_inner: Box::into_raw_non_null(cto_pool_alloc_arc).into(),
		}
	}
}
