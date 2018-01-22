// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct Node<T>
{
	value: Option<NonNull<T>>,
	prev: TaggedPointerToNode<T>,
	next: TaggedPointerToNode<T>,
	
	reference_count: AtomicUsize,
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
	// FIXME: Is this actually 1?
	// TODO: Is this actually 1?
	const InitialReferenceCountToPreventEverBeingFreed: usize = 2;
	
	// FIXME: Is this actually 0?
	// TODO: Is this actually 0?
	const InitialReferenceCountForRegularNodes: usize = 1;
	
	#[inline(always)]
	pub(crate) const fn head_or_tail_dummy_node() -> Self
	{
		Self::empty_node(Self::InitialReferenceCountToPreventEverBeingFreed)
	}
	
	#[inline(always)]
	pub(crate) fn move_value(&mut self) -> Option<NonNull<T>>
	{
		replace(&mut self.value, None)
	}
	
	#[inline(always)]
	pub(crate) fn clone_value(&mut self) -> Option<NonNull<T>>
	{
		replace(&mut self.value, None)
	}
	
	#[inline(always)]
	const fn empty_node(initial_reference_count: usize) -> Self
	{
		Self
		{
			value: None,
			prev: TaggedPointerToNode::Null,
			next: TaggedPointerToNode::Null,
			reference_count: AtomicUsize::new(initial_reference_count),
		}
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn NewNode() -> NonNull<Self>
	{
		// TODO: Review if this is 1 or 0.
		let malloc = Box::new(Self::empty_node(Self::InitialReferenceCountForRegularNodes));
		Box::into_raw_non_null(malloc)
	}
	
	/// "New nodes are created dynamically with the CreateNode function which in turn makes use of the NewNode function for the actual memory allocation".
	#[allow(non_snake_case)]
	#[inline(always)]
	fn CreateNode<'a>(value: NonNull<T>) -> TaggedPointerToNode<T>
	{
		// CN1
		let mut node = Self::NewNode();
		
		// CN2
		unsafe { node.as_mut() }.value = Some(value);
		
		// CN3
		TaggedPointerToNode
		{
			tagged_pointer: node.as_ptr() as usize,
			phantom_data: PhantomData,
		}
	}
	
	/// The procedure `DeleteNode` should be called when a node has been logically removed from the data structure and its memory should eventually be reclaimed.
	/// The user operation that called `DeleteNode` is responsible\* for removing all references to the deleted node from the active nodes in the data structure.
	/// This is similar to what is required when calling a memory allocator directly in a sequential data structure.
	/// However, independently of the state at the call of `DeleteNode` and of when concurrent operations eventually remove their (own or created) references to the deleted node, 'BEWARE&CLEANUP' will not reclaim the deleted node until it is safe to do so, ie, when there are no threads that could potentially access the node anymore, thus completely avoiding the possibility of dangling pointers.
	/// \* After the call of `DeleteNode`, concurrent operations may still use references to the deleted node and might even temporary add links to it, although concurrent operations that observe a deleted node are supposed to eventually remove all references and links to it.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn DeleteNode(&self)
	{
		let _node = self;
		
		unimplemented!("External definition required")
	}
	
	/// `TerminateNode` makes sure that none of the node’s contained links have any claim on any other node.
	/// `TerminateNode` is called on a deleted node when there are no claims from any other node or thread to the node.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn TerminateNode(&self)
	{
		let node = self;
		
		// TN1
		node.prev.StoreRef(TaggedPointerToNode::Null);
		
		// TN2
		node.next.StoreRef(TaggedPointerToNode::Null)
	}
	
	/// The procedure `CleanUpNode` makes sure that all references from the links in the given node point to active nodes, thus removing redundant reference chains passing through an arbitrary number of deleted nodes.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn CleanUpNode(&self)
	{
		let node = self;
		
		// CU1
		let mut prev;
		loop
		{
			// CU2
			prev = node.prev.DeRefLink();
			
			// CU3
			if prev.prev.d_is_false()
			{
				break
			}
			
			// CU4
			let prev2 = prev.prev.DeRefLink();
			
			// CU5
			node.prev.CASRef(prev.p().with_delete_mark(), prev2.p().with_delete_mark());
			
			// CU6
			prev2.ReleaseRef2(prev);
		}
		
		// CU7
		let mut next;
		loop
		{
			// CU8
			next = node.next.DeRefLink();
			
			// CU9
			if next.next.d_is_false()
			{
				break
			}
			
			// CU10
			let next2 = next.next.DeRefLink();
			
			// CU11
			node.next.CASRef(next.p().with_delete_mark(), next2.p().with_delete_mark());
			
			// CU12
			next2.ReleaseRef2(next);
		}
		
		// CU13
		prev.ReleaseRef2(next)
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
