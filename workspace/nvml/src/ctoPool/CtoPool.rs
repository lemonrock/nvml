// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct CtoPool(*mut PMEMctopool, Arc<CtoPoolDropWrapper>);

unsafe impl Send for CtoPool
{
}

unsafe impl Sync for CtoPool
{
}

impl PartialEq for CtoPool
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
}

impl Eq for CtoPool
{
}

impl CtoPool
{
	/// The reference passed to initializer() will be uninitialized memory; it won't even be zeroed or have default values.
	/// Returns on success a CtoBox, which is conceptually similar to a Box.
	/// Returns on error either an initialisation failure (Left) or an allocation failure (Right).
	#[inline(always)]
	pub fn allocate_box<T: CtoSafe, Failure, Initializer: FnOnce(&mut T, &Self) -> Result<(), Failure>>(&self, initializer: Initializer) -> Result<CtoBox<T>, Either<Failure, GenericError>>
	{
		let pointer = match self.0.aligned_alloc::<T>()
		{
			Err(allocation_error) => return Err(Right(allocation_error)),
			Ok(pointer) => pointer,
		};
		
		match initializer(unsafe { &mut *pointer }, self)
		{
			Err(failure) => Err(Left(failure)),
			Ok(()) => Ok(CtoBox(pointer, self.clone())),
		}
	}
	
	#[inline(always)]
	pub fn validate(pool_set_file_path: &Path, layout_name: &CStr) -> Result<bool, GenericError>
	{
		pool_set_file_path.validatePersistentMemoryCtoPoolIsConsistent(layout_name)
	}
	
	#[inline(always)]
	pub fn open(pool_set_file_path: &Path, layout_name: &CStr) -> Result<Self, GenericError>
	{
		pool_set_file_path.openPersistentMemoryCtoPool(layout_name).map(Self::from_handle)
	}
	
	#[inline(always)]
	pub fn create(pool_set_file_path: &Path, layout_name: &CStr, pool_size: usize, mode: mode_t) -> Result<Self, GenericError>
	{
		pool_set_file_path.createPersistentMemoryCtoPool(layout_name, pool_size, mode).map(Self::from_handle)
	}
	
	#[inline(always)]
	fn from_handle(handle: *mut PMEMctopool) -> Self
	{
		debug_assert!(!handle.is_null(), "PMEMctopool handle is null");
		
		CtoPool(handle, CtoPoolDropWrapper::new(handle))
	}
}
