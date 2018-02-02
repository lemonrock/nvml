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
	head: AtomicLink<T>,
	tail: AtomicLink<T>,
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
		let this = unsafe
		{
			let mut this: Self = uninitialized();
			write(&mut this.dummy_head_node, Node::head_or_tail_dummy_node());
			write(&mut this.dummy_tail_node, Node::head_or_tail_dummy_node());
			write(&mut this.head, AtomicLink::from_raw_pointer(&mut this.dummy_head_node));
			write(&mut this.tail, AtomicLink::from_raw_pointer(&mut this.dummy_tail_node));
			
			// head.next => tail
			// tail.prev => head
			write(&mut this.dummy_head_node.next, AtomicLink::from_raw_pointer(&mut this.dummy_tail_node));
			write(&mut this.dummy_tail_node.previous, AtomicLink::from_raw_pointer(&mut this.dummy_head_node));
			
			// head.prev => null
			// tail.next => null
			write(&mut this.dummy_head_node.previous, AtomicLink::Null);
			write(&mut this.dummy_tail_node.next, AtomicLink::Null);
			
			this
		};
		
		fence(Release);
		
		this
	}
}

impl<T> LockFreeDoublyLinkedListAndDeque<T>
{
	/// Creates a new cursor starting from the head.
	/// A `Cursor` is similar to an iterator but is more capable.
	#[inline(always)]
	pub fn cursor_from_head<'a>(&'a self) -> Cursor<'a, T>
	{
		Cursor::new(self, self.head())
	}

	/// Creates a new cursor starting from the tail.
	/// A `Cursor` is similar to an iterator but is more capable.
	#[inline(always)]
	pub fn cursor_from_tail<'a>(&'a self) -> Cursor<'a, T>
	{
		Cursor::new(self, self.tail())
	}
	
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
		let node: DereferencedLink<T> = Node::CreateNode(value);
		
		// L2
		let prev: DereferencedLink<T> = self.head().DeRefLink();
		
		// L3
		let mut next: DereferencedLink<T> = prev.next_link_atomic().DeRefLink();
		
		// L4
		loop
		{
			// L5
			node.previous_link_atomic().StoreRef(prev.without_delete_tag_link_stack());
			
			// L6
			node.next_link_atomic().StoreRef(next.without_delete_tag_link_stack());
			
			// L7
			if prev.next_link_atomic().CASRef(next.without_delete_tag_link_stack(), node.without_delete_tag_link_stack())
			{
				break
			}
			
			// L8
			next.ReleaseRef();
			
			// L9
			next = prev.next_link_atomic().DeRefLink();
			
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
		let node: DereferencedLink<T> = Node::CreateNode(value);
		
		// R2
		let next: DereferencedLink<T> = self.tail().DeRefLink();
		
		// R3
		let mut prev: DereferencedLink<T> = next.previous_link_atomic().DeRefLink();
		
		// R4
		loop
		{
			// R5
			node.previous_link_atomic().StoreRef(prev.without_delete_tag_link_stack());
			
			// R6
			node.next_link_atomic().StoreRef(next.without_delete_tag_link_stack());
			
			// R7
			if prev.next_link_atomic().CASRef(next.without_delete_tag_link_stack(), node.without_delete_tag_link_stack())
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
		let mut prev: DereferencedLink<T> = self.head().DeRefLink();
		
		// PL2
		let mut node: DereferencedLink<T>;
		let mut next: DereferencedLink<T>;
		let value;
		loop
		{
			// PL3
			node = prev.next_link_atomic().DeRefLink();
		
			// PL4
			if node.to_link_stack() == self.tail().without_lock_dereference_tag_but_with_delete_tag_if_set()
			{
				// PL5
				node.ReleaseRef();
				prev.ReleaseRef();
		
				// PL6
				return None
			}
			
			// PL7
			next = node.next_link_atomic().DeRefLink();
		
			// PL8
			if next.has_delete_tag()
			{
				// PL9
				node.previous_link_atomic().SetMark();
		
				// PL10
				prev.next_link_atomic().CASRef(node.to_link_stack(), next.without_delete_tag_link_stack());
		
				// PL11
				next.ReleaseRef();
				node.ReleaseRef();
		
				// PL12
				continue
			}
			
			// PL13
			if node.next_link_atomic().CASRef(next.to_link_stack(), next.with_delete_tag_link_stack())
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
			next.ReleaseRef();
			node.ReleaseRef();
		
			// PL19
			Back_Off()
		}
		
		// PL20
		next.ReleaseRef();
		node.ReleaseRef();
		
		// PL21
		value
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn PopRight(&self) -> Option<NonNull<T>>
	{
		let value;
		
		// PR1
		let next: DereferencedLink<T> = self.tail().DeRefLink();
		
		// PR2
		let mut node: DereferencedLink<T> = next.previous_link_atomic().DeRefLink();
		
		// PR3
		loop
		{
			// PR4
			if node.next_link_stack() != next.without_delete_tag_link_stack()
			{
				// PR5
				node = node.CorrectPrev(next);
		
				// PR6
				continue
			}
			
			// PR7
			if node.to_link_stack() == self.head().without_lock_dereference_tag_but_with_delete_tag_if_set()
			{
				// PR8
				node.ReleaseRef();
				next.ReleaseRef();
		
				// PR10
				return None
			}
			
			// PR11
			if node.next_link_atomic().CASRef(next.without_delete_tag_link_stack(), next.with_delete_tag_link_stack())
			{
				// PR13
				let prev = node.previous_link_atomic().DeRefLink();
		
				// PR14
				let prev = prev.CorrectPrev(next);
		
				// PR15
				prev.ReleaseRef();
				next.ReleaseRef();
		
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
	fn PushEnd(mut node: DereferencedLink<T>, next: DereferencedLink<T>)
	{
		// P1
		loop
		{
			// P2
			let link1: StackLink<T> = next.previous_link_stack();
			
			// P3
			if link1.has_delete_tag() || node.next_link_stack() != next.without_delete_tag_link_stack()
			{
				break
			}
			
			// P4
			if next.previous_link_atomic().CASRef(link1, node.without_delete_tag_link_stack())
			{
				// P5
				if node.previous_link_stack().has_delete_tag()
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
		next.ReleaseRef();
		node.ReleaseRef()
	}
	
	/// The data structure is given an orientation by denoting the head side as being left and the tail side as being right, and we can consequently use this orientation to relate nodes as being to the left or right of each other.
	#[inline(always)]
	fn head(&self) -> &AtomicLink<T>
	{
		&self.head
	}
	
	/// The data structure is given an orientation by denoting the head side as being left and the tail side as being right, and we can consequently use this orientation to relate nodes as being to the left or right of each other.
	#[inline(always)]
	fn tail(&self) -> &AtomicLink<T>
	{
		&self.tail
	}
}
