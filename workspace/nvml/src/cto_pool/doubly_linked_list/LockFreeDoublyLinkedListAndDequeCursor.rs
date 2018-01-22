// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A 'cursor' over the doubly-linked list.
/// This cursor potentially isn't safe to use if another thread is calling `PopRight()` or `PopLeft()`. It isn't clear how to free a Read()'s reference count.
/// The remaining necessary functionality for initializing the cursor positions like `First()` and `Last()` can be trivially derived by using the dummy nodes.
/// If an `Update()` functionality is necessary, this could easily be achieved by extending the value field of the node data structure with a deletion mark, and throughout the whole algorithm interpret the deletion state of the whole node using this mark when semantically necessary, in combination with the deletion marks on the next and prev pointers.
pub struct LockFreeDoublyLinkedListAndDequeCursor<'a, T: 'a>
{
	list: &'a LockFreeDoublyLinkedListAndDeque<T>,
	position: TaggedPointerToNode<T>,
}

impl<'a, T: 'a> LockFreeDoublyLinkedListAndDequeCursor<'a, T>
{
	/// Creates a new cursor starting from the head.
	#[inline(always)]
	pub fn from_head(list: &'a LockFreeDoublyLinkedListAndDeque<T>) -> Self
	{
		Self
		{
			list,
			position: list.head(),
		}
	}
	
	/// Creates a new cursor starting from the tail.
	#[inline(always)]
	pub fn from_tail(list: &'a LockFreeDoublyLinkedListAndDeque<T>) -> Self
	{
		Self
		{
			list,
			position: list.tail(),
		}
	}
	
	/// Moves the cursor from left to right, ie moves it closer to the tail.
	/// Returns false if the cursor is at the tail, true otherwise.
	#[inline(always)]
	pub fn forward(&mut self) -> bool
	{
		let tail = self.tail();
		let cursor = self.cursor();
		Self::Next(cursor, tail)
	}
	
	/// Moves the cursor from right to left, ie moves it closer to the head.
	/// Returns false if the cursor is at the head, true otherwise.
	#[inline(always)]
	pub fn backward(&mut self) -> bool
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		Self::Prev(cursor, head, tail)
	}
	
	/// Tries to read the next value after advancing the cursor.
	/// Use carefully; if another thread also tries to read or delete or pop, then multiple copies of the result `NonNull<T>` will exist.
	/// Returns None if there are no more values, or if created with `Self::from_tail()`.
	#[inline(always)]
	pub fn forward_and_read_next_value(&'a mut self) -> Option<NonNull<T>>
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		
		if Self::Next(cursor, tail)
		{
			Self::Read(cursor, head, tail)
		}
		else
		{
			None
		}
	}
	
	/// Tries to read the previous value after retarding the cursor.
	/// Use carefully; if another thread also tries to read or delete or pop, then multiple copies of the result `NonNull<T>` will exist.
	/// Returns None if there are no more values, or if created with `Self::from_head()`.
	#[inline(always)]
	pub fn backward_and_read_previous_value(&'a mut self) -> Option<NonNull<T>>
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		
		if Self::Prev(cursor, head, tail)
		{
			Self::Read(cursor, head, tail)
		}
		else
		{
			None
		}
	}
	
	/// Tries to read the next value after advancing the cursor.
	/// Returns a reference.
	/// Returns None if there are no more values, or if created with `Self::from_tail()`.
	#[inline(always)]
	pub fn forward_and_delete_next_value(&mut self) -> Option<NonNull<T>>
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		
		if Self::Next(cursor, tail)
		{
			Self::Delete(cursor, head, tail)
		}
		else
		{
			None
		}
	}
	
	/// Tries to read the previous value after retarding the cursor.
	/// Returns a reference.
	/// Returns None if there are no more values, or if created with `Self::from_head()`.
	#[inline(always)]
	pub fn backward_and_delete_previous_value(&mut self) -> Option<NonNull<T>>
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		
		if Self::Prev(cursor, head, tail)
		{
			Self::Delete(cursor, head, tail)
		}
		else
		{
			None
		}
	}
	
	/// Insert value before cursor's current position.
	/// If current position is the head, then inserts immediately after the head, ie equivalent to `insert_value_at_head()`.
	/// The cursor will be positioned 'on' the inserted value.
	#[inline(always)]
	pub fn insert_value_before_current_position(&mut self, value: NonNull<T>)
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		Self::InsertBefore(cursor, value, head, tail)
	}
	
	/// Insert value after cursor's current position.
	/// If current position is the tail, then inserts immediately before the tail, ie equivalent to `insert_value_at_tail()`.
	/// The cursor will be positioned 'on' the inserted value.
	#[inline(always)]
	pub fn insert_value_after_current_position(&mut self, value: NonNull<T>)
	{
		let head = self.head();
		let tail = self.tail();
		let cursor = self.cursor();
		Self::InsertAfter(cursor, value, head, tail)
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn Next(cursor: &mut TaggedPointerToNode<T>, tail: TaggedPointerToNode<T>) -> bool
	{
		// NT1
		loop
		{
			// NT2
			if *cursor == tail
			{
				return false
			}
			
			// NT3
			let next = (*cursor).next.DeRefLink();
			
			// NT4
			let d = next.next.d();
			
			// NT5
			if d && (*cursor).next != next.with_delete_mark()
			{
				// NT6
				next.prev.SetMark();
				
				// NT7
				(*cursor).next.CASRef(next, next.p());
				
				// NT8
				next.ReleaseRef();
				
				// NT9
				continue
			}
			
			// NT10
			(*cursor).ReleaseRef();
			
			// NT11
			*cursor = next;
			
			// NT12
			if !d && next != tail
			{
				return true
			}
		}
	}
	
	#[allow(non_snake_case)]
	fn Prev(cursor: &mut TaggedPointerToNode<T>, head: TaggedPointerToNode<T>, tail: TaggedPointerToNode<T>) -> bool
	{
		// PV1
		loop
		{
			// PV2
			if *cursor == head
			{
				return false
			}
		
			// PV3
			let mut prev = (*cursor).prev.DeRefLink();
		
			// PV4
			if prev.next == (*cursor).without_delete_mark() && (*cursor).next.d_is_false()
			{
				// PV5
				(*cursor).ReleaseRef();
		
				// PV6
				*cursor = prev;
		
				// PV7
				if prev != head
				{
					return true
				}
			}
			// PV8
			else if (*cursor).next.d_is_true()
			{
				// PV9
				prev.ReleaseRef();
		
				// PV10
				Self::Next(cursor, tail);
			}
			// PV11
			else
			{
				// PV12
				prev = prev.CorrectPrev(*cursor);
		
				// PV13
				prev.ReleaseRef()
			}
		}
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn Read(cursor: &'a mut TaggedPointerToNode<T>, head: TaggedPointerToNode<T>, tail: TaggedPointerToNode<T>) -> Option<NonNull<T>>
	{
		// RD1
		if *cursor == head || *cursor == tail
		{
			return None
		}
		
		// RD2
		let value = (*cursor).clone_value();
		
		// RD3
		if (*cursor).next.d_is_true()
		{
			return None
		}
		
		// RD4
		value
	}

	#[allow(non_snake_case)]
	fn InsertBefore(cursor: &mut TaggedPointerToNode<T>, value: NonNull<T>, head: TaggedPointerToNode<T>, tail: TaggedPointerToNode<T>)
	{
		// IB1
		if *cursor == head
		{
			return Self::InsertAfter(cursor, value, head, tail)
		}
	
		// IB2
		let node = Node::CreateNode(value);
		
		// IB3
		let mut prev = (*cursor).prev.DeRefLink();
		
		// IB4
		let mut next;
		loop
		{
			// IB5
			while (*cursor).next.d_is_true()
			{
				// IB6
				Self::Next(cursor, tail);
		
				// IB7
				prev = prev.CorrectPrev(*cursor);
			}
			
			// IB8
			next = *cursor;
		
			// IB9
			node.prev.StoreRef(prev.without_delete_mark());
		
			// IB10
			node.next.StoreRef(next.without_delete_mark());
		
			// IB11
			if prev.next.CASRef((*cursor).without_delete_mark(), node.without_delete_mark())
			{
				break
			}
		
			// IB12
			prev = prev.CorrectPrev(*cursor);
		
			// IB13
			Back_Off()
		}
		
		// IB14
		*cursor = node;
		
		// IB15
		prev = prev.CorrectPrev(next);
		
		// IB16
		prev.ReleaseRef2(next)
	}
	
	#[allow(non_snake_case)]
	fn InsertAfter(cursor: &mut TaggedPointerToNode<T>, value: NonNull<T>, head: TaggedPointerToNode<T>, tail: TaggedPointerToNode<T>)
	{
		// IA1
		if *cursor == tail
		{
			return Self::InsertBefore(cursor, value, head, tail)
		}
		
		// IA2
		let node = Node::CreateNode(value);
		
		// IA3
		let mut prev = *cursor;
		
		// IA4
		loop
		{
			// IA5
			let next = prev.next.DeRefLink();
		
			// IA6
			node.prev.StoreRef(prev.without_delete_mark());
		
			// IA7
			node.next.StoreRef(next.without_delete_mark());
		
			// IA8
			if (*cursor).next.CASRef(next.without_delete_mark(), node.without_delete_mark())
			{
				break
			}
		
			// IA9
			next.ReleaseRef();
		
			// IA10
			if prev.next.d_is_true()
			{
				// IA11
				node.ReleaseRef();
				node.DeleteNode();
				
				// IA12
				return Self::InsertBefore(cursor, value, head, tail)
			}
			
			// IA13
			Back_Off();
			
			// IA14
			*cursor = node;
		
			// IA15
			prev = prev.CorrectPrev(next);
		
			// IA16
			prev.ReleaseRef2(next)
		}
	}

	#[allow(non_snake_case)]
	#[inline(always)]
	fn Delete(cursor: &mut TaggedPointerToNode<T>, head: TaggedPointerToNode<T>, tail: TaggedPointerToNode<T>) -> Option<NonNull<T>>
	{
		// D1
		let mut node = *cursor;
		
		// D2
		if node == head || node == tail
		{
			return None
		}
		
		// D3
		loop
		{
			// D4
			let next = (*cursor).next.DeRefLink();
			
			// D5
			if next.d_is_true()
			{
				// D6
				next.ReleaseRef();
		
				// D7
				return None
			}
			
			// D8
			// NOTE: CAS, not CASRef
			if node.next.CAS(next, next.p().with_delete_mark())
			{
				let mut prev;
				
				// D9
				loop
				{
					// D10
					prev = node.prev.DeRefLink();
		
					// D11
					// NOTE: CAS, not CASRef
					if prev.d_is_true() || node.prev.CAS(prev, prev.p().with_delete_mark())
					{
						break
					}
				}
				
				// D12
				prev = prev.p().CorrectPrev(next);
			
				// D13
				prev.ReleaseRef();
				next.ReleaseRef();
		
				// D14
				let value = node.move_value();
		
				// D15
				node.ReleaseRef();
				node.DeleteNode();
				
				// D16
				return value
			}
		}
	}
	
	#[inline(always)]
	fn head(&self) -> TaggedPointerToNode<T>
	{
		self.list.head()
	}
	
	#[inline(always)]
	fn tail(&self) -> TaggedPointerToNode<T>
	{
		self.list.tail()
	}
	
	#[inline(always)]
	fn cursor(&mut self) -> &mut TaggedPointerToNode<T>
	{
		&mut self.position
	}
}
