// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Unfortunately, currently public due to CtoSafe being public.
#[derive(Debug)]
pub struct CtoPoolInner(*mut PMEMctopool);

impl Drop for CtoPoolInner
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close();
	}
}

impl PartialEq for CtoPoolInner
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
}

impl Eq for CtoPoolInner
{
}

impl CtoPoolInner
{
	/// Free a previously allocated and initialized object. Calls `drop_in_place()` if T needs to be dropped.
	#[inline(always)]
	pub fn free_persistent_memory<PersistentMemory>(this: &Arc<CtoPoolInner>, persistent_memory_pointer: *mut PersistentMemory)
	{
		if needs_drop::<PersistentMemory>()
		{
			unsafe { drop_in_place(persistent_memory_pointer) }
		}
		
		let cto_pool_inner = this.deref();
		cto_pool_inner.0.free(persistent_memory_pointer)
	}
	
	#[inline(always)]
	fn get_root<T: CtoSafe>(&self) -> *mut T
	{
		self.0.get_root()
	}
	
	#[inline(always)]
	fn set_root<T: CtoSafe>(&self, root: *mut T)
	{
		self.0.set_root(root)
	}
}
