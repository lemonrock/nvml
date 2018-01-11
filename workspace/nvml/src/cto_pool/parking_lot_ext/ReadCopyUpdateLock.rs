// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Lock-free read access, but write access requires a lock and a Clone, so it is very inefficient for large objects.
#[derive(Debug)]
pub struct ReadCopyUpdateLock<Value: CtoSafe>
{
	cto_arc_cell: CtoArcCell<Value>,
	write_lock: Mutex<()>,
}

impl<Value: CtoSafe + Clone> ReadCopyUpdateLock<Value>
{
	/// Create a new ReadCopyUpdateLock<Value>.
	#[inline(always)]
	pub fn new(cto_arc: CtoArc<Value>) -> Self
	{
		Self
		{
			cto_arc_cell: CtoArcCell::new(cto_arc),
			write_lock: Mutex::new(()),
		}
	}
	
	/// Acquire a read handle to the `ReadCopyUpdateLock`.
	/// This operation never blocks.
	/// This is lock-free.
	/// This thread may spin-loop slightly if another thread is doing `read()` at the same time.
	#[inline(always)]
	pub fn read(&self) -> CtoArc<Value>
	{
		self.cto_arc_cell.get()
	}
	
	/// A version of `write()` that provides customization of the deep clone of the contents of value.
	/// The pointer (first argument) supplied to the callback is effectively the uninitialized clone.
	/// The third third argument is the original value.
	#[inline(always)]
	pub fn write_customized_deep_clone<CallbackError, DeepCloneCallback: FnOnce(*mut Value, &CtoPoolArc, &Value) -> Result<(), CallbackError>>(&self, deep_clone_callback: DeepCloneCallback) -> Result<ReadCopyUpdateLockWriteGuard<Value>, CallbackError>
	{
		let guard = self.write_lock.lock();
		let cto_arc = self.cto_arc_cell.get();
		let deep_clone_of_value = cto_arc.deep_clone_customized(deep_clone_callback)?;
		
		Ok
		(
			ReadCopyUpdateLockWriteGuard
			{
				lock: self,
				deep_clone_of_value,
				_guard: guard,
			}
		)
	}
}

impl<Value: CtoSafe + Clone> ReadCopyUpdateLock<Value>
{
	/// Acquire an exclusive write handle to the `ReadCopyUpdateLock`, protected by an `ReadCopyUpdateLockGuard`.
	/// This operation blocks if another `ReadCopyUpdateLockGuard` is currently alive, ie the `ReadCopyUpdateLock` has already handed one out to another writer.
	/// Clones the data protected by the `ReadCopyUpdateLock`, which can be expensive.
	#[inline(always)]
	pub fn write(&self) -> ReadCopyUpdateLockWriteGuard<Value>
	{
		let guard = self.write_lock.lock();
		let cto_arc = self.cto_arc_cell.get();
		
		ReadCopyUpdateLockWriteGuard
		{
			lock: self,
			deep_clone_of_value: cto_arc.deep_clone(),
			_guard: guard,
		}
	}
}
