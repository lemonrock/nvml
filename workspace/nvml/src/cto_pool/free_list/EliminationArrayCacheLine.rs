// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct EliminationArrayCacheLine<T>
{
	cache_line: [EliminationArrayEntry<T>; MaximumNumberOfFreeListElementPointersThatFitInACacheLine]
}

impl<T> CtoSafe for EliminationArrayCacheLine<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let mut entry_index = 0;
		while entry_index < MaximumNumberOfFreeListElementPointersThatFitInACacheLine
		{
			self.entry_mut(entry_index).cto_pool_opened(cto_pool_arc);
			
			entry_index += 1;
		}
	}
}

impl<T> EliminationArrayCacheLine<T>
{
	#[inline(always)]
	fn initialize<FreeListElementProvider: Fn(&CtoPoolArc) -> Option<InitializedFreeListElement<T>>>(&self, cto_pool_arc: &CtoPoolArc, free_list_element_provider: Option<&FreeListElementProvider>)
	{
		let mut entry_index = 0;
		while entry_index < MaximumNumberOfFreeListElementPointersThatFitInACacheLine
		{
			self.entry(entry_index).set_initial_value_to_null_or(cto_pool_arc, free_list_element_provider);
			entry_index += 1;
		}
	}
	
	#[inline(always)]
	fn entry(&self, entry_index: usize) -> &EliminationArrayEntry<T>
	{
		debug_assert!(entry_index < MaximumNumberOfFreeListElementPointersThatFitInACacheLine, "entry_index is not less than MaximumNumberOfFreeListElementPointersThatFitInACacheLine '{}'", MaximumNumberOfFreeListElementPointersThatFitInACacheLine);
		
		unsafe { self.cache_line.get_unchecked(entry_index) }
	}
	
	#[inline(always)]
	fn entry_mut(&mut self, entry_index: usize) -> &mut EliminationArrayEntry<T>
	{
		debug_assert!(entry_index < MaximumNumberOfFreeListElementPointersThatFitInACacheLine, "entry_index is not less than MaximumNumberOfFreeListElementPointersThatFitInACacheLine '{}'", MaximumNumberOfFreeListElementPointersThatFitInACacheLine);
		
		unsafe { self.cache_line.get_unchecked_mut(entry_index) }
	}
}
