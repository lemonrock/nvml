// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use ExtendedNonNull;
use ToNonNull;
use super::*;
use super::arc::CtoStrongArc;
use super::arc::CtoStrongArcInner;
use super::free_list::FreeList;
use super::free_list::FreeListElement;
use super::free_list::OwnedFreeListElement;
use ::std::cmp::min;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ops::Deref;
use ::std::ops::DerefMut;
use ::std::ptr::null_mut;
use ::std::ptr::write;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicPtr;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::Ordering::SeqCst;


trait ExtendedAtomic<T>
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: T);
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent(&self, compare: &mut T, value: T) -> bool;
}

impl ExtendedAtomic<u32> for AtomicU32
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: u32)
	{
		unsafe { (self as *mut Self).write(Self::new(initial_value)) }
	}
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent(&self, compare: &mut u32, value: u32) -> bool
	{
		match self.compare_exchange(*compare, value, SeqCst, SeqCst)
		{
			Ok(_) => true,
			Err(updated) =>
			{
				*compare = updated;
				false
			}
		}
	}
}

impl<T> ExtendedAtomic<*mut T> for AtomicPtr<T>
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: *mut T)
	{
		unsafe { (self as *mut Self).write(Self::new(initial_value)) }
	}
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent(&self, compare: &mut *mut T, value: *mut T) -> bool
	{
		match self.compare_exchange(*compare, value, SeqCst, SeqCst)
		{
			Ok(_) => true,
			Err(updated) =>
			{
				*compare = updated;
				false
			}
		}
	}
}


#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
#[derive(Debug)]
pub(crate) struct CacheAligned<T>(T);

impl<T> Deref for CacheAligned<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<T> CacheAligned<T>
{
	#[inline(always)]
	pub(crate) const fn new(value: T) -> Self
	{
		CacheAligned(value)
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
#[derive(Debug)]
pub(crate) struct DoubleCacheAligned<T>(T);

impl<T> Deref for DoubleCacheAligned<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<T> DerefMut for DoubleCacheAligned<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<T> DoubleCacheAligned<T>
{
	#[inline(always)]
	pub(crate) const fn new(value: T) -> Self
	{
		DoubleCacheAligned(value)
	}
}

const MaximumSupportedHyperThreads: usize = 256;


// Implementation based on the paper (Hazard Pointers: Safe Memory Reclamation for Lock-Free Objects)[http://web.cecs.pdx.edu/~walpole/class/cs510/papers/11.pdf] by Maged M Michael.
#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
pub(crate) struct HazardPointerPerHyperThread<Hazardous: CtoSafe>
{
	// Cache alignment here of an 8 byte pointer to 128 bytes to try to eliminate 'false sharing'.
	hazard_pointer_per_hyper_thread: [DoubleCacheAligned<AtomicPtr<FreeListElement<Hazardous>>>; MaximumSupportedHyperThreads],
	
	// Cache alignment here to try to eliminate 'false sharing'.
	retired_lists_per_hyper_thread: [DoubleCacheAligned<Vec<NonNull<FreeListElement<Hazardous>>>>; MaximumSupportedHyperThreads],
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
	const MaximumSupportedHyperThreads: usize = MaximumSupportedHyperThreads;
	
	// This is 'R' in the paper (Hazard Pointers: Safe Memory Reclamation for Lock-Free Objects)[http://web.cecs.pdx.edu/~walpole/class/cs510/papers/11.pdf].
	// With a ReclamationThreshold of 1, this will always be true... as `retired_list_for_hyper_thread.push()` occurred above.
	const ReclamationThreshold: usize = 1;
	
	// MUST be called when queues are quiescent to clean-out any retired objects.
	// This design is not particularly safe, and will cause memory to be 'lost' in the event of a power outage.
	#[inline(always)]
	pub(crate) fn shutdown(&mut self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Hazardous>>)
	{
		let slice = &mut self.retired_lists_per_hyper_thread[.. maximum_hyper_threads];
		for retired_list_for_hyper_thread in slice.iter_mut()
		{
			for retired_object in retired_list_for_hyper_thread.drain(..)
			{
				free_list.push(OwnedFreeListElement::from_non_null(retired_object))
			}
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
					let mut array: [DoubleCacheAligned<Vec<NonNull<FreeListElement<Hazardous>>>>; MaximumSupportedHyperThreads] = uninitialized();
					for element in array.iter_mut()
					{
						// TODO: Eliminate Vec, move to a fixed-size array?
						// Costly: A list can grow as long as the number of hyper threads, but performance will be increased. In exchange for much, much higher memory usage (but memory usage that is fixed at allocation time, so can not run out).
						// Current estimate is 512Kb+ per queue for 256 hyper threads.
						write(element, DoubleCacheAligned::new(Vec::with_capacity(Self::ReclamationThreshold)))
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
	pub(crate) fn retire(&mut self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Hazardous>>, hyper_thread_index: usize, retire_this_object: NonNull<FreeListElement<Hazardous>>)
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
	fn reclaim(&mut self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Hazardous>>, hyper_thread_index: usize, original_length: usize)
	{
		// Similar to Vec.retain() but changes particularly include truncate() replaced with logic to push to a free list.
		
		let mut deletion_count = 0;
		{
			for index in 0 .. original_length
			{
				let our_retired_object = unsafe { *self.retired_list_for_hyper_thread(hyper_thread_index).get_unchecked(index) };
				let delete = self.scan_all_hyper_threads_to_see_if_they_are_still_using_a_reference_to_our_retired_object_and_if_not_delete_it(maximum_hyper_threads, our_retired_object);
				
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
		
		let mut other_hyper_thread_index = 0;
		while other_hyper_thread_index < maximum_hyper_threads
		{
			if self.hazard_pointer_for_hyper_thread(other_hyper_thread_index).load(SeqCst) == our_retired_object
			{
				// Another hyper thread is using a reference to `our_retired_object`, so return early and try the next our_retired_object_index
				return false
			}
			
			other_hyper_thread_index += 1;
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
		unsafe { self.retired_lists_per_hyper_thread.get_unchecked(hyper_thread_index) }.deref()
	}
	
	#[inline(always)]
	fn retired_list_for_hyper_thread_mut(&mut self, hyper_thread_index: usize) -> &mut Vec<NonNull<FreeListElement<Hazardous>>>
	{
		unsafe { self.retired_lists_per_hyper_thread.get_unchecked_mut(hyper_thread_index) }.deref_mut()
	}
}

// -2 makes Node<T> exactly 8192 bytes, or 2 pages.
// -3 makes OwnedFreeListElement<Node<Value>> 8192 bytes (OwnedFreeListElement has a 8 byte next pointer for the first field).
const ExclusiveMaximumNumberOfItems: usize = 1024 - 3;

/// A node.
pub struct Node<Value: CtoSafe>
{
	dequeue_index_in_items: AtomicU32,
	items: [AtomicPtr<Value>; ExclusiveMaximumNumberOfItems],
	enqueue_index_in_items: AtomicU32,
	next: AtomicPtr<FreeListElement<Node<Value>>>,
}

impl<Value: CtoSafe> Debug for Node<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "Node<Value>")
	}
}

impl<Value: CtoSafe> CtoSafe for Node<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let mut dequeue_index_in_items = self.dequeue_index_in_items.load(Relaxed) as usize;
		let enqueue_index_in_items = self.dequeue_index_in_items.load(Relaxed) as usize;
		let maximum = min(ExclusiveMaximumNumberOfItems, enqueue_index_in_items + 1);
		
		while dequeue_index_in_items < maximum
		{
			let item = unsafe { self.items.get_unchecked(dequeue_index_in_items) }.load(Relaxed);
			if item.is_not_null()
			{
				item.to_non_null().mutable_reference().cto_pool_opened(cto_pool_arc)
			}
			dequeue_index_in_items += 1
		}
		
		let next = self.next_relaxed();
		if next.is_not_null()
		{
			OwnedFreeListElement::from_non_null_pointer(next).cto_pool_opened(cto_pool_arc)
		}
	}
}

impl<Value: CtoSafe> Node<Value>
{
	const ExclusiveMaximumNumberOfItems: usize = ExclusiveMaximumNumberOfItems;
	
	const MaximumIndex: u32 = (Self::ExclusiveMaximumNumberOfItems - 1) as u32;
	
	// Starts with the first entry pre-filled and enqidx at 1.
	#[inline(always)]
	fn initialize(&mut self, item: *mut Value)
	{
		debug_assert_ne!(Self::ExclusiveMaximumNumberOfItems, 0, "ExclusiveMaximumNumberOfItems should not be zero");
		
		self.dequeue_index_in_items.initialize(0);
		self.enqueue_index_in_items.initialize(1);
		self.next.initialize(null_mut());
		
		self.relaxed_store_of_item(0, item);
		
		let mut item_index = 1;
		while item_index < Self::ExclusiveMaximumNumberOfItems
		{
			self.relaxed_store_of_item(item_index, null_mut());
			item_index += 1;
		}
	}
	
	#[inline(always)]
	fn is_node_full(next_enqueue_index: u32) -> bool
	{
		next_enqueue_index > Self::MaximumIndex
	}
	
	#[inline(always)]
	fn is_node_drained(next_dequeue_index: u32) -> bool
	{
		next_dequeue_index > Self::MaximumIndex
	}
	
	#[inline(always)]
	fn next(&self) -> *mut FreeListElement<Self>
	{
		self.next.load(SeqCst)
	}
	
	#[inline(always)]
	fn next_relaxed(&self) -> *mut FreeListElement<Self>
	{
		self.next.load(Relaxed)
	}
	
	#[inline(always)]
	fn cas_next(&self, compare: &mut *mut FreeListElement<Node<Value>>, value: *mut FreeListElement<Node<Value>>) -> bool
	{
		self.next.compare_and_swap_strong_sequentially_consistent(compare, value)
	}
	
	#[inline(always)]
	fn relaxed_store_of_item(&self, item_index: usize, item: *mut Value)
	{
		debug_assert!(item_index <= ::std::u32::MAX as usize, "item_index exceeds u32::MAX");
		self.item(item_index as u32).store(item, Relaxed);
	}
	
	#[inline(always)]
	fn item(&self, item_index: u32) -> &AtomicPtr<Value>
	{
		debug_assert!((item_index as usize) < Self::ExclusiveMaximumNumberOfItems, "item_index '{}' exceeds Self::ExclusiveMaximumNumberOfItems '{}'", item_index, Self::ExclusiveMaximumNumberOfItems);
		
		unsafe { self.items.get_unchecked(item_index as usize) }
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
struct FetchAndAddArrayQueue<Value: CtoSafe>
{
	// head and tail should never be null.
	head: DoubleCacheAligned<AtomicPtr<FreeListElement<Node<Value>>>>,
	tail: DoubleCacheAligned<AtomicPtr<FreeListElement<Node<Value>>>>,
	maximum_hyper_threads: usize,
	hazard_pointers: Box<HazardPointerPerHyperThread<Node<Value>>>,
	free_list: CtoStrongArc<FreeList<Node<Value>>>,
	reference_counter: AtomicUsize,
	cto_pool_arc: CtoPoolArc,
}

impl<Value: CtoSafe> CtoSafe for FetchAndAddArrayQueue<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.free_list.cto_pool_opened(cto_pool_arc);
		cto_pool_arc.write(&mut self.cto_pool_arc);
		
		self.reinitialize_hazard_pointers();
		
		// head is never null.
		OwnedFreeListElement::from_non_null_pointer(self.head.load(Relaxed)).cto_pool_opened(cto_pool_arc);
		
		// We do not need to the same as above from tail, as tail should be reachable from head via .next on Node instances.
	}
}

impl<Value: CtoSafe> Drop for FetchAndAddArrayQueue<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		const ArbitraryHyperThreadIndex: usize = 0;
		
		// Drain the queue.
		while self.dequeue(ArbitraryHyperThreadIndex).is_some()
		{
		}
		
		// Destroy the last node; the head always has a value.
		self.free_list.push(OwnedFreeListElement::from_non_null_pointer(self.head.load(SeqCst)));
		
		// Destroy ourselves
		let cto_pool_arc = self.cto_pool_arc.clone();
		cto_pool_arc.free_pointer(self)
	}
}

impl<Value: CtoSafe> CtoStrongArcInner for FetchAndAddArrayQueue<Value>
{
	#[inline(always)]
	fn reference_counter(&self) -> &AtomicUsize
	{
		&self.reference_counter
	}
}

quick_error!
{
	/// Reason for failing to instantiate.
	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum OutOfMemoryError
	{
		/// The free list has no more space.
		FreeList
		{
			description("No more space (currently) available in FreeList")
		}
		
		/// The cto pool arc has no more space.
		CtoPoolArc(cause: PmdkError)
		{
			cause(cause)
			description("No more space (currently) available in CtoPoolArc")
		}
	}
}

impl<Value: CtoSafe> FetchAndAddArrayQueue<Value>
{
	const MaximumSupportedHyperThreads: usize = MaximumSupportedHyperThreads;
	
	const TakenSentinel: *mut Value = !0 as *mut Value;
	
	#[inline(always)]
	fn cas_tail(&self, compare: &mut *mut FreeListElement<Node<Value>>, value: *mut FreeListElement<Node<Value>>) -> bool
	{
		self.tail.compare_and_swap_strong_sequentially_consistent(compare, value)
	}
	
	#[inline(always)]
	fn cas_head(&self, compare: &mut *mut FreeListElement<Node<Value>>, value: *mut FreeListElement<Node<Value>>) -> bool
	{
		self.head.compare_and_swap_strong_sequentially_consistent(compare, value)
	}
	
	#[inline(always)]
	pub fn new(maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Node<Value>>>, cto_pool_arc: &CtoPoolArc) -> Result<CtoStrongArc<Self>, OutOfMemoryError>
	{
		debug_assert_ne!(maximum_hyper_threads, 0);
		debug_assert!(maximum_hyper_threads <= HazardPointerPerHyperThread::<Value>::MaximumSupportedHyperThreads);
		
		let mut sentinel_owned_free_list_element = match free_list.pop()
		{
			None => return Err(OutOfMemoryError::FreeList),
			Some(free_list_element) => free_list_element,
		};
		
		let mut this = match cto_pool_arc.pool_pointer().malloc::<Self>()
		{
			Err(pmdk_error) =>
			{
				free_list.push(sentinel_owned_free_list_element);
				return Err(OutOfMemoryError::CtoPoolArc(pmdk_error))
			},
			Ok(pointer) => pointer.to_non_null(),
		};
		
		unsafe
		{
			let this = this.mutable_reference();
			write(&mut this.maximum_hyper_threads, maximum_hyper_threads);
			this.reinitialize_hazard_pointers();
			
			let pointer = sentinel_owned_free_list_element.as_ptr();
			let sentinel_node = sentinel_owned_free_list_element.deref_mut().deref_mut();
			
			sentinel_node.initialize(null_mut());
			sentinel_node.enqueue_index_in_items.store(0, Relaxed);
			this.head.store(pointer, Relaxed);
			this.tail.store(pointer, Relaxed);
			
			write(&mut this.free_list, free_list.clone());
			write(&mut this.reference_counter, Self::new_reference_counter());
			write(&mut this.cto_pool_arc, cto_pool_arc.clone());
		}
		
		Ok(CtoStrongArc::new(this))
	}
	
	#[inline(always)]
	pub fn enqueue(&self, hyper_thread_index: usize, item: NonNull<Value>)
	{
		let item = item.as_ptr();
		assert_ne!(item, Self::TakenSentinel, "item pointer can not be the TakenSentinel '0x{:X}'", Self::TakenSentinel as usize);
		
		loop
		{
			let mut tail = self.protect(hyper_thread_index, &self.tail);
			let tail_non_null = tail.to_non_null();
			let tail_reference = tail_non_null.reference();
			let next_enqueue_index = tail_reference.enqueue_index_in_items.fetch_add(1, SeqCst);
			
			if Node::<Value>::is_node_full(next_enqueue_index)
			{
				if tail != self.tail.load(SeqCst)
				{
					continue;
				}
				
				let next = tail_reference.next();
				if next.is_null()
				{
					// TODO: Handle out-of-memory
					let mut new_node = self.free_list.pop().expect("OUT OF MEMORY");
					new_node.initialize(item);
					// TODO: Don't care about null_mut()
					let new_node_pointer = new_node.as_ptr();
					if tail_reference.cas_next(&mut null_mut(), new_node_pointer)
					{
						// TODO: Don't care about tail
						self.cas_tail(&mut tail, new_node_pointer);
						self.clear(hyper_thread_index);
						return
					}
					self.free_list.push(new_node)
				}
				else
				{
					// TODO: Don't care about tail
					self.cas_tail(&mut tail, next);
				}
				continue
			}
			
			if tail_reference.item(next_enqueue_index).compare_and_swap_strong_sequentially_consistent(&mut null_mut(), item)
			{
				self.clear(hyper_thread_index);
				return
			}
		}
	}
	
	#[inline(always)]
	pub fn dequeue(&mut self, hyper_thread_index: usize) -> Option<NonNull<Value>>
	{
		loop
		{
			let mut head = self.protect(hyper_thread_index, &self.head);
			let head_non_null = head.to_non_null();
			let head_reference = head_non_null.reference();
			
			if head_reference.dequeue_index_in_items.load(SeqCst) >= head_reference.enqueue_index_in_items.load(SeqCst) && head_reference.next().is_null()
			{
				break
			}
			
			let next_dequeue_index = head_reference.dequeue_index_in_items.fetch_add(1, SeqCst);
			if Node::<Value>::is_node_drained(next_dequeue_index)
			{
				// This node has been drained: check if there is another one.
				let next = head_reference.next();
				if next.is_null()
				{
					break
				}
				
				if self.cas_head(&mut head, next)
				{
					self.retire(hyper_thread_index, head_non_null)
				}
				
				continue
			}
			
			let item = head_reference.item(next_dequeue_index).swap(Self::TakenSentinel, SeqCst);
			debug_assert_ne!(item, Self::TakenSentinel, "dequeued item should never be the TakenSentinel");
			
			if item.is_not_null()
			{
				self.clear(hyper_thread_index);
				return Some(item.to_non_null())
			}
		}
		
		self.clear(hyper_thread_index);
		None
	}
	
	#[inline(always)]
	fn reinitialize_hazard_pointers(&mut self)
	{
		unsafe { write(&mut self.hazard_pointers, HazardPointerPerHyperThread::new()) }
	}
	
	#[inline(always)]
	fn protect(&self, hyper_thread_index: usize, atom: &AtomicPtr<FreeListElement<Node<Value>>>) -> *mut FreeListElement<Node<Value>>
	{
		self.hazard_pointers.protect(hyper_thread_index, atom)
	}
	
	#[inline(always)]
	fn clear(&self, hyper_thread_index: usize)
	{
		self.hazard_pointers.clear(hyper_thread_index);
	}
	
	#[inline(always)]
	fn retire(&mut self, hyper_thread_index: usize, retire_this_object: NonNull<FreeListElement<Node<Value>>>)
	{
		self.hazard_pointers.retire(self.maximum_hyper_threads, &self.free_list, hyper_thread_index, retire_this_object)
	}
}
