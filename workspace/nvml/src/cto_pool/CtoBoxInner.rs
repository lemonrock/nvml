// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


struct CtoBoxInner<T: CtoSafe>
{
	cto_pool_inner: Arc<CtoPoolInner>,
	value: T,
}

impl<T: CtoSafe> CtoSafe for CtoBoxInner<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.cto_pool_inner = cto_pool_inner.clone();
		self.value.reinitialize(cto_pool_inner);
	}
}

impl<T: CtoSafe> Deref for CtoBoxInner<T>
{
	type Target = T;
	
	fn deref(&self) -> &T
	{
		&self.value
	}
}

impl<T: CtoSafe> DerefMut for CtoBoxInner<T>
{
	fn deref_mut(&mut self) -> &mut T
	{
		&mut self.value
	}
}
