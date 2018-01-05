// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
	#[inline(always)]
	fn fromHandle(handle: *mut PMEMlogpool) -> Self
	{
		debug_assert!(!handle.is_null(), "PMEMlogpool handle is null");
		
		LogPool(handle, LogPoolDropWrapper::new(handle))
	}
	
	#[inline(always)]
	pub fn validate(poolSetFilePath: &Path) -> Result<bool, PmdkError>
	{
		poolSetFilePath.validatePersistentMemoryLogPoolIsConsistent()
	}
	
	#[inline(always)]
	pub fn open(poolSetFilePath: &Path) -> Result<Self, PmdkError>
	{
		poolSetFilePath.openPersistentMemoryLogPool().map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn create(poolSetFilePath: &Path, poolSize: usize, mode: mode_t) -> Result<Self, PmdkError>
	{
		poolSetFilePath.createPersistentMemoryLogPool(poolSize, mode).map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn amountOfUsableSpaceInTheInBytes(&self) -> usize
	{
		self.0.amountOfUsableSpaceInTheLogPoolInBytes()
	}
	
	#[inline(always)]
	pub fn appendAtomically(&mut self, buffer: *const c_void, count: usize) -> Result<(), AppendError>
	{
		self.0.appendAtomically(buffer, count)
	}
	
	#[inline(always)]
	pub fn appendVectorAtomically(&mut self, buffer: *const iovec, count: u31) -> Result<(), AppendError>
	{
		self.0.appendVectorAtomically(buffer, count)
	}
	
	#[inline(always)]
	pub fn tell(&self) -> i64
	{
		self.0.tell()
	}
	
	#[inline(always)]
	pub fn rewind(&mut self)
	{
		self.0.rewind()
	}
	
	#[inline(always)]
	pub fn walk(&self, chunkSize: usize, processChunkCallback: unsafe extern "C" fn(dataInLog: *const c_void, length: usize, callbackArgument: *mut c_void) -> WalkCallbackResult, callbackArgument: *mut c_void)
	{
		self.0.walk(chunkSize, processChunkCallback, callbackArgument)
	}
}
