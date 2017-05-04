// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct ObjectPool(*mut PMEMobjpool, Arc<ObjectPoolDropWrapper>);

unsafe impl Send for ObjectPool
{
}

unsafe impl Sync for ObjectPool
{
}

impl ObjectPool
{
	#[inline(always)]
	fn fromHandle(handle: *mut PMEMobjpool) -> Self
	{
		debug_assert!(!handle.is_null(), "PMEMobjpool handle is null");
		
		ObjectPool(handle, ObjectPoolDropWrapper::new(handle))
	}
	
	#[inline(always)]
	pub fn validate(poolSetFilePath: &Path, layoutName: Option<&str>) -> Result<bool, GenericError>
	{
		poolSetFilePath.validatePersistentMemoryObjectPoolIsConsistent(layoutName)
	}
	
	#[inline(always)]
	pub fn open(poolSetFilePath: &Path, layoutName: Option<&str>) -> Result<Self, GenericError>
	{
		poolSetFilePath.openPersistentMemoryObjectPool(layoutName).map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn create(poolSetFilePath: &Path, layoutName: Option<&str>, poolSize: usize, mode: mode_t) -> Result<Self, GenericError>
	{
		poolSetFilePath.createPersistentMemoryObjectPool(layoutName, poolSize, mode).map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn persist(&self, address: *const c_void, length: usize)
	{
		self.0.persist(address, length)
	}
	
	#[inline(always)]
	pub fn copy_nonoverlapping_then_persist(&self, address: *mut c_void, length: usize, from: *const c_void)
	{
		self.0.copy_nonoverlapping_then_persist(address, length, from)
	}
	
	#[inline(always)]
	pub fn write_bytes_then_persist(&self, address: *mut c_void, count: usize, value: u8)
	{
		self.0.write_bytes_then_persist(address, count, value)
	}
	
	#[inline(always)]
	pub fn persistOnDrop<'a>(&'a self, address: *mut c_void) -> ObjectPoolPersistOnDrop<'a>
	{
		ObjectPoolPersistOnDrop(self.0, address, PhantomData)
	}
	
	#[inline(always)]
	pub fn first(&self) -> PMEMoid
	{
		unsafe { pmemobj_first(self.0) }
	}
	
	#[inline(always)]
	pub fn firstOf<T: Persistable>(&self) -> Option<PersistentObject<T>>
	{
		let first = self.first();
		if unlikely(first.is_null())
		{
			None
		}
		else if likely(first.typeNumber() == T::TypeNumber)
		{
			Some(PersistentObject::new(first))
		}
		else
		{
			let mut previous = first;
			loop
			{
				let next = unsafe { pmemobj_next(previous) };
				if unlikely(next.is_null())
				{
					return None
				}
				else if likely(next.typeNumber() == T::TypeNumber)
				{
					return Some(PersistentObject::new(next))
				}
				previous = next;
			}
		}
	}
	
	#[inline(always)]
	pub fn rootObjectSize(&self) -> Option<size_t>
	{
		let result = unsafe { pmemobj_root_size(self.0) } as size_t;
		if unlikely(result == 0)
		{
			None
		}
		else
		{
			Some(result)
		}
	}
	
	#[inline(always)]
	pub fn allocateZeroedOrReturnExistingRootObject<T: Persistable>(&mut self) -> PersistentObject<T>
	{
		let size = T::size();
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		debug_assert!(T::TypeNumber == 0, "T is not a root object, as it has a non-zero TypeNumber of '{}'", T::TypeNumber);
		
		let resultantOid = unsafe { pmemobj_root(self.0, size) };
		assert!(!resultantOid.is_null(), "Could not re-allocate requested root object size of '{}'", size);
		PersistentObject::new(resultantOid)
	}
}
