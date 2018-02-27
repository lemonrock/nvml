// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Implementation based on the paper (Hazard Pointers: Safe Memory Reclamation for Lock-Free Objects)[http://web.cecs.pdx.edu/~walpole/class/cs510/papers/11.pdf] by Maged M Michael.
#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
pub(crate) struct HazardPointerPerHyperThread<Hazardous: CtoSafe>
{
	// Cache alignment here of an 8 byte pointer to 128 bytes to try to eliminate 'false sharing'.
	hazard_pointer_per_hyper_thread: [DoubleCacheAligned<AtomicPtr<FreeListElement<Hazardous>>>; MaximumSupportedHyperThreads],
	
	// Cache alignment here to try to eliminate 'false sharing'.
	retired_lists_per_hyper_thread: [DoubleCacheAligned<UnsafeCell<Vec<NonNull<FreeListElement<Hazardous>>>>>; MaximumSupportedHyperThreads],
}

impl<Hazardous: CtoSafe> Debug for HazardPointerPerHyperThread<Hazardous>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "HazardPointerPerHyperThread<Value>")
	}
}

impl<Hazardous: CtoSafe> HazardPointerPerHyperThread<Hazardous>
{
	// This is 'R' in the paper (Hazard Pointers: Safe Memory Reclamation for Lock-Free Objects)[http://web.cecs.pdx.edu/~walpole/class/cs510/papers/11.pdf].
	// With a ReclamationThreshold of 1, this will always be true... as `retired_list_for_hyper_thread.push()` occurred above.
	const ReclamationThreshold: usize = 1;
	
	// MUST be called when queues are quiescent to clean-out any retired objects.
	// This design is not particularly safe, and will cause memory to be 'lost' in the event of a power outage.
	#[inline(always)]
	pub(crate) fn shutdown(&self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Hazardous>>)
	{
		let mut hyper_thread_index = 0;
		while hyper_thread_index < maximum_hyper_threads
		{
			for retired_object in self.retired_list_for_hyper_thread_mut(hyper_thread_index).drain(..)
			{
				free_list.push(OwnedFreeListElement::from_non_null(retired_object))
			}
			hyper_thread_index += 1;
		}
	}
	
	#[inline(always)]
	pub(crate) fn new() -> Box<Self>
	{
		Box::new
		(
			Self
			{
				hazard_pointer_per_hyper_thread: unsafe { zeroed() },
				retired_lists_per_hyper_thread: unsafe
				{
					let mut array: [DoubleCacheAligned<UnsafeCell<Vec<NonNull<FreeListElement<Hazardous>>>>>; MaximumSupportedHyperThreads] = uninitialized();
					for element in array.iter_mut()
					{
						// TODO: Eliminate Vec, move to a fixed-size array?
						// Costly: A list can grow as long as the number of hyper threads, but performance will be increased. In exchange for much, much higher memory usage (but memory usage that is fixed at allocation time, so can not run out).
						// Current estimate is 512Kb+ per queue for 256 hyper threads.
						write(element, DoubleCacheAligned::new(UnsafeCell::new(Vec::with_capacity(Self::ReclamationThreshold))))
					}
					array
				},
			}
		)
	}
	
	// Progress Condition: lock-free.
	#[inline(always)]
	pub(crate) fn protect(&self, hyper_thread_index: usize, atom: &AtomicPtr<FreeListElement<Hazardous>>) -> *mut FreeListElement<Hazardous>
	{
		let hazard_pointer_for_thread = self.hazard_pointer_for_hyper_thread(hyper_thread_index);
		
		let mut n = null_mut();
		let mut result;
		
		// Effectively loops until the value loaded is 'stable'.
		// load atom - store hazard pointer - load atom; if atom unchanged, then we're OK.
		// Does not store a hazard pointer if load atom is null.
		while
		{
			result = atom.load(SeqCst);
			result != n
		}
		{
			hazard_pointer_for_thread.store(result, SeqCst);
			n = result
		}
		
		result
	}
	
	// Progress Condition: wait-free population oblivious.
	#[inline(always)]
	pub(crate) fn clear(&self, hyper_thread_index: usize)
	{
		self.hazard_pointer_for_hyper_thread(hyper_thread_index).store(null_mut(), Release);
	}
	
	// Progress Condition: wait-free bounded (by the number of threads squared).
	#[inline(always)]
	pub(crate) fn retire(&self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Hazardous>>, hyper_thread_index: usize, retire_this_object: NonNull<FreeListElement<Hazardous>>)
	{
		let length =
		{
			let retired_list_for_hyper_thread = self.retired_list_for_hyper_thread_mut(hyper_thread_index);
			free_list.push(OwnedFreeListElement::from_non_null(retire_this_object));
			retired_list_for_hyper_thread.len()
		};
		
		if length >= Self::ReclamationThreshold
		{
			self.reclaim(maximum_hyper_threads, free_list, hyper_thread_index, length)
		}
	}
	
	#[inline(always)]
	fn reclaim(&self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Hazardous>>, hyper_thread_index: usize, original_length: usize)
	{
		// Similar to Vec.retain() but changes particularly include truncate() replaced with logic to push to a free list.
		
		let mut deletion_count = 0;
		{
			for index in 0 .. original_length
			{
				let our_retired_object = unsafe { *self.retired_list_for_hyper_thread(hyper_thread_index).get_unchecked(index) };
				let delete = self.scan_all_hyper_threads_to_see_if_they_are_still_using_a_reference_to_our_retired_object_and_if_not_delete_it(maximum_hyper_threads,our_retired_object);
				
				if delete
				{
					deletion_count += 1;
				}
				else if deletion_count > 0
				{
					self.retired_list_for_hyper_thread_mut(hyper_thread_index).swap(index - deletion_count, index)
				}
			}
		}
		
		if deletion_count > 0
		{
			let mut index = original_length - deletion_count;
			while index < original_length
			{
				free_list.push(OwnedFreeListElement::from_non_null(*unsafe { self.retired_list_for_hyper_thread(hyper_thread_index).get_unchecked(index) }));
				index += 1;
			}
			
			let new_length = original_length - deletion_count;
			let retired_list_for_hyper_thread = self.retired_list_for_hyper_thread_mut(hyper_thread_index);
			unsafe { retired_list_for_hyper_thread.set_len(new_length) }
			
			// Reclaim memory.
			if deletion_count > 4 && new_length > 0
			{
				retired_list_for_hyper_thread.shrink_to_fit();
			}
		}
	}
	
	#[inline(always)]
	fn scan_all_hyper_threads_to_see_if_they_are_still_using_a_reference_to_our_retired_object_and_if_not_delete_it(&self, maximum_hyper_threads: usize, our_retired_object: NonNull<FreeListElement<Hazardous>>) -> bool
	{
		let our_retired_object = our_retired_object.as_ptr();
		
		let mut other_current_hyper_thread_index = 0;
		while other_current_hyper_thread_index < maximum_hyper_threads
		{
			if self.hazard_pointer_for_hyper_thread(other_current_hyper_thread_index).load(SeqCst) == our_retired_object
			{
				// Another hyper thread is using a reference to `our_retired_object`, so return early and try the next our_retired_object_index
				return false
			}
			
			other_current_hyper_thread_index += 1;
		}
		true
	}
	
	#[inline(always)]
	fn hazard_pointer_for_hyper_thread(&self, hyper_thread_index: usize) -> &AtomicPtr<FreeListElement<Hazardous>>
	{
		unsafe { self.hazard_pointer_per_hyper_thread.get_unchecked(hyper_thread_index) }
	}
	
	#[inline(always)]
	fn retired_list_for_hyper_thread(&self, hyper_thread_index: usize) -> &Vec<NonNull<FreeListElement<Hazardous>>>
	{
		unsafe { &* self.retired_lists_per_hyper_thread.get_unchecked(hyper_thread_index).deref().get() }
	}
	
	#[inline(always)]
	fn retired_list_for_hyper_thread_mut(&self, hyper_thread_index: usize) -> &mut Vec<NonNull<FreeListElement<Hazardous>>>
	{
		unsafe { &mut * self.retired_lists_per_hyper_thread.get_unchecked(hyper_thread_index).deref().get() }
	}
}
