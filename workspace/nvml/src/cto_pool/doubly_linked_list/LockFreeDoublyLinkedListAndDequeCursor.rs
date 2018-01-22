// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// TODO: value is NOT Box<T>
/*

	
	it needs to be reference counted, otherwise fn Read() won't work.
	
	returning &Box<T> (or any other &) won't work well.

	A variant we could create re-uses the list memory from an allocator.§
*/
/// A 'cursor' over the doubly-linked list.
/// This cursor potentially isn't safe to use if another thread is calling `PopRight()` or `PopLeft()`. It isn't clear how to free a Read()'s reference count.
/// The remaining necessary functionality for initializing the cursor positions like `First()` and `Last()` can be trivially derived by using the dummy nodes.
/// If an `Update()` functionality is necessary, this could easily be achieved by extending the value field of the node data structure with a deletion mark, and throughout the whole algorithm interpret the deletion state of the whole node using this mark when semantically necessary, in combination with the deletion marks on the next and prev pointers.
pub struct LockFreeDoublyLinkedListAndDequeCursor<'a, T: 'a>
{
	list: &'a LockFreeDoublyLinkedListAndDeque<T>,
	position: &'a Node<T>,
}

impl<'a, T> LockFreeDoublyLinkedListAndDequeCursor<'a, T>
{
	#[inline(always)]
	fn from_head(list: &'a LockFreeDoublyLinkedListAndDeque<T>) -> Self
	{
		Self
		{
			list,
			position: list.head(),
		}
	}
	
	#[inline(always)]
	fn from_tail(list: &'a LockFreeDoublyLinkedListAndDeque<T>) -> Self
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
		let mut cursor = &mut self.position;
		self.Next(cursor)
	}
	
	/// Moves the cursor from right to left, ie moves it closer to the head.
	/// Returns false if the cursor is at the head, true otherwise.
	#[inline(always)]
	pub fn backward(&mut self) -> bool
	{
		let mut cursor = &mut self.position;
		self.Prev(cursor)
	}
	
	/// Tries to read the next value after advancing the cursor.
	/// Returns a reference.
	/// Returns None if there are no more values, or if created with `Self::from_tail()`.
	#[inline(always)]
	pub fn forward_and_read_next_value(&'a mut self) -> Option<&'a Box<T>>
	{
		let mut cursor = &mut self.position;
		
		if self.Next(cursor)
		{
			self.Read(cursor)
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
	pub fn backward_and_read_previous_value(&'a mut self) -> Option<&'a Box<T>>
	{
		let mut cursor = &mut self.position;
		
		if self.Prev(cursor)
		{
			self.Read(cursor)
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
	pub fn forward_and_delete_next_value(&mut self) -> Option<Box<T>>
	{
		let mut cursor = &mut self.position;
		
		if self.Next(cursor)
		{
			self.Delete(cursor)
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
	pub fn backward_and_delete_previous_value(&mut self) -> Option<Box<T>>
	{
		let mut cursor = &mut self.position;
		
		if self.Prev(cursor)
		{
			self.Delete(cursor)
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
	pub fn insert_value_before_current_position(&mut self, value: Box<T>)
	{
		let mut cursor = &mut self.position;
		self.InsertBefore(cursor, value)
	}
	
	/// Insert value after cursor's current position.
	/// If current position is the tail, then inserts immediately before the tail, ie equivalent to `insert_value_at_tail()`.
	/// The cursor will be positioned 'on' the inserted value.
	#[inline(always)]
	pub fn insert_value_after_current_position(&mut self, value: Box<T>)
	{
		let mut cursor = &mut self.position;
		self.InsertAfter(cursor, value)
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn Next(&self, cursor: &mut &Node<T>) -> bool
	{
		// NT1
		loop
		{
			// NT2
			if *cursor == self.tail()
			{
				return false
			}
			
			// NT3
			let next = (*cursor).DeRefLink().next;
			
			// NT4
			let d = next.next.d();
			
			// NT5
			if d && (*cursor).next != Link::new_with_delete_mark(next)
			{
				// NT6
				next.prev.SetMark();
				
				// NT7
				(*cursor).next.CASRef(next, next.p);
				
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
			if !d && next != self.tail()
			{
				return true
			}
		}
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn Prev(&self, cursor: &mut &Node<T>) -> bool
	{
		// PV1
		loop
		{
			// PV2
			if *cursor == self.head()
			{
				return false
			}
		
			// PV3
			let prev = (*cursor).DeRefLink().prev;
		
			// PV4
			if prev.next == Link::new_without_marks(*cursor) && (*cursor).next.d_is_false()
			{
				// PV5
				(*cursor).ReleaseRef();
		
				// PV6
				*cursor = prev;
		
				// PV7
				if prev != self.head()
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
				self.Next(cursor);
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
	fn Read(&self, cursor: &'a mut &Node<T>) -> Option<&'a Box<T>>
	{
		// RD1
		if *cursor == self.head() || *cursor == self.tail()
		{
			return None
		}
		
		// RD2
		// WAS: let value = (*cursor).value;
		// Changed to pass out a reference.
		let value = (*cursor).value.as_ref();
		
		// RD3
		if (*cursor).next.d_is_true()
		{
			return None
		}
		
		// RD4
		value
	}

	#[allow(non_snake_case)]
	#[inline(always)]
	fn InsertBefore(&self, cursor: &mut &Node<T>, value: Box<T>)
	{
		// IB1
		if *cursor == self.head()
		{
			return self.InsertAfter(cursor, value)
		}
	
		// IB2
		let node = Node::CreateNode(value);
		
		// IB3
		let mut prev = (*cursor).DeRefLink().prev;
		
		// IB4
		let mut next;
		loop
		{
			// IB5
			while (*cursor).next.d_is_true()
			{
				// IB6
				self.Next(cursor);
		
				// IB7
				prev = prev.CorrectPrev(*cursor);
			}
			
			// IB8
			next = *cursor;
		
			// IB9
			node.prev.StoreRef(Link::new_without_marks(prev));
		
			// IB10
			node.next.StoreRef(Link::new_without_marks(next));
		
			// IB11
			if prev.next.CASRef(Link::new_without_marks(*cursor), Link::new_without_marks(node))
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
	#[inline(always)]
	fn InsertAfter(&self, cursor: &mut &Node<T>, value: Box<T>)
	{
		// IA1
		if *cursor == self.tail()
		{
			return self.InsertBefore(cursor, value)
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
			node.prev.StoreRef(Link::new_without_marks(prev));
		
			// IA7
			node.next.StoreRef(Link::new_without_marks(next));
		
			// IA8
			if (*cursor).next.CASRef(Link::new_without_marks(next), Link::new_without_marks(node))
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
				return self.InsertBefore(cursor, value)
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
	fn Delete(&self, cursor: &mut &Node<T>) -> Option<Box<T>>
	{
		// D1
		let node = *cursor;
		
		// D2
		if node == self.head() || node == self.tail()
		{
			return None
		}
		
		// D3
		loop
		{
			// D4
			let next = (*cursor).DeRefLink().next;
			
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
			if node.next.CAS(next, Link::new_with_delete_mark(next.p()))
			{
				let mut prev;
				
				// D9
				loop
				{
					// D10
					prev = node.prev.DeRefLink();
		
					// D11
					// NOTE: CAS, not CASRef
					if prev.d_is_true() || node.prev.CAS(prev, Link::new_with_delete_mark(prev.p()))
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
				let value = node.value;
		
				// D15
				node.ReleaseRef();
				node.DeleteNode();
				
				// D16
				return value
			}
		}
	}
	
	#[inline(always)]
	fn head(&self) -> &Node<T>
	{
		self.list.head()
	}
	
	#[inline(always)]
	fn tail(&self) -> &Node<T>
	{
		self.list.tail()
	}
}
