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
	#[inline(always)]
	fn fromHandle(handle: *mut PMEMobjpool) -> Self
	{
		debug_assert!(!handle.is_null(), "PMEMobjpool handle is null");
		
		ObjectPool(handle, ObjectPoolDropWrapper::new(handle))
	}
	
	#[inline(always)]
	pub fn validate(poolSetFilePath: &Path, layoutName: Option<&str>) -> Result<bool, PmdkError>
	{
		poolSetFilePath.validatePersistentMemoryObjectPoolIsConsistent(layoutName)
	}
	
	#[inline(always)]
	pub fn open(poolSetFilePath: &Path, layoutName: Option<&str>) -> Result<Self, PmdkError>
	{
		poolSetFilePath.openPersistentMemoryObjectPool(layoutName).map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn create(poolSetFilePath: &Path, layoutName: Option<&str>, poolSize: usize, mode: mode_t) -> Result<Self, PmdkError>
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
	
	/// Result may be null, use `.is_null()` to check
	#[inline(always)]
	pub fn first(&self) -> PMEMoid
	{
		unsafe { pmemobj_first(self.0) }
	}
	
	#[inline(always)]
	pub fn firstOfType<T: Persistable>(&self) -> Option<PersistentObject<T>>
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
	
	/// Returns None if there is no root object
	/// Never returns Some(0)
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
	
	/// Only affects calls to pmemobj_create()
	#[inline(always)]
	pub fn getPrefaultAtCreate() -> bool
	{
		PrefaultAtCreateKey.get_bool_global()
	}
	
	/// Only affects calls to pmemobj_create()
	#[inline(always)]
	pub fn setPrefaultAtCreate(prefaultAtCreate: bool)
	{
		PrefaultAtCreateKey.set_bool_global(prefaultAtCreate);
	}
	
	/// Only affects calls to pmemobj_open()
	#[inline(always)]
	pub fn getPrefaultAtOpen() -> bool
	{
		PrefaultAtOpenKey.get_bool_global()
	}
	
	/// Only affects calls to pmemobj_open()
	#[inline(always)]
	pub fn setPrefaultAtOpen(prefaultAtOpen: bool)
	{
		PrefaultAtOpenKey.set_bool_global(prefaultAtOpen);
	}
	
	#[inline(always)]
	pub fn getTransactionDebugSkipExpensiveChecks(&self) -> bool
	{
		TransactionDebugSkipExpensiveChecksKey.get_bool(self.0)
	}
	
	#[inline(always)]
	pub fn setTransactionDebugSkipExpensiveChecks(&self, skipExpensiveChecks: bool)
	{
		TransactionDebugSkipExpensiveChecksKey.set_bool(self.0, skipExpensiveChecks);
	}
	
	#[inline(always)]
	pub fn getTransactionCacheSizeAndThreshold(&self) -> (u64, u64)
	{
		(self.getTransactionCacheSize(), self.getTransactionCacheThreshold())
	}
	
	#[inline(always)]
	pub fn setTransactionCacheSizeAndThreshold(&self, cacheSize: u64, cacheThreshold: u64)
	{
		debug_assert!(cacheThreshold <= cacheSize, "cacheThreshold '{}' exceeds cacheSize '{}'", cacheThreshold, cacheSize);
		
		self.setTransactionCacheSize(cacheSize);
		self.setTransactionCacheThreshold(cacheThreshold);
	}
	
	#[inline(always)]
	fn getTransactionCacheSize(&self) -> u64
	{
		let transactionCacheSize = TransactionCacheSizeKey.get_integer(self.0);
		debug_assert!(transactionCacheSize < 0, "transactionCacheSize '{}' is negative", transactionCacheSize);
		
		let transactionCacheSize = transactionCacheSize as u64;
		debug_assert!(transactionCacheSize <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "transactionCacheSize '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE, '{}'", transactionCacheSize, PMEMOBJ_MAX_ALLOC_SIZE);
		
		transactionCacheSize
	}
	
	/// Maximum is 15Gb, PMEMOBJ_MAX_ALLOC_SIZE
	#[inline(always)]
	fn setTransactionCacheSize(&self, cacheSize: u64)
	{
		debug_assert!(cacheSize <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "cacheSize '{}' exceeds 0 and PMEMOBJ_MAX_ALLOC_SIZE '{}'", cacheSize, PMEMOBJ_MAX_ALLOC_SIZE);
		
		TransactionCacheSizeKey.set_integer(self.0, cacheSize as i64);
	}
	
	#[inline(always)]
	fn getTransactionCacheThreshold(&self) -> u64
	{
		let transactionCacheThreshold = TransactionCacheThresholdKey.get_integer(self.0);
		debug_assert!(transactionCacheThreshold < 0, "transactionCacheThreshold '{}' is negative", transactionCacheThreshold);
		
		let transactionCacheThreshold = transactionCacheThreshold as u64;
		debug_assert!(transactionCacheThreshold <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "transactionCacheThreshold '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE, '{}'", transactionCacheThreshold, PMEMOBJ_MAX_ALLOC_SIZE);
		
		transactionCacheThreshold
	}
	
	#[inline(always)]
	fn setTransactionCacheThreshold(&self, cacheThreshold: u64)
	{
		debug_assert!(cacheThreshold <= PMEMOBJ_MAX_ALLOC_SIZE as u64, "cacheThreshold '{}' exceeds 0 and PMEMOBJ_MAX_ALLOC_SIZE '{}'", cacheThreshold, PMEMOBJ_MAX_ALLOC_SIZE);
		
		TransactionCacheThresholdKey.set_integer(self.0, cacheThreshold as i64);
	}
}

static PrefaultAtCreateKey: &'static [u8] = b"prefault.at_create\0";

static PrefaultAtOpenKey: &'static [u8] = b"prefault.at_open\0";

static TransactionDebugSkipExpensiveChecksKey: &'static [u8] = b"tx.debug.skip_expensive_checks\0";

static TransactionCacheSizeKey: &'static [u8] = b"tx.cache.size\0";

static TransactionCacheThresholdKey: &'static [u8] = b"tx.cache.threshold\0";
