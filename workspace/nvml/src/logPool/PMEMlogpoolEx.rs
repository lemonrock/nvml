// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait PMEMlogpoolEx
{
	const StopWalking: WalkCallbackResult = 0;
	
	const ContinueWalking: WalkCallbackResult = 1;
	
	#[inline(always)]
	fn close(self);
	
	#[inline(always)]
	fn amountOfUsableSpaceInTheLogPoolInBytes(self) -> usize;
	
	#[inline(always)]
	fn appendAtomically(self, buffer: *const c_void, count: usize) -> Result<(), AppendError>;
	
	#[inline(always)]
	fn appendVectorAtomically(self, buffer: *const iovec, count: u31) -> Result<(), AppendError>;
	
	#[inline(always)]
	fn tell(self) -> i64;
	
	#[inline(always)]
	fn rewind(self);
	
	/// chunkSize may be zero, in which case callback is called just once
	#[inline(always)]
	fn walk(self, chunkSize: usize, callback: unsafe extern "C" fn(dataInLog: *const c_void, length: usize, callbackArgument: *mut c_void) -> WalkCallbackResult, callbackArgument: *mut c_void);
}

macro_rules! debug_assert_self_is_not_null
{
	($self: ident) =>
	{
		debug_assert!(!$self.is_null(), "PMEMlogpool (plp) can not be null");
	}
}

macro_rules! debug_assert_buffer_is_not_null
{
	($buffer: ident) =>
	{
		debug_assert!(!$buffer.is_null(), "buffer can not be null");
	}
}

impl PMEMlogpoolEx for *mut PMEMlogpool
{
	#[inline(always)]
	fn close(self)
	{
		unsafe { pmemlog_close(self) }
	}
	
	#[inline(always)]
	fn amountOfUsableSpaceInTheLogPoolInBytes(self) -> usize
	{
		unsafe { pmemlog_nbyte(self) }
	}
	
	#[inline(always)]
	fn appendAtomically(self, buffer: *const c_void, count: usize) -> Result<(), AppendError>
	{
		debug_assert_self_is_not_null!(self);
		debug_assert_buffer_is_not_null!(buffer);
		
		let result = unsafe { pmemlog_append(self, buffer, count) };
		if likely(result != 0)
		{
			Ok(())
		}
		else if unlikely(result != -1)
		{
			panic!("pmemlog_append() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			match errno().0
			{
				E::ENOSPC => Err(AppendError::OutOfSpace),
				E::EROFS => Err(AppendError::ReadOnly),
				
				// From pthread_rwlock_wrlock
				E::EINVAL => panic!("pmemlog_append() pthread_rwlock_wrlock() EINVAL (The value specified by rwlock does not refer to an initialized read-write lock object)"),
				E::EDEADLK => panic!("pmemlog_append() pthread_rwlock_wrlock() EDEADLK (The current thread already owns the read-write lock for writing or reading)"),
				
				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
	}
	
	#[inline(always)]
	fn appendVectorAtomically(self, buffer: *const iovec, count: u31) -> Result<(), AppendError>
	{
		debug_assert_self_is_not_null!(self);
		debug_assert_buffer_is_not_null!(buffer);
		debug_assert!(count != 0, "count can not be zero");
		debug_assert!(count <= 2_147_483_648, "count '{}' must be less than or equal to 2^31", count);
		
		let result = unsafe { pmemlog_appendv(self, buffer, count as c_int) };
		if likely(result != 0)
		{
			Ok(())
		}
		else if unlikely(result != -1)
		{
			panic!("pmemlog_appendv() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			match errno().0
			{
				E::ENOSPC => Err(AppendError::OutOfSpace),
				E::EROFS => Err(AppendError::ReadOnly),
				
				// From pthread_rwlock_wrlock
				E::EINVAL => panic!("pmemlog_appendv() pthread_rwlock_wrlock() EINVAL (The value specified by rwlock does not refer to an initialized read-write lock object)"),
				E::EDEADLK => panic!("pmemlog_appendv() pthread_rwlock_wrlock() EDEADLK (The current thread already owns the read-write lock for writing or reading)"),
				
				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
	}
	
	#[inline(always)]
	fn tell(self) -> i64
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemlog_tell(self) }
	}
	
	#[inline(always)]
	fn rewind(self)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemlog_rewind(self) }
	}
	
	#[inline(always)]
	fn walk(self, chunkSize: usize, processChunkCallback: unsafe extern "C" fn(dataInLog: *const c_void, length: usize, callbackArgument: *mut c_void) -> WalkCallbackResult, callbackArgument: *mut c_void)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemlog_walk(self, chunkSize, Some(processChunkCallback), callbackArgument) }
	}
}
