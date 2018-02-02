// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A 'cursor' over the doubly-linked list.
/// This cursor potentially isn't safe to use if another thread is calling `PopRight()` or `PopLeft()`.
/// The remaining necessary functionality for initializing the cursor positions like `First()` and `Last()` can be trivially derived by using the dummy nodes.
/// If an `Update()` functionality is necessary, this could easily be achieved by extending the value field of the node data structure with a deletion mark, and throughout the whole algorithm interpret the deletion state of the whole node using this mark when semantically necessary, in combination with the deletion marks on the next and prev pointers.
pub struct Cursor<'a, T: 'a>
{
	list: &'a LockFreeDoublyLinkedListAndDeque<T>,
	position: DereferencedLink<T>,
}

impl<'a, T: 'a> Drop for Cursor<'a, T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.position.ReleaseRef()
	}
}

impl<'a, T: 'a> Cursor<'a, T>
{
	/// This function assumes the `current_node` is **NOT** dereference locked on entry.
	/// This function will dereference lock it.
	#[inline(always)]
	fn new(list: &'a LockFreeDoublyLinkedListAndDeque<T>, current_node: &AtomicLink<T>) -> Self
	{
		Self
		{
			list,
			position: current_node.DeRefLink(),
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
	/// Returns a copy of the value.
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
	/// Returns a copy of the value.
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
	
	/// Tries to delete the next value after advancing the cursor.
	/// Returns a value.
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
	
	/// Tries to delete the previous value after retarding the cursor.
	/// Returns a value.
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
	
	// Note: This function will update the cursor with the `next` node which will be "dereference locked".
	// Note: This function assumes the `current_node` is "dereference locked" on entry.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn Next(cursor: &mut DereferencedLink<T>, tail: &AtomicLink<T>) -> bool
	{
		// NT1
		loop
		{
			// Only this thread can modify the cursor.
			// Changed to make logic clearer. Since we do not modify the cursor until NT11, this is a safe change.
			let current_node: DereferencedLink<T> = *cursor;
			
			// NT2
			// Was: if *cursor == tail
			// Changed to make logic clearer. Since we do not modify the cursor until NT11, this is a safe change.
			if current_node.to_link_stack() == tail.without_lock_dereference_tag_but_with_delete_tag_if_set()
			{
				return false
			}
			
			// NT3
			// Was: (*cursor).next.DeRefLink()
			// Changed to make logic clearer. Since we do not modify the cursor until NT11, this is a safe change.
			let next: DereferencedLink<T> = current_node.next_link_atomic().DeRefLink();
			
			// NT4
			let has_delete_tag = next.next_link_stack().has_delete_tag();
			
			// NT5
			// Was: if d && (*cursor).next != next.with_delete_tag()
			// Changed to make logic clearer. Since we do not modify the cursor until NT11, this is a safe change.
			if has_delete_tag && current_node.next_link_stack() != next.with_delete_tag_link_stack()
			{
				// NT6
				next.previous_link_atomic().SetMark();
				
				// NT7
				// Was: (*cursor).next.CASRef(next, next.p());
				// Changed to make logic clearer. Since we do not modify the cursor until NT11, this is a safe change.
				current_node.next_link_atomic().CASRef(next.to_link_stack(), next.next_link_stack().without_delete_tag());
				
				// NT8
				next.ReleaseRef();
				
				// NT9
				continue
			}
			
			// NT10
			// Was: (*cursor).ReleaseRef();
			// Changed to make logic clearer. Since we do not modify the cursor until NT11, this is a safe change.
			current_node.ReleaseRef();
			
			// NT11
			*cursor = next;
			
			// NT12
			if !has_delete_tag && next.to_link_stack() != tail.without_lock_dereference_tag_but_with_delete_tag_if_set()
			{
				return true
			}
		}
	}
	
	// Note: This function will update the cursor with the `prev` node which will be "dereference locked".
	// Note: This function assumes the `current_node` is "dereference locked" on entry.
	#[allow(non_snake_case)]
	fn Prev(cursor: &mut DereferencedLink<T>, head: &AtomicLink<T>, tail: &AtomicLink<T>) -> bool
	{
		// PV1
		loop
		{
			// Only this thread can modify the cursor.
			// Changed to make logic clearer. Since we do not modify the cursor until PV6 or PV10 this is a safe change.
			let current_node: DereferencedLink<T> = *cursor;
			
			// PV2
			// Was: if *cursor == head
			// Changed to make logic clearer. Since we do not modify the cursor until PV6 or PV10 this is a safe change.
			if current_node.to_link_stack() == head.without_lock_dereference_tag_but_with_delete_tag_if_set()
			{
				return false
			}
		
			// PV3
			// Was: let mut prev = (*cursor).prev.DeRefLink();
			// Changed to make logic clearer. Since we do not modify the cursor until PV6 or PV10 this is a safe change.
			let mut prev: DereferencedLink<T> = current_node.previous_link_atomic().DeRefLink();
		
			// PV4
			// Was: if prev.next == (*cursor).without_delete_tag() && (*cursor).next.d_is_false()
			// Changed to make logic clearer. Since we do not modify the cursor until PV6 or PV10 this is a safe change.
			if prev.next_link_stack() == current_node.without_delete_tag_link_stack() && current_node.next_link_stack().does_not_have_delete_tag()
			{
				// PV5
				// Was: (*cursor).ReleaseRef();
				// Changed to make logic clearer. Since we do not modify the cursor until PV6 or PV10 this is a safe change.
				current_node.ReleaseRef();
		
				// PV6
				*cursor = prev;
		
				// PV7
				if prev.to_link_stack() != head.without_lock_dereference_tag_but_with_delete_tag_if_set()
				{
					return true
				}
			}
			// PV8
			// Was: else if (*cursor).next.d_is_true()
			// Changed to make logic clearer. Since we do not modify the cursor except in PV6 or PV10 this is a safe change.
			else if current_node.next_link_stack().has_delete_tag()
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
				// Was: prev = prev.CorrectPrev(*cursor);
				// Changed to make logic clearer. Since we do not modify the cursor except in PV6 or PV10 this is a safe change.
				prev = prev.CorrectPrev(current_node);
		
				// PV13
				prev.ReleaseRef()
			}
		}
	}
	
	// Note: This function assumes (as far as one can tell) that the `current_node` is "dereference locked" on entry.
	// This makes the read of the value safe.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn Read(cursor: &mut DereferencedLink<T>, head: &AtomicLink<T>, tail: &AtomicLink<T>) -> Option<NonNull<T>>
	{
		// Only this thread can modify the cursor.
		// Changed to make logic clearer. Since we never modify the cursor, this is a safe change.
		let current_node: DereferencedLink<T> = *cursor;
		
		// RD1
		// Was: *cursor == head || *cursor == tail
		// Changed to make logic clearer. Since we never modify the cursor, this is a safe change.
		if current_node.to_link_stack() == head.without_lock_dereference_tag_but_with_delete_tag_if_set() || current_node.to_link_stack() == tail.without_lock_dereference_tag_but_with_delete_tag_if_set()
		{
			return None
		}
		
		// RD2
		// Was: let value = (*cursor).ref_value();
		// Changed to make logic clearer. Since we never modify the cursor, this is a safe change.
		let value = current_node.ref_value();
		
		// RD3
		// Was: if (*cursor).next.d_is_true()
		// Changed to make logic clearer. Since we never modify the cursor, this is a safe change.
		if current_node.next_link_stack().has_delete_tag()
		{
			return None
		}
		
		// RD4
		value
	}
	
	// Note: This function assumes the `current_node` is "dereference locked" on entry.
	#[allow(non_snake_case)]
	fn InsertBefore(cursor: &mut DereferencedLink<T>, value: NonNull<T>, head: &AtomicLink<T>, tail: &AtomicLink<T>)
	{
		// Only this thread can modify the cursor.
		// Changed to make logic clearer. We do not modify the cursor until either IB6 or IB14.
		let mut current_node: DereferencedLink<T> = *cursor;
		
		let mut next: DereferencedLink<T>;
		
		// IB1
		// Was: *cursor == head
		if current_node.to_link_stack() == head.without_lock_dereference_tag_but_with_delete_tag_if_set()
		{
			return Self::InsertAfter(cursor, value, head, tail)
		}
	
		// IB2
		let node = Node::CreateNode(value);
		
		// IB3
		// Was: let mut prev = (*cursor).prev.DeRefLink();
		// Changed to make logic clearer.
		let mut prev: DereferencedLink<T> = current_node.previous_link_atomic().DeRefLink();
		
		// IB4
		loop
		{
			// IB5
			// Was: while (*cursor).next.d_is_true()
			// Changed to make logic clearer.
			while current_node.next_link_stack().has_delete_tag()
			{
				// IB6
				Self::Next(cursor, tail);
				current_node = *cursor;
		
				// IB7
				// Was: prev = prev.CorrectPrev(*cursor);
				// Changed to make logic clearer.
				prev = prev.CorrectPrev(current_node);
			}
			
			// IB8
			// (cursor can have changed inside loop at IB5 in call to IB6)
			next = *cursor;
		
			// IB9
			node.previous_link_atomic().StoreRef(prev.without_delete_tag_link_stack());
		
			// IB10
			node.next_link_atomic().StoreRef(next.without_delete_tag_link_stack());
		
			// IB11
			// Was: if prev.next.CASRef((*cursor).without_delete_tag(), node.without_delete_tag())
			// Changed to make logic clearer. As only this thread can modify the `cursor` this change is correct.
			if prev.next_link_atomic().CASRef(current_node.without_delete_tag_link_stack(), node.without_delete_tag_link_stack())
			{
				break
			}
		
			// IB12
			// prev = prev.CorrectPrev(*cursor);
			// Changed to make logic clearer. As only this thread can modify the `cursor` this change is correct.
			prev = prev.CorrectPrev(current_node);
		
			// IB13
			Back_Off()
		}
		
		// IB14
		*cursor = node;
		
		// IB15
		prev = prev.CorrectPrev(next);
		
		// IB16
		prev.ReleaseRef();
		next.ReleaseRef()
	}
	
	// Note: This function assumes the `current_node` is "dereference locked" on entry.
	#[allow(non_snake_case)]
	fn InsertAfter(cursor: &mut DereferencedLink<T>, value: NonNull<T>, head: &AtomicLink<T>, tail: &AtomicLink<T>)
	{
		// Only this thread can modify the cursor.
		// Changed to make logic clearer.
		let current_node: DereferencedLink<T> = *cursor;
		
		// IA1
		if current_node.to_link_stack() == tail.without_lock_dereference_tag_but_with_delete_tag_if_set()
		{
			return Self::InsertBefore(cursor, value, head, tail)
		}
		
		// IA2
		let node: DereferencedLink<T> = Node::CreateNode(value);
		
		// IA3
		let mut prev: DereferencedLink<T> = current_node;
		
		// IA4
		loop
		{
			// IA5
			let next = prev.next_link_atomic().DeRefLink();
		
			// IA6
			node.previous_link_atomic().StoreRef(prev.without_delete_tag_link_stack());
		
			// IA7
			node.next_link_atomic().StoreRef(next.without_delete_tag_link_stack());
		
			// IA8
			// NOTE: (*cursor) is not the same as prev in any second or subsequent loop because of IA14.
			if (*cursor).next_link_atomic().CASRef(next.without_delete_tag_link_stack(), node.without_delete_tag_link_stack())
			{
				break
			}
		
			// IA9
			next.ReleaseRef();
		
			// IA10
			if prev.next_link_stack().has_delete_tag()
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
			prev.ReleaseRef();
			next.ReleaseRef()
		}
	}

	#[allow(non_snake_case)]
	#[inline(always)]
	fn Delete(cursor: &mut DereferencedLink<T>, head: &AtomicLink<T>, tail: &AtomicLink<T>) -> Option<NonNull<T>>
	{
		// D1
		let node = *cursor;
		
		// D2
		if node.to_link_stack() == head.without_lock_dereference_tag_but_with_delete_tag_if_set() || node.to_link_stack() == tail.without_lock_dereference_tag_but_with_delete_tag_if_set()
		{
			return None
		}
		
		// D3
		loop
		{
			// D4
			// Was: let next = (*cursor).next.DeRefLink();
			// Changed to make logic clearer. As only this thread can modify the `cursor` this change is correct.
			let next = node.next_link_atomic().DeRefLink();
			
			// D5
			if next.has_delete_tag()
			{
				// D6
				next.ReleaseRef();
		
				// D7
				return None
			}
			
			// D8
			// NOTE: CAS, not CASRef
			if node.next_link_atomic().CAS(next.to_link_stack(), next.with_delete_tag_link_stack())
			{
				let mut prev;
				
				// D9
				loop
				{
					// D10
					prev = node.previous_link_atomic().DeRefLink();
		
					// D11
					// NOTE: CAS, not CASRef
					if prev.has_delete_tag() || node.previous_link_atomic().CAS(prev.to_link_stack(), prev.with_delete_tag_link_stack())
					{
						break
					}
				}
				
				// D12
				prev = DereferencedLink(prev.without_delete_tag_link_stack()).CorrectPrev(next);
			
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
	fn head(&self) -> &'a AtomicLink<T>
	{
		self.list.head()
	}
	
	#[inline(always)]
	fn tail(&self) -> &'a AtomicLink<T>
	{
		self.list.tail()
	}
	
	#[inline(always)]
	fn cursor(&mut self) -> &mut DereferencedLink<T>
	{
		&mut self.position
	}
}
