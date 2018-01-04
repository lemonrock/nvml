// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
pub struct CtoPool<T: CtoSafe + Send + Sync>(Arc<CtoPoolInner>, RwLock<CtoRootBox<T>>);

unsafe impl<T: CtoSafe + Send + Sync> Send for CtoPool<T>
{
}

unsafe impl<T: CtoSafe + Send + Sync> Sync for CtoPool<T>
{
}

impl<T: CtoSafe + Send + Sync> CtoPool<T>
{
	/// Validate this CTO pool.
	#[inline(always)]
	pub fn validate(pool_set_file_path: &Path, layout_name: &CStr) -> Result<bool, GenericError>
	{
		pool_set_file_path.validatePersistentMemoryCtoPoolIsConsistent(layout_name)
	}
	
	/// This method is unsafe, because nothing stops T being of a different layout (struct type).
	/// Additionally, nothing stops the layout of T changing from compile to compile.
	#[inline(always)]
	pub fn open<InitializationError: error::Error, Initializer: FnOnce(&mut T, &CtoPoolAllocator) -> Result<(), InitializationError>>(pool_set_file_path: &Path, layout_name: &CStr, root_initializer: Initializer) -> Result<Self, CtoPoolOpenError<InitializationError>>
	{
		let cto_pool_inner = match CtoPoolInner::open(pool_set_file_path, layout_name)
		{
			Err(generic_error) => return Err(CtoPoolOpenError::Open(generic_error)),
			Ok(cto_pool_inner) => cto_pool_inner,
		};
		
		let existing_root = cto_pool_inner.get_root();
		let root = if unlikely(existing_root.is_null())
		{
			let new_root_cto_box = CtoPoolAllocator(&cto_pool_inner).allocate_box(root_initializer).map_err(|cto_pool_allocation_error| CtoPoolOpenError::RootCreation(cto_pool_allocation_error))?;
			let new_root = CtoBox::into_raw(new_root_cto_box);
			cto_pool_inner.set_root(new_root);
			new_root
		}
		else
		{
			let mutable_root_reference = unsafe { &mut * (existing_root as *mut T) };
			mutable_root_reference.reinitialize(&cto_pool_inner);
			existing_root
		};
		
		Ok(CtoPool(cto_pool_inner, RwLock::new(CtoRootBox(root))))
	}
	
	/// Returns an allocator which can be used to create new CtoBox and other 'heap-like' persistent memory objects.
	#[inline(always)]
	pub fn allocator<'ctopool>(&'ctopool self) -> CtoPoolAllocator<'ctopool>
	{
		CtoPoolAllocator(self.cto_pool_inner())
	}
	
	/// Returns a Read-Write lock to access the root of the CTO object graph.
	#[inline(always)]
	pub fn root(&self) -> &RwLock<CtoRootBox<T>>
	{
		&self.1
	}
	
	#[inline(always)]
	fn cto_pool_inner(&self) -> &Arc<CtoPoolInner>
	{
		&self.0
	}
}

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
