// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct EliminationArrayEntry<T>(AtomicPtr<FreeListElement<T>>);

impl<T> CtoSafe for EliminationArrayEntry<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let value = self.value();
		if value.is_not_null()
		{
			unsafe { &mut * value }.cto_pool_opened(cto_pool_arc)
		}
	}
}

impl<T> EliminationArrayEntry<T>
{
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.value().is_null()
	}
	
	#[inline(always)]
	fn is_not_null(&self) -> bool
	{
		self.value().is_null()
	}
	
	#[inline(always)]
	fn swap(&self, new_free_list_element: *mut FreeListElement<T>) -> *mut FreeListElement<T>
	{
		self.0.swap(new_free_list_element, Relaxed)
	}
	
	#[inline(always)]
	fn value(&self) -> *mut FreeListElement<T>
	{
		self.0.load(Relaxed)
	}
	
	#[inline(always)]
	fn set_initial_value_to_null_or<FreeListElementProvider: Fn(&CtoPoolArc) -> Option<InitializedFreeListElement<T>>>(&self, cto_pool_arc: &CtoPoolArc, free_list_element_provider: Option<&FreeListElementProvider>)
	{
		let free_list_element_pointer = if let Some(free_list_element_provider) = free_list_element_provider
		{
			if let Some(initialized_free_list_element) = free_list_element_provider(cto_pool_arc)
			{
				initialized_free_list_element.into_inner().into_inner().as_ptr()
			}
			else
			{
				null_mut()
			}
		}
		else
		{
			null_mut()
		};
		
		self.0.store(free_list_element_pointer, Relaxed)
	}
}
