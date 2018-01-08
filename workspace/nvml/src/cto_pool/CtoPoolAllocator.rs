// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A wrapper struct providing access to the chosen pool's allocator
#[derive(Debug)]
pub struct CtoPoolAllocator<'ctopool>(&'ctopool Arc<CtoPoolInner>);

impl<'ctopool> CtoPoolAllocator<'ctopool>
{
	/// Allocate a CtoReadWriteLock, which is similar to a Rust Mutex but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoReadWriteLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_cto_read_write_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoReadWriteLock<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoReadWriteLock<Value>, InitializationError, Initializer>(initializer)
	}
	
	/// Allocate a CtoMutexLock, which is similar to a Rust Mutex but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoMutexLock.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_cto_mutex_lock<Value: CtoSafe, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoMutexLock<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoMutexLock<Value>, InitializationError, Initializer>(initializer)
	}
	
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
	
	/// Allocate a CtoRootBox, which is similar to a Rust Box but uses the persistent memory pool instead of the system allocator.
	#[inline(always)]
	pub(crate) fn allocate_root_box<Value: CtoSafe + Sync, InitializationError, Initializer: FnOnce(&mut Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoRootBox<Value>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate::<CtoRootBox<Value>, InitializationError, Initializer>(initializer)
	}
	
	#[inline(always)]
	fn allocate<P: PersistentMemoryWrapper, InitializationError, Initializer: FnOnce(&mut P::Value) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<P, CtoPoolAllocationError<InitializationError>>
	{
		match self.aligned_allocate::<P>()
		{
			Err(allocation_error) => return Err(CtoPoolAllocationError::Allocation(allocation_error)),
			
			Ok(persistent_memory_pointer) => match P::initialize_persistent_memory(persistent_memory_pointer, self.0, initializer)
			{
				Ok(outer) => Ok(outer),
				
				Err(initialization_error) =>
				{
					(self.0).0.free(persistent_memory_pointer);
					
					Err(CtoPoolAllocationError::Initialization(initialization_error))
				}
			},
		}
	}
	
	#[inline(always)]
	fn aligned_allocate<P: PersistentMemoryWrapper>(&self) -> Result<*mut P::PersistentMemory, PmdkError>
	{
		let alignment = align_of::<P::PersistentMemory>();
		let size = size_of::<P::PersistentMemory>() as size_t;
		self.as_ptr().aligned_alloc(alignment, size).map(|pointer| pointer as *mut P::PersistentMemory)
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut PMEMctopool
	{
		self.0.deref().0
	}
}
