// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Implementation based on the paper: "Lock-free deques and doubly linked lists", by Håkan Sundell and Philippas Tsigas, 2008
/// The Sundell & Tsigas algorithm treats the deque structure as a singly-linked list with `prev` pointers that may not be up to date.
/// We wrap the algorithm's global variables of `head` and `tail` in this struct.
/// "The data structure is given an orientation by denoting the head side as being left and the tail side as being right, and we can consequently use this orientation to relate nodes as being to the left or right of each other."
#[derive(Debug)]
pub struct LockFreeDoublyLinkedListAndDeque<T>
{
	dummy_head_node: Node<T>,
	dummy_tail_node: Node<T>,
}

unsafe impl<T> Send for LockFreeDoublyLinkedListAndDeque<T>
{
}

unsafe impl<T> Sync for LockFreeDoublyLinkedListAndDeque<T>
{
}

impl<T> Drop for LockFreeDoublyLinkedListAndDeque<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		while self.remove_value_from_tail().is_some()
		{
		}
	}
}

impl<T> Default for LockFreeDoublyLinkedListAndDeque<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		let mut this = Self
		{
			dummy_head_node: Node::head_or_tail_dummy_node(),
			dummy_tail_node: Node::head_or_tail_dummy_node(),
		};
		
		this.dummy_head_node.next = this.tail().without_delete_mark();
		this.dummy_head_node.prev = TaggedPointerToNode::Null;
		
		this.dummy_tail_node.next = TaggedPointerToNode::Null;
		this.dummy_tail_node.prev = this.head().without_delete_mark();
		
		fence(Release);
		
		this
	}
}

impl<T> LockFreeDoublyLinkedListAndDeque<T>
{
	/// Inserts this value at the head of the list.
	#[inline(always)]
	pub fn insert_value_at_head(&self, value: NonNull<T>)
	{
		self.PushLeft(value)
	}
	
	/// Inserts this value at the tail of the list.
	#[inline(always)]
	pub fn insert_value_at_tail(&self, value: NonNull<T>)
	{
		self.PushRight(value)
	}
	
	/// Removes a value from the head of the list, if any.
	#[inline(always)]
	pub fn remove_value_from_head(&self) -> Option<NonNull<T>>
	{
		self.PopLeft()
	}
	
	/// Removes a value from the tail of the list, if any.
	#[inline(always)]
	pub fn remove_value_from_tail(&self) -> Option<NonNull<T>>
	{
		self.PopRight()
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn PushLeft(&self, value: NonNull<T>)
	{
		// L1
		let node = Node::CreateNode(value);
		
		// L2
		let prev = self.head().DeRefLink();
		
		// L3
		let mut next = prev.next.DeRefLink();
		
		// L4
		loop
		{
			// L5
			node.prev.StoreRef(prev.without_delete_mark());
			
			// L6
			node.next.StoreRef(next.without_delete_mark());
			
			// L7
			if prev.next.CASRef(next.without_delete_mark(), node.without_delete_mark())
			{
				break
			}
			
			// L8
			next.ReleaseRef();
			
			// L9
			next = prev.next.DeRefLink();
			
			// L10
			Back_Off()
		}
		
		// L11
		Self::PushEnd(node, next)
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn PushRight(&self, value: NonNull<T>)
	{
		// R1
		let node = Node::CreateNode(value);
		
		// R2
		let next = self.tail().DeRefLink();
		
		// R3
		let mut prev = next.prev.DeRefLink();
		
		// R4
		loop
		{
			// R5
			node.prev.StoreRef(prev.without_delete_mark());
			
			// R6
			node.next.StoreRef(next.without_delete_mark());
			
			// R7
			if prev.next.CASRef(next.without_delete_mark(), node.without_delete_mark())
			{
				break
			}
			
			// R8
			prev = prev.CorrectPrev(next);
			
			// R9
			Back_Off()
		}
		
		// R10
		Self::PushEnd(node, next)
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn PopLeft(&self) -> Option<NonNull<T>>
	{
		// PL1
		let mut prev = self.head().DeRefLink();
		
		// PL2
		let mut node;
		let mut next;
		let value;
		loop
		{
			// PL3
			node = prev.next.DeRefLink();
		
			// PL4
			if node == self.tail()
			{
				// PL5
				node.ReleaseRef2(prev);
		
				// PL6
				return None
			}
			
			// PL7
			next = node.next.DeRefLink();
		
			// PL8
			if next.d_is_true()
			{
				// PL9
				node.prev.SetMark();
		
				// PL10
				prev.next.CASRef(node, next.p().without_delete_mark());
		
				// PL11
				next.p().ReleaseRef2(node);
		
				// PL12
				continue
			}
			
			// PL13
			if node.next.CASRef(next, next.p().with_delete_mark())
			{
				// PL14
				prev = prev.CorrectPrev(next);
		
				// PL15
				prev.ReleaseRef();
		
				// PL16
				value = node.move_value();
		
				// PL17
				break
			}
			
			// PL18
			next.ReleaseRef2(node);
		
			// PL19
			Back_Off()
		}
		
		// PL20
		next.ReleaseRef2(node);
		
		// PL21
		value
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn PopRight(&self) -> Option<NonNull<T>>
	{
		let value;
		
		// PR1
		let next = self.tail().DeRefLink();
		
		// PR2
		let mut node = next.prev.DeRefLink();
		
		// PR3
		loop
		{
			// PR4
			if node.next != next.without_delete_mark()
			{
				// PR5
				node = node.CorrectPrev(next);
		
				// PR6
				continue
			}
			
			// PR7
			if node == self.head()
			{
				// PR8
				node.ReleaseRef2(next);
		
				// PR10
				return None
			}
			
			// PR11
			if node.next.CASRef(next.without_delete_mark(), next.with_delete_mark())
			{
				// PR13
				let prev = node.prev.DeRefLink();
		
				// PR14
				let prev = prev.CorrectPrev(next);
		
				// PR15
				prev.ReleaseRef2(next);
		
				// There is no PR16 in the algorithm
				
				// PR17
				value = node.move_value();
		
				// PR18
				break
			}
			
			// PR19
			Back_Off()
		}
		
		// PR21
		node.ReleaseRef();
		
		// PR22
		value
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn PushEnd(mut node: TaggedPointerToNode<T>, next: TaggedPointerToNode<T>)
	{
		// P1
		loop
		{
			// P2
			let link1 = next.prev;
			
			// P3
			if link1.d_is_true() || node.next != next.without_delete_mark()
			{
				break
			}
			
			// P4
			if next.prev.CASRef(link1, node.without_delete_mark())
			{
				// P5
				if node.prev.d_is_true()
				{
					// P6
					node = node.CorrectPrev(next);
				}
				
				// P7
				break
			}
			
			// P8
			Back_Off()
		}
		
		// P9
		next.ReleaseRef2(node)
	}
	
	/// The data structure is given an orientation by denoting the head side as being left and the tail side as being right, and we can consequently use this orientation to relate nodes as being to the left or right of each other.
	#[inline(always)]
	fn head(&self) -> TaggedPointerToNode<T>
	{
		TaggedPointerToNode::new(&self.dummy_head_node as *const Node<T> as usize)
	}
	
	/// The data structure is given an orientation by denoting the head side as being left and the tail side as being right, and we can consequently use this orientation to relate nodes as being to the left or right of each other.
	#[inline(always)]
	fn tail(&self) -> TaggedPointerToNode<T>
	{
		TaggedPointerToNode::new(&self.dummy_tail_node as *const Node<T> as usize)
	}
}
