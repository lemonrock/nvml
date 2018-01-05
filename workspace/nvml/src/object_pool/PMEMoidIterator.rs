// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A simple iterator over PMEMoid, ie references to object instances.
#[derive(Default, Debug, Copy, Clone)]
pub struct PMEMoidIterator
{
	next: PMEMoid
}

impl Iterator for PMEMoidIterator
{
	type Item = PMEMoid;
	
	fn next(&mut self) -> Option<PMEMoid>
	{
		if self.next.is_null()
		{
			return None;
		}
		
		let result = Some(self.next);
		self.next = unsafe { pmemobj_next(self.next) };
		result
	}
}

impl PMEMoidIterator
{
	/// Create a new iterator starting at the specific PMEMoid.
	#[inline(always)]
	pub fn from(starts_at: PMEMoid) -> Self
	{
		Self
		{
			next: starts_at
		}
	}
}
