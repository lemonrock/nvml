// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
pub struct CtoPoolAllocator<'ctopool>(&'ctopool Arc<CtoPoolInner>);

impl<'ctopool> CtoPoolAllocator<'ctopool>
{
	/// The reference passed to initializer() will be ALMOST uninitialized memory; it won't even be zeroed or have default values.
	/// The exception is that `CtoSafe.reinitialize()` will have been called first.
	/// Returns on success a CtoBox, which is conceptually similar to a Box.
	/// Do not use Heap-allocated objects for fields of T, ie only use CtoSafe fields.
	#[inline(always)]
	pub fn allocate_box<T: CtoSafe, InitializationError, Initializer: FnOnce(&mut T, &Self) -> Result<(), InitializationError>>(&self, initializer: Initializer) -> Result<CtoBox<T>, CtoPoolAllocationError<InitializationError>>
	{
		self.allocate(initializer, CtoBox)
	}
	
	#[inline(always)]
	fn allocate<T: CtoSafe, InitializationError, Initializer: FnOnce(&mut T, &Self) -> Result<(), InitializationError>, Constructor: FnOnce(*mut T, Arc<CtoPoolInner>) -> Instance, Instance>(&self, initializer: Initializer, constructor: Constructor) -> Result<Instance, CtoPoolAllocationError<InitializationError>>
	{
		match self.0.deref().0.aligned_alloc::<T>()
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
}
