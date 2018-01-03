// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct CtoPoolDropWrapper(*mut PMEMctopool);

impl Drop for CtoPoolDropWrapper
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl CtoPoolDropWrapper
{
	#[inline(always)]
	fn new(handle: *mut PMEMctopool) -> Arc<Self>
	{
		Arc::new(CtoPoolDropWrapper(handle))
	}
}
