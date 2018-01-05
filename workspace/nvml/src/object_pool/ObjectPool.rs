// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// An ObjectPool is a thread-safe owner of persistent objects (structs implementing Persistable).
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
	/// Validate an existing pool.
	#[inline(always)]
	pub fn validate(pool_set_file_path: &Path, layout_name: Option<&str>) -> Result<bool, PmdkError>
	{
		pool_set_file_path.validate_object_pool_is_consistent(layout_name)
	}
	
	/// Open an existing pool.
	/// Prefer the use of `ObjectPoolConfiguration.open_or_create()`.
	#[inline(always)]
	pub fn open(pool_set_file_path: &Path, layout_name: Option<&str>) -> Result<Self, PmdkError>
	{
		pool_set_file_path.open_object_pool(layout_name).map(Self::from_handle)
	}
	
	/// Create a new pool.
	/// Prefer the use of `ObjectPoolConfiguration.open_or_create()`.
	#[inline(always)]
	pub fn create(pool_set_file_path: &Path, layout_name: Option<&str>, pool_size: usize, mode: mode_t) -> Result<Self, PmdkError>
	{
		pool_set_file_path.create_object_pool(layout_name, pool_size, mode).map(Self::from_handle)
	}
	
	/// Persist this pool.
	#[inline(always)]
	pub fn persist(&self, address: *const c_void, length: usize)
	{
		self.0.persist(address, length)
	}
	
	/// aka 'memcpy' in C.
	#[inline(always)]
	pub fn copy_nonoverlapping_then_persist(&self, address: *mut c_void, length: usize, from: *const c_void)
	{
		self.0.copy_nonoverlapping_then_persist(address, length, from)
	}
	
	/// aka 'memset' in C.
	#[inline(always)]
	pub fn write_bytes_then_persist(&self, address: *mut c_void, count: usize, value: u8)
	{
		self.0.write_bytes_then_persist(address, count, value)
	}
	
	/// Obtain a persist on drop object to make sure persistence occurs after a number of write operations.
	#[inline(always)]
	pub fn persist_on_drop<'a>(&'a self, address: *mut c_void) -> ObjectPoolPersistOnDrop<'a>
	{
		ObjectPoolPersistOnDrop(self.0, address, PhantomData)
	}
	
	/// First persisted object in pool.
	/// Result may be null, use `.is_null()` to check
	#[inline(always)]
	pub fn first(&self) -> PMEMoid
	{
		unsafe { pmemobj_first(self.0) }
	}
	
	/// First persisted object of type in pool.
	#[inline(always)]
	pub fn first_of_type<T: Persistable>(&self) -> Option<PersistentObject<T>>
	{
		let first = self.first();
		if unlikely(first.is_null())
		{
			None
		}
		else if likely(first.type_number() == T::TypeNumber)
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
				else if likely(next.type_number() == T::TypeNumber)
				{
					return Some(PersistentObject::new(next))
				}
				previous = next;
			}
		}
	}
	
	/// Size in bytes of root object.
	/// Returns None if there is no root object.
	/// Never returns Some(0).
	#[inline(always)]
	pub fn root_object_size(&self) -> Option<size_t>
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
	
	/// Find if a potential performance improvement is enabled.
	/// Only affects calls to pmemobj_create().
	#[inline(always)]
	pub fn get_prefault_at_create() -> bool
	{
		PrefaultAtCreateKey.get_bool_global()
	}
	
	/// Enable a potential performance improvement.
	/// Only affects calls to pmemobj_create().
	#[inline(always)]
	pub fn set_prefault_at_create(enable: bool)
	{
		PrefaultAtCreateKey.set_bool_global(enable);
	}
	
	/// Find if a potential performance improvement is enabled.
	/// Only affects calls to pmemobj_open().
	#[inline(always)]
	pub fn get_prefault_at_open() -> bool
	{
		PrefaultAtOpenKey.get_bool_global()
	}
	
	/// Enable a potential performance improvement.
	/// Only affects calls to pmemobj_open().
	#[inline(always)]
	pub fn set_prefault_at_open(enable: bool)
	{
		PrefaultAtOpenKey.set_bool_global(enable);
	}
	
	/// Find if a potential performance improvement is enabled.
	/// Get whether transaction debug skip expensive checks are enabled.
	#[inline(always)]
	pub fn get_transaction_debug_skip_expensive_checks(&self) -> bool
	{
		TransactionDebugSkipExpensiveChecksKey.get_bool(self.0)
	}
	
	/// Enable a potential performance improvement.
	/// Set whether transaction debug skip expensive checks are enabled.
	#[inline(always)]
	pub fn set_transaction_debug_skip_expensive_checks(&self, enable: bool)
	{
		TransactionDebugSkipExpensiveChecksKey.set_bool(self.0, enable);
	}
	
	/// Get transaction cache size and threshold in bytes.
	/// Maximum transaction cache size is 15Gb, `nvml_sys::PMEMOBJ_MAX_ALLOC_SIZE`.
	#[inline(always)]
	pub fn get_transaction_cache_size_and_threshold(&self) -> (u64, u64)
	{
		(self.get_transaction_cache_size(), self.get_transaction_cache_threshold())
	}
	
	/// Set transaction cache size and threshold in bytes.
	/// Maximum transaction cache size is 15Gb,`nvml_sys::PMEMOBJ_MAX_ALLOC_SIZE`.
	#[inline(always)]
	pub fn set_transaction_cache_size_and_threshold(&self, cache_size: u64, cache_threshold: u64)
	{
		debug_assert!(cache_threshold <= cache_size, "cache_threshold '{}' exceeds cache_size '{}'", cache_threshold, cache_size);
		
		self.set_transaction_cache_size(cache_size);
		self.set_transaction_cache_threshold(cache_threshold);
	}
	
	#[inline(always)]
	fn get_transaction_cache_size(&self) -> u64
	{
		let transaction_cache_size = TransactionCacheSizeKey.get_integer(self.0);
		debug_assert!(transaction_cache_size < 0, "transaction_cache_size '{}' is negative", transaction_cache_size);
		
		let transaction_cache_size = transaction_cache_size as u64;
		debug_assert!(transaction_cache_size <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "transaction_cache_size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE, '{}'", transaction_cache_size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		transaction_cache_size
	}
	
	#[inline(always)]
	fn set_transaction_cache_size(&self, cache_size: u64)
	{
		debug_assert!(cache_size <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "cache_size '{}' exceeds 0 and PMEMOBJ_MAX_ALLOC_SIZE '{}'", cache_size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		TransactionCacheSizeKey.set_integer(self.0, cache_size as i64);
	}
	
	#[inline(always)]
	fn get_transaction_cache_threshold(&self) -> u64
	{
		let transaction_cache_threshold = TransactionCacheThresholdKey.get_integer(self.0);
		debug_assert!(transaction_cache_threshold < 0, "transaction_cache_threshold '{}' is negative", transaction_cache_threshold);
		
		let transaction_cache_threshold = transaction_cache_threshold as u64;
		debug_assert!(transaction_cache_threshold <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "transaction_cache_threshold '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE, '{}'", transaction_cache_threshold, PMEMOBJ_MAX_ALLOC_SIZE);
		
		transaction_cache_threshold
	}
	
	#[inline(always)]
	fn set_transaction_cache_threshold(&self, cache_threshold: u64)
	{
		debug_assert!(cache_threshold <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "cache_threshold '{}' exceeds 0 and PMEMOBJ_MAX_ALLOC_SIZE '{}'", cache_threshold, PMEMOBJ_MAX_ALLOC_SIZE);
		
		TransactionCacheThresholdKey.set_integer(self.0, cache_threshold as i64);
	}
	
	#[inline(always)]
	fn from_handle(handle: *mut PMEMobjpool) -> Self
	{
		debug_assert!(!handle.is_null(), "PMEMobjpool handle is null");
		
		ObjectPool(handle, ObjectPoolDropWrapper::new(handle))
	}
}

static PrefaultAtCreateKey: &'static [u8] = b"prefault.at_create\0";

static PrefaultAtOpenKey: &'static [u8] = b"prefault.at_open\0";

static TransactionDebugSkipExpensiveChecksKey: &'static [u8] = b"tx.debug.skip_expensive_checks\0";

static TransactionCacheSizeKey: &'static [u8] = b"tx.cache.size\0";

static TransactionCacheThresholdKey: &'static [u8] = b"tx.cache.threshold\0";
