// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO pool of persistent memory is a kind of `malloc`- or heap- like allocator.
/// Unlike the system `malloc`, multiple instances of it can be created, one for each bank of Persistent memory.
/// And, it has a graph 'root'.
/// To access the 'root' of the graph, use `deref()` or `deref_mut()`.
/// Persistence does not happen successfully until this object is closed (dropped).
/// Dropping only occurs when there are not more instances of `CtoPoolArc`.
pub struct CtoPool<RootValue: CtoSafe>(CtoPoolAlloc, PhantomData<RootValue>);

impl<RootValue: CtoSafe> PartialEq for CtoPool<RootValue>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.pool_pointer() == other.pool_pointer()
	}
}

impl<RootValue: CtoSafe> Eq for CtoPool<RootValue>
{
}

impl<RootValue: CtoSafe> Debug for CtoPool<RootValue>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		f.write_str(&format!("CtoPoolAlloc({:?})", self.0))
	}
}

unsafe impl<RootValue: CtoSafe> Send for CtoPool<RootValue>
{
}

unsafe impl<RootValue: CtoSafe> Sync for CtoPool<RootValue>
{
}

impl<RootValue: CtoSafe> Deref for CtoPool<RootValue>
{
	type Target = RootValue;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		let existing_root = self.pool_pointer().get_root();
		if unlikely(existing_root.is_null())
		{
			panic!("No root object");
		}
		else
		{
			unsafe { & * (existing_root as *const RootValue) }
		}
	}
}

impl<RootValue: CtoSafe> DerefMut for CtoPool<RootValue>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		let existing_root = self.pool_pointer().get_root();
		if unlikely(existing_root.is_null())
		{
			panic!("No root object");
		}
		else
		{
			unsafe { &mut * (existing_root as *mut RootValue) }
		}
	}
}

impl<RootValue: CtoSafe> Borrow<RootValue> for CtoPool<RootValue>
{
	#[inline(always)]
	fn borrow(&self) -> &RootValue
	{
		self.deref()
	}
}

impl<RootValue: CtoSafe> BorrowMut<RootValue> for CtoPool<RootValue>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut RootValue
	{
		self.deref_mut()
	}
}

impl<RootValue: CtoSafe> AsRef<RootValue> for CtoPool<RootValue>
{
	#[inline(always)]
	fn as_ref(&self) -> &RootValue
	{
		self.deref()
	}
}

impl<RootValue: CtoSafe> AsMut<RootValue> for CtoPool<RootValue>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut RootValue
	{
		self.deref_mut()
	}
}

impl<RootValue: CtoSafe> CtoPool<RootValue>
{
	/// Opens a pool, creating it if necessary, and re-initializing any memory that is volatile (eg condition variables, mutex locks, etc).
	/// If the pool does not contain a root, then it is initialized using `root_value_initializer`.
	#[inline(always)]
	pub fn open<InitializationError: error::Error, RootValueInitializer: FnOnce(&mut RootValue, &CtoPoolArc) -> Result<(), InitializationError>>(pool_set_file_path: &Path, layout_name: &str, pool_size: usize, mode: mode_t, root_value_initializer: RootValueInitializer) -> Result<Self, CtoPoolOpenError<InitializationError>>
	{
		let layout_name = CString::new(layout_name).expect("Embedded NULs are not allowed in a layout name");
		let length = layout_name.as_bytes().len();
		assert!(length <= PMEMCTO_MAX_LAYOUT, "layout_name length exceeds PMEMCTO_MAX_LAYOUT, {}", PMEMCTO_MAX_LAYOUT);
		
		let layout_name = layout_name.as_c_str();
		
		let pool_pointer = match pool_set_file_path.create_cto_pool(layout_name, pool_size, mode)
		{
			Err(generic_error) => return Err(CtoPoolOpenError::CreateFailed(generic_error)),
			Ok(pool_pointer) => if pool_pointer.is_null()
			{
				match pool_set_file_path.validate_cto_pool_is_consistent(layout_name)
				{
					Err(generic_error) => return Err(CtoPoolOpenError::ValidationFailed(generic_error)),
					Ok(is_valid) => if is_valid
					{
						()
					}
					else
					{
						return Err(CtoPoolOpenError::Invalid)
					},
				};
				
				match pool_set_file_path.open_cto_pool(layout_name)
				{
					Err(generic_error) => return Err(CtoPoolOpenError::OpenFailed(generic_error)),
					Ok(pool_pointer) => pool_pointer,
				}
			}
			else
			{
				pool_pointer
			},
		};
		
		let cto_pool_arc = CtoPoolArc::new(pool_pointer);
		
		let cto_pool_alloc: CtoPool<RootValue> = CtoPool(CtoPoolAlloc(cto_pool_arc), PhantomData);
		
		let existing_root = pool_pointer.get_root();
		if unlikely(existing_root.is_null())
		{
			let new_root = cto_pool_alloc.pool_pointer().aligned_allocate::<RootValue>().map_err(|pmdk_error| CtoPoolOpenError::RootCreation(CtoPoolAllocationError::Allocation(pmdk_error)))?;
			let root = unsafe { &mut * (new_root as *mut RootValue) };
			root_value_initializer(root, cto_pool_alloc.allocator()).map_err(|initialization_error| CtoPoolOpenError::RootCreation(CtoPoolAllocationError::Initialization(initialization_error)))?;
			pool_pointer.set_root(new_root);
		}
		else
		{
			let root = unsafe { &mut * (existing_root as *mut RootValue) };
			root.cto_pool_opened(cto_pool_alloc.allocator());
		}
		
		Ok(cto_pool_alloc)
	}
	
	/// Returns an object that can be used for allocations.
	#[inline(always)]
	pub fn allocator(&self) -> &CtoPoolArc
	{
		&self.alloc().0
	}
	
	/// Returns an object that can be used as a Rust `Alloc` for `RawVec`.
	#[inline(always)]
	pub fn alloc(&self) -> &CtoPoolAlloc
	{
		&self.0
	}
	
	#[inline(always)]
	fn pool_pointer(&self) -> *mut PMEMctopool
	{
		self.alloc().pool_pointer()
	}
}
