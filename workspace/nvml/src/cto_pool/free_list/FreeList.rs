// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// To be useful, needs to be held in a CtoArc or similar structure.
/// Uses `#[repr(C)]` to prevent re-ordering of fields.
/// Uses align of AtomicIsolationSize.
/// NOTE: Can not be placed inside `CtoArc` without using double indirection, which is inefficient.
#[cfg_attr(any(target_arch = "x86", target_arch = "mips", target_arch = "sparc", target_arch = "nvptx", target_arch = "wasm32", target_arch = "hexagon"), repr(C, align(32)))]
#[cfg_attr(any(target_arch = "mips64", target_arch = "sparc64", target_arch = "s390x"), repr(C, align(64)))]
#[cfg_attr(any(target_arch = "x86_64", target_arch = "powerpc", target_arch = "powerpc64"), repr(C, align(128)))]
#[cfg_attr(any(target_arch = "arm", target_arch = "aarch64"), repr(C, align(2048)))]
pub struct FreeList<T>
{
	reference_counter: AtomicUsize,
	cto_pool_arc: CtoPoolArc,
	pop_back_off_state: BackOffState,
	push_back_off_state: BackOffState,
	top: AtomicPointerAndCounter<FreeListElement<T>>,
	
	// MUST be last item as it is variable-length.
	elimination_array: EliminationArray<T>,
}

impl<T> CtoSafe for FreeList<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		// self.reference_counter is left as-is
		cto_pool_arc.write(&mut self.cto_pool_arc);
		self.pop_back_off_state.cto_pool_opened(cto_pool_arc);
		self.push_back_off_state.cto_pool_opened(cto_pool_arc);
		self.top.cto_pool_opened(cto_pool_arc);
		self.elimination_array.cto_pool_opened(cto_pool_arc);
	}
}

impl<T> Drop for FreeList<T>
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
		
		let cto_pool_arc = self.cto_pool_arc.clone();
		cto_pool_arc.free_pointer(self);
	}
}

impl<T> CtoStrongArcInner for FreeList<T>
{
	#[inline(always)]
	fn reference_counter(&self) -> &AtomicUsize
	{
		&self.reference_counter
	}
}

impl<T> FreeList<T>
{
	/// `initial_value` is written into this `FreeListElement`: it is ***not dropped***.
	/// `trailing_additional_size_in_value_in_bytes` is used when `T` is a variably-sized type, for example, it represents a block of memory to be allocated inline in a `FreeListElement`.
	/// An `InitializedFreeListElement` can still be dropped.
	/// Call `push` on it, and it will no longer be possible to drop it.
	#[inline(always)]
	pub fn new_free_list_element<'free_list>(&'free_list self, initial_value: T, trailing_additional_size_in_value_in_bytes: usize) -> InitializedFreeListElement<'free_list, T>
	{
		InitializedFreeListElement
		{
			inner: OwnedFreeListElement::new(&self.cto_pool_arc, initial_value, trailing_additional_size_in_value_in_bytes),
			free_list: self
		}
	}
	
	/// Call this on any other thread after `new()` before using the free list for the first time
	pub fn make_free_list_safe_to_use_on_this_thread()
	{
		fence(Acquire);
	}
	
	/// Provides a strong atomically referenced counted ('Arc') wrapper around a FreeList.
	/// When the last instance of `CtoFreeListArc` is dropped, the FreeList is dropped and all FreeListElements in the list are dropped.
	/// Supply a `free_list_element_provider` if you want to make sure the elimination array is initially populated.
	/// This can return `None` if it no longer can provide free list elements.
	/// `elimination_array_length` should be equivalent to the number of threads.
	pub fn new<FreeListElementProvider: Fn(&CtoPoolArc) -> Option<InitializedFreeListElement<T>>>(cto_pool_arc: &CtoPoolArc, elimination_array_length: EliminationArrayLength, free_list_element_provider: Option<FreeListElementProvider>) -> CtoStrongArc<Self>
	{
		let allocate_aligned_size = size_of::<Self>() + EliminationArray::<T>::variable_size_of_elimination_array_data(elimination_array_length);
		
		let mut this = cto_pool_arc.aligned_allocate_or_panic_of_type::<Self>(AtomicIsolationSize, allocate_aligned_size);
		
		unsafe
		{
			let this = this.as_mut();
			
			write(&mut this.reference_counter, Self::new_reference_counter());
			write(&mut this.cto_pool_arc, cto_pool_arc.clone());
			write(&mut this.pop_back_off_state, BackOffState::default());
			write(&mut this.push_back_off_state, BackOffState::default());
			write(&mut this.top, AtomicPointerAndCounter::default());
			this.elimination_array.initialize(elimination_array_length, cto_pool_arc, free_list_element_provider)
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
		
		CtoStrongArc::new(this)
	}
	
	/// Push a free list element.
	pub fn push(&self, free_list_element: OwnedFreeListElement<T>)
	{
		debug_assert!(free_list_element.next_is_null(), "free_list_element.next should be null");
		
		let free_list_element = free_list_element.into_inner();
		
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
	/// Be careful; the popped free list element is in persistent memory, but it is rootless and can not be reset with `cto_pool_opened()`.
	pub fn pop(&self) -> Option<OwnedFreeListElement<T>>
	{
		fence(Acquire);
		
		// (1) Try elimination array
		if let Some(free_list_element) = self.pop_with_elimination_array()
		{
			debug_assert!(free_list_element.next_is_null(), "free_list_element.next should be null, because items in the elimination array should be placed in it with next null");
			return Some(free_list_element)
		}
		
		// (2) Fallback to hammering on self.top
		if let Some(mut free_list_element) = self.pop_without_elimination_array()
		{
			free_list_element.reset_next_to_null_so_cto_pool_opened_can_not_read_junk();
			return Some(free_list_element)
		}
		
		None
	}
	
	#[inline(always)]
	fn pop_with_elimination_array(&self) -> Option<OwnedFreeListElement<T>>
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
					return Some(OwnedFreeListElement::from_non_null_pointer(free_list_element))
				}
			}
			
			index_in_cache_line += 1
		}
		
		None
	}
	
	#[inline(always)]
	fn pop_without_elimination_array(&self) -> Option<OwnedFreeListElement<T>>
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
				return Some(OwnedFreeListElement::from_non_null_pointer(original_top_pointer))
			}
			else
			{
				back_off.exponential_back_off();
				fence(Acquire);
			}
		}
	}
}
