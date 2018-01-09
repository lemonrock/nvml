// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A CTO Read-Write lock read guard; the result of read-locking a CtoReadWriteLock.
/// When dropped (ie goes out of scope) the read lock is released.
#[must_use]
pub struct CtoReadWriteLockReadGuard<'read_write_lock, Value: 'read_write_lock + CtoSafe>(&'read_write_lock CtoReadWriteLockInner<Value>);

impl<'read_write_lock, Value: CtoSafe> !Send for CtoReadWriteLockReadGuard<'read_write_lock, Value>
{
}

unsafe impl<'read_write_lock, Value: CtoSafe + Sync> Sync for CtoReadWriteLockReadGuard<'read_write_lock, Value>
{
}

impl<'read_write_lock, Value: CtoSafe> Drop for CtoReadWriteLockReadGuard<'read_write_lock, Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { self.0.read_unlock(); }
	}
}

impl<'read_write_lock, Value: CtoSafe> Deref for CtoReadWriteLockReadGuard<'read_write_lock, Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'read_write_lock, Value: CtoSafe> Borrow<Value> for CtoReadWriteLockReadGuard<'read_write_lock, Value>
{
	#[inline(always)]
	fn borrow(&self) -> &Value
	{
		self.deref()
	}
}

impl<'read_write_lock, Value: CtoSafe> AsRef<Value> for CtoReadWriteLockReadGuard<'read_write_lock, Value>
{
	#[inline(always)]
	fn as_ref(&self) -> &Value
	{
		self.deref()
	}
}
