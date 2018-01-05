// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A wrapper struct providing access to the chosen pool's allocator
#[derive(Debug)]
pub struct CtoPoolAllocator<'ctopool>(&'ctopool Arc<CtoPoolInner>);

impl<'ctopool> CtoPoolAllocator<'ctopool>
{
	/// Allocate a CtoRc, which is similar to a Rust Rc but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// The exception is that `CtoSafe.reinitialize()` will have been called first.
	/// Returns on success a CtoBox, which is conceptually similar to a Box.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_rc<T: CtoSafe, InitializationError, Initializer: FnOnce(&mut T, &Self) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoRc<T>, CtoPoolAllocationError<InitializationError>>
	{
		let cto_box: CtoBox<CtoRcInner<T>> = self.allocate_box(|cto_rc_inner: &mut CtoRcInner<T>, cto_pool_allocator|
		{
			cto_rc_inner.strong_counter = CtoRcCounter::default();
			cto_rc_inner.weak_counter = CtoRcCounter::default();
			cto_rc_inner.cto_pool_inner = cto_pool_allocator.0.clone();
			initializer(&mut cto_rc_inner.value, cto_pool_allocator)
		})?;
		let pointer = CtoBox::into_pointer(cto_box);
		Ok(CtoRc(pointer))
	}
	
	/// Allocate a CtoRootBox, which is similar to a Rust Box but uses the persistent memory pool instead of the system allocator.
	#[inline(always)]
	pub(crate) fn allocate_root_box<T: CtoSafe + Send + Sync, InitializationError, Initializer: FnOnce(&mut T, &Self) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoRootBox<T>, CtoPoolAllocationError<InitializationError>>
	{
		let cto_box = self.allocate_box(initializer)?;
		Ok(CtoRootBox(CtoBox::into_pointer(cto_box)))
	}
	
	/// Allocate a CtoBox, which is similar to a Rust Box but uses the persistent memory pool instead of the system allocator.
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// The exception is that `CtoSafe.reinitialize()` will have been called first.
	/// Returns on success a CtoBox, which is conceptually similar to a Box.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_box<T: CtoSafe, InitializationError, Initializer: FnOnce(&mut T, &Self) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoBox<T>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate(initializer, CtoBox::constructor)
	}
	
	#[inline(always)]
	fn allocate<T: CtoSafe, InitializationError, Initializer: FnOnce(&mut T, &Self) -> Result<(), InitializationError>, Constructor: FnOnce(*mut T, Arc<CtoPoolInner>) -> Instance, Instance>(&self, initializer: Initializer, constructor: Constructor) -> Result<Instance, CtoPoolAllocationError<InitializationError>>
	{
		match self.aligned_allocate()
		{
			Err(allocation_error) => return Err(CtoPoolAllocationError::Allocation(allocation_error)),
			
			Ok(pointer) =>
			{
				let mutable_reference = unsafe { &mut *pointer };
				
				match initializer(mutable_reference, self)
				{
					Ok(()) => Ok(constructor(pointer, self.0.clone())),
					Err(initialization_error) =>
					{
						(self.0).0.free(pointer);
						
						Err(CtoPoolAllocationError::Initialization(initialization_error))
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn aligned_allocate<T: CtoSafe>(&self) -> Result<*mut T, PmdkError>
	{
		self.0.deref().0.aligned_alloc::<T>()
	}
}
