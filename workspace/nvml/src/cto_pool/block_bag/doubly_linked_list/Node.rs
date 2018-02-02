// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct Node<T>
{
	next: AtomicLink<T>,
	previous: AtomicLink<T>,
	reference_count: AtomicUsize,
	value: Cell<Option<NonNull<T>>>,
}

impl<T> PartialEq for Node<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self as *const Self == other as *const Self
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		self as *const Self != other as *const Self
	}
}

impl<T> Eq for Node<T>
{
}

impl<T> Node<T>
{
	#[inline(always)]
	pub(crate) fn move_value(&self) -> Option<NonNull<T>>
	{
		// TODO: Should this be atomic?
		let result = self.value.get();
		self.value.set(None);
		result
	}
	
	#[inline(always)]
	pub(crate) fn ref_value(&self) -> Option<NonNull<T>>
	{
		self.value.get()
	}
	
	// By making this 1, a dummy node will never be freed.
	const InitialReferenceCountToPreventHeadOrTailDummyNodeEverBeingFreed: usize = 1;
	
	const InitialReferenceCountForRegularNodes: usize = 0;
	
	#[inline(always)]
	pub(crate) const fn head_or_tail_dummy_node() -> Self
	{
		Self::empty_node(Self::InitialReferenceCountToPreventHeadOrTailDummyNodeEverBeingFreed)
	}
	
	#[inline(always)]
	const fn empty_node(initial_reference_count: usize) -> Self
	{
		Self
		{
			value: Cell::new(None),
			next: AtomicLink::Null,
			previous: AtomicLink::Null,
			reference_count: AtomicUsize::new(initial_reference_count),
		}
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn NewNode(value: NonNull<T>) -> DereferencedLink<T>
	{
		let malloc = Box::new(Self::empty_node(Self::InitialReferenceCountForRegularNodes));
		
		malloc.value.set(Some(value));
		
		let node = Box::into_raw_non_null(malloc);
		
		// Ensures that node can then be later called with ReleaseRef()
		let node = AtomicLink::from_raw_pointer(node.as_ptr());
		node.DeRefLink()
	}
	
	/// "New nodes are created dynamically with the `CreateNode` function which in turn makes use of the `NewNode` function for the actual memory allocation".
	#[allow(non_snake_case)]
	#[inline(always)]
	fn CreateNode<'a>(value: NonNull<T>) -> DereferencedLink<T>
	{
		// CN1
		let node = Self::NewNode(value);
		
		// CN2
		// (logically done in NewNode)
		
		// CN3
		node
	}
	
	/// `TerminateNode` makes sure that none of the node’s contained links have any claim on any other node.
	/// `TerminateNode` is called on a deleted node when there are no claims from any other node or thread to the node.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn TerminateNode(&self)
	{
		let node = self;
		
		// TN1
		node.previous.StoreRef(StackLink::Null);
		
		// TN2
		node.next.StoreRef(StackLink::Null)
	}
	
	/// The procedure `CleanUpNode` makes sure that all references from the links in the given node point to active nodes, thus removing redundant reference chains passing through an arbitrary number of deleted nodes.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn CleanUpNode(&self)
	{
		let node = self;
		
		// CU1
		let mut prev: DereferencedLink<T>;
		loop
		{
			// CU2
			prev = node.previous.DeRefLink();
			
			// CU3
			if prev.previous_link_stack().has_delete_tag()
			{
				break
			}
			
			// CU4
			let prev2 = prev.previous_link_atomic().DeRefLink();
			
			// CU5
			node.previous.CASRef(prev.with_delete_tag_link_stack(), prev2.with_delete_tag_link_stack());
			
			// CU6
			prev2.ReleaseRef();
			prev.ReleaseRef()
		}
		
		// CU7
		let mut next: DereferencedLink<T>;
		loop
		{
			// CU8
			next = node.next.DeRefLink();
			
			// CU9
			if next.next_link_stack().has_delete_tag()
			{
				break
			}
			
			// CU10
			let next2 = next.next_link_atomic().DeRefLink();
			
			// CU11
			node.next.CASRef(next.with_delete_tag_link_stack(), next2.with_delete_tag_link_stack());
			
			// CU12
			next2.ReleaseRef();
			next.ReleaseRef()
		}
		
		// CU13
		prev.ReleaseRef();
		next.ReleaseRef()
	}
	
	#[inline(always)]
	fn increment_reference_count(&self)
	{
		self.reference_count.fetch_add(1, AcqRel);
	}
	
	#[inline(always)]
	fn decrement_reference_count(&self)
	{
		debug_assert_ne!(self.reference_count.load(Acquire), 0, "reference_count should never be zero during `Node.decrement_reference_count()`");
		
		self.reference_count.fetch_sub(1, AcqRel);
	}
}
