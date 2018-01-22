// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct Node<T>
{
	value: Option<Box<T>>,
	prev: Link<T>,
	next: Link<T>,
	
	// Not part of algorithm per-se.
	reference_count: AtomicUsize,
}

impl<T> Drop for Node<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		forget(self.value);
	}
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
	fn empty_node(initial_reference_count: usize) -> Self
	{
		
		Self
		{
			value: None,
			prev: Link::Null,
			next: Link::Null,
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
	fn CreateNode<'a>(value: Box<T>) -> &'a Self
	{
		// CN1
		let mut node = Self::NewNode();
		
		// CN2
		node.as_mut().value = Some(value);
		
		// CN3
		node.as_ref()
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn DeleteNode(&self)
	{
		let node = self;
		
		unimplemented!("External definition required")
	}
	
	// FIXME: Not currently linked to from any algorithm code...
	/// Callback procedure for memory management.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn TerminateNode(&self)
	{
		let node = self;
		
		// TN1
		node.prev.StoreRef(Link::Null);
		
		// TN2
		node.next.StoreRef(Link::Null)
	}
	
	// FIXME: Not currently linked to from any algorithm code...
	/// Callback procedure for memory management.
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
			node.prev.CASRef((prev.p(), true), (prev2.p(), true));
			
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
			node.next.CASRef((next.p(), true), (next2.p(), true));
			
			// CU12
			next2.ReleaseRef2(next);
		}
		
		// CU13
		prev.ReleaseRef2(next)
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	fn ReleaseRef(&self)
	{
		let node = self;
		
		unimplemented!("External definition required")
	}
	
	// Problem: some parts of the algorithm call `ReleaseRef()` with 2 (two) arguments! WTF?
	// Hence this overloaded variant, `ReleaseRef2()`.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn ReleaseRef2(&self, what_is_this: &Self)
	{
		// Assumption about ReleaseRef2
		self.ReleaseRef();
		what_is_this.ReleaseRef();
	}
	
	//noinspection SpellCheckingInspection
	#[allow(non_snake_case)]
	#[inline(always)]
	fn CorrectPrev(&self, node: &Self) -> &Node<T>
	{
		let mut prev = self;
		
		// CP1
		let mut lastlink: *const Self = null();
		
		// CP2
		loop
		{
			// CP3
			let link1 = node.prev;
			if link1.d_is_true()
			{
				break
			}
			
			// CP4
			let mut prev2 = prev.next.DeRefLink();
		
			// CP5
			if prev2.d_is_true()
			{
				// CP6
				if lastlink.is_not_null()
				{
					let lastlink_ref = unsafe { &* lastlink };
					
					// CP7
					prev.prev.SetMark();
				
					// CP8
					lastlink_ref.next.CASRef(prev, Link::new_without_marks(prev2.p()));
					
					// CP9
					prev2.p().ReleaseRef2(prev);
		
					// CP10
					prev = lastlink_ref;
					lastlink = null();
					
					// CP11
					continue
				}
				
				// CP12
				prev2.p().ReleaseRef();
	
				// CP13
				prev2 = prev.prev.DeRefLink();
				
				// CP14
				prev.ReleaseRef();
				prev = prev2;
				
				// CP15
				continue
			}
			
			// CP16
			if prev2 != node
			{
				// CP17
				if lastlink.is_not_null()
				{
					let lastlink_ref = unsafe { &* lastlink };
					lastlink_ref.ReleaseRef()
				}
				
				// CP18
				lastlink = prev as *const Node<T>;
				
				// CP19
				prev = prev2;
				
				// CP20
				continue
			}
			
			// CP21
			prev2.ReleaseRef();
		
			// CP22
			if node.prev.CASRef(link1, Link::new_without_marks(prev))
			{
				// CP23
				if prev.prev.d_is_true()
				{
					continue
				}
		
				// CP24
				break
			}
			
			// CP25
			Back_Off()
		}
		
		// CP26
		if lastlink.is_not_null()
		{
			let lastlink_ref = unsafe { &* lastlink };
			lastlink_ref.ReleaseRef()
		}
		
		// CP27
		prev
	}
}
