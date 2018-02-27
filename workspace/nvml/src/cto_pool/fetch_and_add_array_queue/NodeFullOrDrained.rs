// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


trait NodeFullOrDrained
{
	const ExclusiveMaximumIndex: Self;
	
	const InclusiveMaximumIndex: Self;
	
	#[inline(always)]
	fn is_node_full(self) -> bool;
	
	#[inline(always)]
	fn is_node_drained(self) -> bool;
}

impl NodeFullOrDrained for u32
{
	const ExclusiveMaximumIndex: Self = ExclusiveMaximumNumberOfItems as Self;
	
	const InclusiveMaximumIndex: Self = (Self::ExclusiveMaximumIndex - 1) as Self;
	
	#[inline(always)]
	fn is_node_full(self) -> bool
	{
		let next_enqueue_index = self;
		next_enqueue_index > Self::InclusiveMaximumIndex
	}
	
	#[inline(always)]
	fn is_node_drained(self) -> bool
	{
		let next_dequeue_index = self;
		next_dequeue_index > Self::InclusiveMaximumIndex
	}
}
