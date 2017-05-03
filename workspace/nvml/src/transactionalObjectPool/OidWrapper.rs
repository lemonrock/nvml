// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct OidWrapper<T: Persistable>
{
	oid: PMEMoid,
	phantomData: PhantomData<T>
}

impl<T: Persistable> OID for OidWrapper<T>
{
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.oid.is_null()
	}
	
	#[inline(always)]
	fn equals(&self, right: &Self) -> bool
	{
		self.oid.equals(&right.oid)
	}
	
	#[inline(always)]
	fn persistentObjectPool(&self) -> *mut PMEMobjpool
	{
		self.oid.persistentObjectPool()
	}
	
	#[inline(always)]
	fn allocatedUsefulSize(&self) -> size_t
	{
		self.oid.allocatedUsefulSize()
	}
	
	#[inline(always)]
	fn typeNumber(&self) -> TypeNumber
	{
		self.oid.typeNumber()
	}
	
	#[inline(always)]
	fn address(&self) -> *mut c_void
	{
		self.oid.address()
	}
}

/// It is possible to violate aliasing rules
impl<T: Persistable> ::std::ops::Deref for OidWrapper<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		
		unsafe { &*self.as_ptr() }
	}
}

/// It is possible to violate aliasing rules
impl<T: Persistable> ::std::ops::DerefMut for OidWrapper<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		
		unsafe { &mut *self.as_ptr() }
	}
}

impl<T: Persistable> OidWrapper<T>
{
	#[inline(always)]
	pub fn new(oid: PMEMoid) -> Self
	{
		Self
		{
			oid: oid,
			phantomData: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn as_ptr(&self) -> *mut T
	{
		self.address() as *mut _
	}
}
