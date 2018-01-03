// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PMEMblkpoolEx
{
	#[inline(always)]
	fn close(self);
	
	#[inline(always)]
	fn blockSize(self) -> usize;
	
	#[inline(always)]
	fn numberOfBlocksAvailableInBlockPool(self) -> usize;
	
	/// Returns false if the block has previously had its error condition set
	#[inline(always)]
	fn read_from(self, to: *mut c_void, zeroBasedBlockIndex: usize) -> bool;
	
	#[inline(always)]
	fn write_to(self, from: *const c_void, zeroBasedBlockIndex: usize);
	
	#[inline(always)]
	fn setZero(self, zeroBasedBlockIndex: usize);
	
	#[inline(always)]
	fn setError(self, zeroBasedBlockIndex: usize);
}

macro_rules! debug_assert_self_is_not_null
{
	($self: ident) =>
	{
		debug_assert!(!$self.is_null(), "PMEMblkpool (pbp) can not be null");
	}
}

impl PMEMblkpoolEx for *mut PMEMblkpool
{
	#[inline(always)]
	fn close(self)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemblk_close(self) }
	}
	
	#[inline(always)]
	fn blockSize(self) -> usize
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemblk_bsize(self) }
	}
	
	#[inline(always)]
	fn numberOfBlocksAvailableInBlockPool(self) -> usize
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemblk_nblock(self) }
	}
	
	#[inline(always)]
	fn read_from(self, to: *mut c_void, zeroBasedBlockIndex: usize) -> bool
	{
		debug_assert_self_is_not_null!(self);
		debug_assert!(!to.is_null(), "to can not be null");
		debug_assert!(zeroBasedBlockIndex < self.numberOfBlocksAvailableInBlockPool(), "zeroBasedBlockIndex '{}' exceeds numberOfBlocksAvailableInBlockPool '{}'", zeroBasedBlockIndex, self.numberOfBlocksAvailableInBlockPool());
		
		let result = unsafe { pmemblk_read(self, to, zeroBasedBlockIndex as c_longlong)};
		if likely(result == 0)
		{
			return true;
		}
		else if unlikely(result != -1)
		{
			panic!("pmemblk_read() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			let osErrorNumber = errno().0;
			if likely(osErrorNumber == E::EIO)
			{
				false
			}
			else
			{
				GenericError::new(osErrorNumber, pmemblk_errormsg, "pmemblk_read").panic();
			}
		}
	}
	
	#[inline(always)]
	fn write_to(self, from: *const c_void, zeroBasedBlockIndex: usize)
	{
		debug_assert_self_is_not_null!(self);
		debug_assert!(!from.is_null(), "from can not be null");
		debug_assert!(zeroBasedBlockIndex < self.numberOfBlocksAvailableInBlockPool(), "zeroBasedBlockIndex '{}' exceeds numberOfBlocksAvailableInBlockPool '{}'", zeroBasedBlockIndex, self.numberOfBlocksAvailableInBlockPool());
		
		let result = unsafe { pmemblk_write(self, from, zeroBasedBlockIndex as c_longlong)};
		if likely(result == 0)
		{
			return;
		}
		else if unlikely(result != -1)
		{
			panic!("pmemblk_write() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			GenericError::new(errno().0, pmemblk_errormsg, "pmemblk_write").panic();
		}
	}
	
	#[inline(always)]
	fn setZero(self, zeroBasedBlockIndex: usize)
	{
		debug_assert_self_is_not_null!(self);
		debug_assert!(zeroBasedBlockIndex < self.numberOfBlocksAvailableInBlockPool(), "zeroBasedBlockIndex '{}' exceeds numberOfBlocksAvailableInBlockPool '{}'", zeroBasedBlockIndex, self.numberOfBlocksAvailableInBlockPool());
		
		let result = unsafe { pmemblk_set_zero(self, zeroBasedBlockIndex as c_longlong)};
		if likely(result == 0)
		{
			return;
		}
		else if unlikely(result != -1)
		{
			panic!("pmemblk_set_zero() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			GenericError::new(errno().0, pmemblk_errormsg, "pmemblk_set_zero").panic();
		}
	}
	
	#[inline(always)]
	fn setError(self, zeroBasedBlockIndex: usize)
	{
		debug_assert_self_is_not_null!(self);
		debug_assert!(zeroBasedBlockIndex < self.numberOfBlocksAvailableInBlockPool(), "zeroBasedBlockIndex '{}' exceeds numberOfBlocksAvailableInBlockPool '{}'", zeroBasedBlockIndex, self.numberOfBlocksAvailableInBlockPool());
		
		let result = unsafe { pmemblk_set_error(self, zeroBasedBlockIndex as c_longlong)};
		if likely(result == 0)
		{
			return;
		}
		else if unlikely(result != -1)
		{
			panic!("pmemblk_set_error() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			GenericError::new(errno().0, pmemblk_errormsg, "pmemblk_set_error").panic();
		}
	}
}
