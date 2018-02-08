// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Uses `#[repr(C)]` to prevent re-ordering of fields.
/// Uses align of AtomicIsolationSize.
#[repr(C, align(128))]
pub struct FreeList<T, UserState>
{
	cto_pool_arc: CtoPoolArc,
	user_state: Option<UserState>,
	pop_back_off_state: BackOffState,
	push_back_off_state: BackOffState,
	top: AtomicPointerAndCounter<FreeListElement<T>>,
	
	// MUST be last item as it is variable-length.
	elimination_array: EliminationArray<T>,
}

impl<T, UserState> Drop for FreeList<T, UserState>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		fence(Acquire);
		
		let mut free_list_element = self.top.get_pointer();
		while free_list_element.is_not_null()
		{
			let free_list_element_to_clean_up = free_list_element;
			free_list_element = unsafe { & * free_list_element }.next;
			unsafe { &mut * free_list_element_to_clean_up }.free_list_is_being_dropped_or_was_never_pushed_ever_so_free(&self.cto_pool_arc);
		}
		
		self.cto_pool_arc.pool_pointer().free(self);
	}
}

impl<T, UserState> FreeList<T, UserState>
{
	/// Call this on any other thread after `new()` before using the free list for the first time
	pub fn make_free_list_safe_to_use_on_this_thread()
	{
		fence(Acquire);
	}
	
	/// Create a new instance.
	pub fn new(cto_pool_arc: &CtoPoolArc, user_state: Option<UserState>, elimination_array_length: EliminationArrayLength) -> NonNull<Self>
	{
		let allocate_aligned_size = size_of::<Self>() + EliminationArray::<T>::variable_size_of_elimination_array_data(elimination_array_length);
		let mut this: NonNull<Self> = unsafe { NonNull::new_unchecked(cto_pool_arc.aligned_allocate_or_panic(AtomicIsolationSize, allocate_aligned_size).as_ptr() as *mut Self) };
		
		unsafe
		{
			let this = this.as_mut();
			
			write(&mut this.cto_pool_arc, cto_pool_arc.clone());
			write(&mut this.user_state, user_state);
			write(&mut this.pop_back_off_state, BackOffState::default());
			write(&mut this.push_back_off_state, BackOffState::default());
			write(&mut this.top, AtomicPointerAndCounter::default());
			this.elimination_array.initialize(elimination_array_length)
		}
		
		fence(Release);
		
		#[inline(never)]
		fn force_store()
		{
			// TODO: Needs to be aligned: AtomicIsolationSize
			let destination: AtomicU64 = AtomicU64::new(1);
			destination.swap(0, Relaxed);
		}
		force_store();
		
		this
	}
	
	/// Push a free list element.
	/// The pushed free list element must be exclusively owned by the caller, and should not be freed after push.
	pub fn push(&self, free_list_element: NonNull<FreeListElement<T>>)
	{
		fence(Acquire);
		
		// (1) Try elimination array
		if self.push_with_elimination_array(free_list_element)
		{
			return
		}
		
		// (2) Delay before (3)
		let mut index_in_cache_line = 0;
		while index_in_cache_line < MaximumNumberOfFreeListElementPointersThatFitInACacheLine
		{
			index_in_cache_line += 1;
		}
		
		// (3) Fallback to hammering on self.top
		self.push_without_elimination_array(free_list_element)
	}
	
	#[inline(always)]
	fn push_with_elimination_array(&self, free_list_element: NonNull<FreeListElement<T>>) -> bool
	{
		let mut free_list_element = free_list_element.as_ptr();
		
		let random_cache_line = self.elimination_array.random_cache_line();
		
		// Full scan of one (cache) line of elimination array.
		let mut index_in_cache_line = 0;
		while index_in_cache_line < MaximumNumberOfFreeListElementPointersThatFitInACacheLine
		{
			let entry = random_cache_line.entry(index_in_cache_line);
			if entry.is_null()
			{
				free_list_element = entry.swap(free_list_element);
				if free_list_element.is_null()
				{
					return true
				}
			}
			
			index_in_cache_line += 1;
		}
		false
	}
	
	#[inline(always)]
	fn push_without_elimination_array(&self, mut free_list_element: NonNull<FreeListElement<T>>)
	{
		let mut new_top = PointerAndCounter::from_pointer(free_list_element.as_ptr());
		
		let free_list_element = unsafe { free_list_element.as_mut() };
		let mut back_off = ExponentialBackOffState::new(&self.push_back_off_state);
		
		let mut original_top = self.top.get_pointer_and_counter();
		
		loop
		{
			free_list_element.next = original_top.get_pointer();
			fence(Release);
			
			new_top.set_counter(original_top.get_incremented_counter());
			
			if self.top.compare_and_swap_weak(&mut original_top, new_top)
			{
				back_off.auto_tune();
				return
			}
			else
			{
				back_off.exponential_back_off();
			}
		}
	}
	
	/// Pop a free list element.
	/// The popped free list element will be exclusively owned by the caller, and should not be freed after pop but recycled.
	/// It is possible for a nearly-empty or newly created queue to produce false negatives due to under population of the elimination array.
	pub fn pop(&self) -> Option<NonNull<FreeListElement<T>>>
	{
		fence(Acquire);
		
		// (1) Try elimination array
		if let Some(free_list_element) = self.pop_with_elimination_array()
		{
			return Some(free_list_element)
		}
		
		// (2) Fallback to hammering on self.top
		self.pop_without_elimination_array()
	}
	
	#[inline(always)]
	fn pop_with_elimination_array(&self) -> Option<NonNull<FreeListElement<T>>>
	{
		let random_cache_line = self.elimination_array.random_cache_line();
		
		// Full scan of one (cache) line of elimination array.
		let mut index_in_cache_line = 0;
		while index_in_cache_line < MaximumNumberOfFreeListElementPointersThatFitInACacheLine
		{
			let entry = random_cache_line.entry(index_in_cache_line);
			if entry.is_not_null()
			{
				let free_list_element = entry.swap(null_mut());
				if free_list_element.is_not_null()
				{
					return Some(unsafe { NonNull::new_unchecked(free_list_element) })
				}
			}
			
			index_in_cache_line += 1
		}
		
		None
	}
	
	#[inline(always)]
	fn pop_without_elimination_array(&self) -> Option<NonNull<FreeListElement<T>>>
	{
		let mut back_off = ExponentialBackOffState::new(&self.pop_back_off_state);
		
		let mut original_top = self.top.get_pointer_and_counter();
		loop
		{
			let original_top_pointer = original_top.get_pointer();
			
			if original_top_pointer.is_null()
			{
				return None
			}
			
			let new_top = PointerAndCounter::new(unsafe { &*original_top_pointer }.next, original_top.get_incremented_counter());
			
			if self.top.compare_and_swap_weak(&mut original_top, new_top)
			{
				back_off.auto_tune();
				return Some(unsafe { NonNull::new_unchecked(original_top_pointer) });
			}
			else
			{
				back_off.exponential_back_off();
				fence(Acquire);
			}
		}
	}
}
