// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A log pool of persistent memory.
#[derive(Debug, Clone)]
pub struct LogPool(*mut PMEMlogpool, Arc<LogPoolDropWrapper>);

unsafe impl Send for LogPool
{
}

unsafe impl Sync for LogPool
{
}

impl LogPool
{
	/// Validate a log pool is consistent.
	#[inline(always)]
	pub fn validate(pool_set_file_path: &Path) -> Result<bool, PmdkError>
	{
		pool_set_file_path.validate_log_pool_is_consistent()
	}
	
	/// Open an existing log pool.
	#[inline(always)]
	pub fn open(pool_set_file_path: &Path) -> Result<Self, PmdkError>
	{
		pool_set_file_path.open_log_pool().map(Self::from_handle)
	}
	
	/// Create (and implicitly open) a new log pool.
	#[inline(always)]
	pub fn create(pool_set_file_path: &Path, pool_size: usize, mode: mode_t) -> Result<Self, PmdkError>
	{
		pool_set_file_path.create_log_pool(pool_size, mode).map(Self::from_handle)
	}
	
	/// How many bytes are free in the log pool?
	#[inline(always)]
	pub fn amount_of_usable_space_in_the_log_pool_in_bytes(&self) -> usize
	{
		self.0.amount_of_usable_space_in_the_log_pool_in_bytes()
	}
	
	/// Atomically append to the log (roughly equivalent to `write`).
	#[inline(always)]
	pub fn append_atomically(&mut self, buffer: *const c_void, count: usize) -> Result<(), AppendError>
	{
		self.0.append_atomically(buffer, count)
	}
	
	/// Atomically append to the log using an `iovec` (roughly equivalent to `writev`).
	#[inline(always)]
	pub fn append_vector_atomically(&mut self, buffer: *const iovec, count: u31) -> Result<(), AppendError>
	{
		self.0.append_vector_atomically(buffer, count)
	}
	
	/// Tell the log (?)
	#[inline(always)]
	pub fn tell(&self) -> i64
	{
		self.0.tell()
	}
	
	/// Rewind the log.
	#[inline(always)]
	pub fn rewind(&mut self)
	{
		self.0.rewind()
	}
	
	/// Walk ('read') the log.
	/// chunk_size may be zero, in which case `for_each_chunk_callback` is called just once.
	#[inline(always)]
	pub fn walk(&self, chunk_size: usize, for_each_chunk_callback: ForEachChunkCallback, callback_argument: *mut c_void)
	{
		self.0.walk(chunk_size, for_each_chunk_callback, callback_argument)
	}
	
	#[inline(always)]
	fn from_handle(handle: *mut PMEMlogpool) -> Self
	{
		debug_assert!(handle.is_not_null(), "PMEMlogpool handle is null");
		
		LogPool(handle, LogPoolDropWrapper::new(handle))
	}
}
