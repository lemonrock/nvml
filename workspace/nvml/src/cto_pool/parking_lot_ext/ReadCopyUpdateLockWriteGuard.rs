// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A write guard that automatically unlocks on drop.
/// This implementation of `deref_mut()` is not cheap; try to use it only once.
pub struct ReadCopyUpdateLockWriteGuard<'read_copy_update_lock, Value: 'read_copy_update_lock + CtoSafe>
{
	lock: &'read_copy_update_lock ReadCopyUpdateLock<Value>,
	deep_clone_of_value: CtoArc<Value>,
	_guard: MutexGuard<'read_copy_update_lock, ()>,
}

impl<'read_copy_update_lock, Value: CtoSafe> Drop for ReadCopyUpdateLockWriteGuard<'read_copy_update_lock, Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.lock.cto_arc_cell.set(self.deep_clone_of_value.clone());
	}
}

impl<'read_copy_update_lock, Value: CtoSafe> Deref for ReadCopyUpdateLockWriteGuard<'read_copy_update_lock, Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.deep_clone_of_value.deref()
	}
}

impl<'read_copy_update_lock, Value: CtoSafe> DerefMut for ReadCopyUpdateLockWriteGuard<'read_copy_update_lock, Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		CtoArc::get_mut(&mut self.deep_clone_of_value).unwrap()
	}
}
