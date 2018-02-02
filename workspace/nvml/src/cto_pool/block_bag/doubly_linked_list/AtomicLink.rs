// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


type AtomicLink<T> = AtomicTaggedPointer<Node<T>>;

// aka `impl AtomicLink<T>`.
impl<T> AtomicTaggedPointer<Node<T>>
{
	/// This function ***MUST*** only ever called on `prev` pointers.
	/// It sets the delete mark on the `prev` pointer.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn SetMark(&self)
	{
		let link: &AtomicLink<T> = self;
		
		// SM1
		loop
		{
			// SM2
			// The code below is logically equivalent to let node = *link;
			let node: StackLink<T> = link.deref_link();
			
			// SM3
			if node.has_delete_tag() || link.CAS(node, node.with_delete_tag())
			{
				break
			}
		}
	}
	
	/// The function `DeRefLink` safely dereferences the given link, doing something (such as setting a hazard pointer or reference count), thus guaranteeing the safety of future accesses to the returned node.
	/// In particular, the calling thread can safely dereference and / or update any links in the returned node subsequently.
	/// Logically, this function is just `*link`, but it makes sure that the Node pointed to can not be free'd until `ReleaseRef` is called.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn DeRefLink(&self) -> DereferencedLink<T>
	{
		/*
			let link = self;
			
			// D1 (choose index)
			let index = xxxx;
			
			// D2
			loop
			{
				// D3
				// ie atomic load of value pointed to.
				// let node = *link;
				let node: StackLink<T> = link.deref_link();
				
				// assign hazard pointer
				// D4
				HP[threadId][index] = node;
				
				// D5
				// atomic load of value pointed to still matches.
				// if *link == node
				if link.deref_link() == node
				{
					// D6
					return node
				}
			}
		
		*/
		let non_atomic_tagged_pointer_to_node = self.acquire_spinlock();
		
		{
			if non_atomic_tagged_pointer_to_node.is_not_null()
			{
				non_atomic_tagged_pointer_to_node.as_reference().increment_reference_count();
			}
		}
		
		self.release_spinlock(non_atomic_tagged_pointer_to_node);
		
		DereferencedLink(self.without_lock_dereference_tag_but_with_delete_tag_if_set())
	}
	
	/// Logically equivalent to let `let some value = *self`, eg `node = *link`, but ensures the lock dereference tag is ignored (cleared).
	#[inline(always)]
	fn deref_link(&self) -> StackLink<T>
	{
		self.without_lock_dereference_tag_but_with_delete_tag_if_set()
	}
	
	/// The function `StoreRef` is used to update a link for which ***there cannot be any concurrent updates.***
	/// This function should only be called with `node` pointers **without** the delete flag set.
	/// This procedure will make sure that any thread that then calls `DeRefLink` on the link can safely do so,.
	/// The requirements are that that no other thread than the calling thread will possibly write concurrently to the link (otherwise `CASRef` should be invoked instead).
	/// Definition inspired by that in "Efficient and Reliable Lock-Free Memory Reclamation Based on Reference Counting", Gildentam et al 2005.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn StoreRef(&self, new_node: StackLink<T>)
	{
		// Called `address` in Sundell & Tsigas.
		let link = self;
		
		// S1
		// The code below is logically equivalent to let old_node = *link;
		let old_node: StackLink<T> = link.deref_link();
		
		// S2
		// *link = new_node;
		link.set(new_node);
		
		// S3
		if new_node.is_not_null()
		{
			// S4
			new_node.as_reference().increment_reference_count()
			
			// S7
			// (no-op for us as we do not use a tracing garbage collector)
		}
		
		// S6
		if old_node.is_not_null()
		{
			old_node.as_reference().decrement_reference_count();
		}
	}
	
	/// The function `CASRef` is used to update a link for ***which there might be concurrent updates.***
	/// It returns `true` if the update was successful and `false` otherwise.
	/// Definition inspired by a synthesis of Sundell & Tsigas, 2008's `CASRef` and Gildentam et al 2005 `CompareAndSwapRef`.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CASRef(&self, old_node: StackLink<T>, new_node: StackLink<T>) -> bool
	{
		// Called `address` in Sundell & Tsigas.
		let link = self;
		
		// C1
		if link.CAS(old_node, new_node)
		{
			// C2
			if new_node.is_not_null()
			{
				// C3
				new_node.as_reference().increment_reference_count()
				
				// C4
				// (no-op for us as we do not use a tracing garbage collector)
			}
			
			// C5
			if old_node.is_not_null()
			{
				old_node.as_reference().decrement_reference_count()
			}
			
			// C6
			return true
		}
		
		// C7
		false
	}
}
