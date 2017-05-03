// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct BlockPool(*mut PMEMblkpool, Arc<BlockPoolDropWrapper>);

unsafe impl Send for BlockPool
{
}

unsafe impl Sync for BlockPool
{
}

impl BlockPool
{
	#[inline(always)]
	fn fromHandle(handle: *mut PMEMblkpool) -> Self
	{
		debug_assert!(!handle.is_null(), "PMEMblkpool handle is null");
		
		BlockPool(handle, BlockPoolDropWrapper::new(handle))
	}
	
	#[inline(always)]
	pub fn validate(poolSetFilePath: &Path, blockSize: usize) -> Result<bool, GenericError>
	{
		poolSetFilePath.validatePersistentMemoryBlockPoolIsConsistent(blockSize)
	}
	
	#[inline(always)]
	pub fn open(poolSetFilePath: &Path) -> Result<Self, GenericError>
	{
		const DoNotValidateBlockSize: usize = 0;
		
		poolSetFilePath.openPersistentMemoryBlockPool(DoNotValidateBlockSize).map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn create(poolSetFilePath: &Path, blockSize: usize, poolSize: usize, mode: mode_t) -> Result<Self, GenericError>
	{
		poolSetFilePath.createPersistentMemoryBlockPool(blockSize, poolSize, mode).map(Self::fromHandle)
	}
	
	#[inline(always)]
	pub fn blockSize(self) -> usize
	{
		self.0.blockSize()
	}
	
	#[inline(always)]
	pub fn numberOfBlocksAvailableInBlockPool(self) -> usize
	{
		self.0.numberOfBlocksAvailableInBlockPool()
	}
	
	#[inline(always)]
	pub fn read(self, to: *mut c_void, zeroBasedBlockIndex: usize) -> bool
	{
		self.0.read(to, zeroBasedBlockIndex)
	}
	
	#[inline(always)]
	pub fn write(self, from: *const c_void, zeroBasedBlockIndex: usize)
	{
		self.0.write(from, zeroBasedBlockIndex)
	}
	
	#[inline(always)]
	pub fn setZero(self, zeroBasedBlockIndex: usize)
	{
		self.0.setZero(zeroBasedBlockIndex)
	}
	
	#[inline(always)]
	pub fn setError(self, zeroBasedBlockIndex: usize)
	{
		self.0.setError(zeroBasedBlockIndex)
	}
}
