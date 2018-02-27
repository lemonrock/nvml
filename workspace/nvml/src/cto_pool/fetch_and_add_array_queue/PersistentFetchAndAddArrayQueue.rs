// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Rust implementation of a persistent variant of <https://github.com/pramalhe/ConcurrencyFreaks/blob/master/CPP/queues/array/FAAArrayQueue.hpp>.
#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
pub struct PersistentFetchAndAddArrayQueue<Value: CtoSafe>
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

impl<Value: CtoSafe> CtoSafe for PersistentFetchAndAddArrayQueue<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.free_list.cto_pool_opened(cto_pool_arc);
		cto_pool_arc.write(&mut self.cto_pool_arc);
		
		self.reinitialize_maximum_hyper_threads();
		self.reinitialize_hazard_pointers();
		
		// head is never null.
		OwnedFreeListElement::from_non_null(self.head()).cto_pool_opened(cto_pool_arc);
		
		// We do not need to the same as above from tail, as tail should be reachable from head via .next on Node instances.
	}
}

impl<Value: CtoSafe> Drop for PersistentFetchAndAddArrayQueue<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		// Drain the queue.
		while self.dequeue_faster(hyper_thread_index()).is_some()
		{
		}
		
		// Destroy the last node; the head always has a value.
		self.free_list.push(OwnedFreeListElement::from_non_null(self.head()));
		
		// Destroy ourselves
		let cto_pool_arc = self.cto_pool_arc.clone();
		cto_pool_arc.free_pointer(self)
	}
}

impl<Value: CtoSafe> CtoStrongArcInner for PersistentFetchAndAddArrayQueue<Value>
{
	#[inline(always)]
	fn reference_counter(&self) -> &AtomicUsize
	{
		&self.reference_counter
	}
}

impl<Value: CtoSafe> PersistentFetchAndAddArrayQueue<Value>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(free_list: &CtoStrongArc<FreeList<Node<Value>>>, cto_pool_arc: &CtoPoolArc) -> Result<CtoStrongArc<Self>, OutOfMemoryError>
	{
		let maximum_hyper_threads = maximum_number_of_hyper_threads();
		
		let initial_free_list_element = match free_list.pop()
		{
			None => return Err(OutOfMemoryError::FreeList),
			Some(mut initial_free_list_element) =>
				{
					initial_free_list_element.initialize_for_initial();
					initial_free_list_element
				},
		};
		
		let mut this = match cto_pool_arc.pool_pointer().malloc::<Self>()
		{
			Err(pmdk_error) =>
				{
					free_list.push(initial_free_list_element);
					return Err(OutOfMemoryError::CtoPoolArc(pmdk_error))
				},
			Ok(pointer) => pointer.to_non_null(),
		};
		
		unsafe
		{
			this.mutable_reference().initialize(maximum_hyper_threads, free_list, cto_pool_arc, initial_free_list_element)
		}
		
		Ok(CtoStrongArc::new(this))
	}
	
	#[inline(always)]
	unsafe fn initialize(&mut self, maximum_hyper_threads: usize, free_list: &CtoStrongArc<FreeList<Node<Value>>>, cto_pool_arc: &CtoPoolArc, initial_free_list_element: OwnedFreeListElement<Node<Value>>)
	{
		write(&mut self.maximum_hyper_threads, maximum_hyper_threads);
		self.reinitialize_hazard_pointers();
		
		self.head_initialize(initial_free_list_element.to_non_null());
		self.tail_initialize(initial_free_list_element.to_non_null());
		
		write(&mut self.free_list, free_list.clone());
		write(&mut self.reference_counter, Self::new_reference_counter());
		write(&mut self.cto_pool_arc, cto_pool_arc.clone());
	}
	
	/// MUST be called when queues are quiescent to clean-out any retired objects.
	/// This design is not particularly safe, and will cause memory to be 'lost' in the event of a power outage.
	#[inline(always)]
	pub fn shutdown(&mut self)
	{
		self.hazard_pointers.shutdown(self.maximum_hyper_threads, &self.free_list)
	}
	
	/// Enqueue an item.
	#[inline(always)]
	pub fn enqueue(&self, item: NonNull<Value>)
	{
		self.enqueue_faster(hyper_thread_index(), item)
	}
	
	/// Enqueue an item.
	/// Slightly faster as no need to look up `hyper_thread_index`.
	#[inline(always)]
	pub fn enqueue_faster(&self, hyper_thread_index: usize, item: NonNull<Value>)
	{
		debug_assert!(hyper_thread_index < self.maximum_hyper_threads, "hyper_thread_index is too large");
		
		loop
		{
			let tail_non_null = self.protect(hyper_thread_index, &self.tail);
			let tail = tail_non_null.reference();
			
			let next_enqueue_index = tail.fetch_then_increment_enqueue_index_in_items();
			
			if Node::<Value>::is_node_full(next_enqueue_index)
			{
				if self.tail_is_no_longer(tail_non_null)
				{
					continue;
				}
				
				let next = tail.next();
				if next.is_null()
				{
					// TODO: Handle out-of-memory
					let mut new_node = self.free_list.pop().expect("OUT OF MEMORY");
					new_node.initialize_for_next(item);
					if tail.next_compare_and_swap_strong_sequentially_consistent(null_mut(), new_node.as_ptr())
					{
						self.tail_compare_and_swap_strong_sequentially_consistent(tail, new_node.to_non_null());
						self.clear(hyper_thread_index);
						return
					}
					self.free_list.push(new_node)
				}
				else
				{
					self.tail_compare_and_swap_strong_sequentially_consistent(tail, next.to_non_null());
				}
				continue
			}
			
			if tail.compare_and_swap_strong_sequentially_consistent_item(next_enqueue_index, item)
			{
				self.clear(hyper_thread_index);
				return
			}
		}
	}
	
	/// Dequeue an item.
	#[inline(always)]
	pub fn dequeue(&self) -> Option<NonNull<Value>>
	{
		self.dequeue_faster(hyper_thread_index())
	}
	
	/// Dequeue an item.
	/// Slightly faster as no need to look up `hyper_thread_index`.
	#[inline(always)]
	pub fn dequeue_faster(&self, hyper_thread_index: usize) -> Option<NonNull<Value>>
	{
		debug_assert!(hyper_thread_index < self.maximum_hyper_threads, "hyper_thread_index is too large");
		
		loop
		{
			let head_non_null = self.protect(hyper_thread_index, &self.head);
			let head = head_non_null.reference();
			
			if head.dequeue_index_in_items() >= head.enqueue_index_in_items() && head.next().is_null()
			{
				return self.release_hazard_pointer_and_return_dequeued_item(hyper_thread_index, None)
			}
			
			let next_dequeue_index = head.fetch_then_increment_dequeue_index_in_items();
			if Node::<Value>::is_node_drained(next_dequeue_index)
			{
				let next = head.next();
				
				// There isn't another node after this one, ie the queue is completely empty.
				if next.is_null()
				{
					return self.release_hazard_pointer_and_return_dequeued_item(hyper_thread_index, None)
				}
				
				// There is another node after this one.
				// Retire this one.
				if self.head_compare_and_swap_strong_sequentially_consistent(head, next.to_non_null())
				{
					self.retire(hyper_thread_index, head_non_null)
				}
				
				continue
			}
			
			let item = head.swap_item_for_taken(next_dequeue_index);
			
			if item.is_not_null()
			{
				return self.release_hazard_pointer_and_return_dequeued_item(hyper_thread_index, Some(item.to_non_null()))
			}
		}
	}
	
	#[inline(always)]
	fn release_hazard_pointer_and_return_dequeued_item(&self, hyper_thread_index: usize, dequeued_item: Option<NonNull<Value>>) -> Option<NonNull<Value>>
	{
		self.clear(hyper_thread_index);
		dequeued_item
	}
	
	#[inline(always)]
	fn reinitialize_maximum_hyper_threads(&mut self)
	{
		unsafe { write(&mut self.maximum_hyper_threads, maximum_number_of_hyper_threads()) }
	}
	
	#[inline(always)]
	fn reinitialize_hazard_pointers(&mut self)
	{
		unsafe { write(&mut self.hazard_pointers, HazardPointerPerHyperThread::new()) }
	}
	
	#[inline(always)]
	fn protect(&self, hyper_thread_index: usize, atom: &AtomicPtr<FreeListElement<Node<Value>>>) -> NonNull<FreeListElement<Node<Value>>>
	{
		self.hazard_pointers.protect(hyper_thread_index, atom).to_non_null()
	}
	
	#[inline(always)]
	fn clear(&self, hyper_thread_index: usize)
	{
		self.hazard_pointers.clear(hyper_thread_index);
	}
	
	#[inline(always)]
	fn retire(&self, hyper_thread_index: usize, retire_this_object: NonNull<FreeListElement<Node<Value>>>)
	{
		self.hazard_pointers.retire(self.maximum_hyper_threads,&self.free_list, hyper_thread_index, retire_this_object)
	}
	
	#[inline(always)]
	fn head_initialize(&self, initial_value: NonNull<FreeListElement<Node<Value>>>)
	{
		self.head.store(initial_value.as_ptr(), Relaxed)
	}
	
	#[inline(always)]
	fn head(&self) -> NonNull<FreeListElement<Node<Value>>>
	{
		self.head.load(SeqCst).to_non_null()
	}
	
	#[inline(always)]
	fn head_compare_and_swap_strong_sequentially_consistent(&self, head_was: &FreeListElement<Node<Value>>, next: NonNull<FreeListElement<Node<Value>>>) -> bool
	{
		self.head.compare_and_swap_strong_sequentially_consistent(head_was as *const _ as *mut _, next.as_ptr())
	}
	
	#[inline(always)]
	fn tail_is_no_longer(&self, original_tail: NonNull<FreeListElement<Node<Value>>>) -> bool
	{
		original_tail.as_ptr() != self.tail().as_ptr()
	}
	
	#[inline(always)]
	fn tail_initialize(&self, initial_value: NonNull<FreeListElement<Node<Value>>>)
	{
		self.tail.store(initial_value.as_ptr(), Relaxed)
	}
	
	#[inline(always)]
	fn tail(&self) -> NonNull<FreeListElement<Node<Value>>>
	{
		self.tail.load(SeqCst).to_non_null()
	}
	
	#[inline(always)]
	fn tail_compare_and_swap_strong_sequentially_consistent(&self, tail_was: &FreeListElement<Node<Value>>, value: NonNull<FreeListElement<Node<Value>>>) -> bool
	{
		self.tail.compare_and_swap_strong_sequentially_consistent(tail_was as *const _ as *mut _, value.as_ptr())
	}
}
