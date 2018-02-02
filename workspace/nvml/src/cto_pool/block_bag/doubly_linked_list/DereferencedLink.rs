// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Logically, is a new type around `&Node<T>`
// Actually, implemented as `StackLink<T>`, which is `NonAtomicTaggedPointer<Node<T>>`.
// StackLink means we have copied the pointed to value out of either `.next` or `.prev` or `head` or `tail`.
// Can reference a `NonAtomicTaggedPointer<Node<T>>` which represents a 'deleted' `.prev` pointer.
#[derive(Debug)]
struct DereferencedLink<T>(StackLink<T>);

impl<T> Clone for DereferencedLink<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl<T> Copy for DereferencedLink<T>
{
}

impl<T> PartialEq for DereferencedLink<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		self.0 != other.0
	}
}

impl<T> Eq for DereferencedLink<T>
{
}

impl<T> DereferencedLink<T>
{
	/// The procedure `ReleaseRef` should be called when the given node will not be accessed by the current thread anymore.
	/// A call to `ReleaseRef` should have been balanced with a prior call to `DeRefLink`.
	/// The underlying tagged pointer value can have the `delete mark` set, so be cautious.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn ReleaseRef(self)
	{
		unimplemented!();
	}
	
	BUG: ? Surely DeleteNode ? should be called after PopLeft / PopRight ?
	
	/// Weirdly, `DeleteNode` is only called from Cursor methods `InsertAfter` and `Delete`.
	///
	/// The procedure `DeleteNode` should be called when a node has been logically removed from the data structure and its memory should eventually be reclaimed.
	/// The user operation that called `DeleteNode` is responsible\* for removing all references to the deleted node from the active nodes in the data structure.
	/// This is similar to what is required when calling a memory allocator directly in a sequential data structure.
	/// However, independently of the state at the call of `DeleteNode` and of when concurrent operations eventually remove their (own or created) references to the deleted node, 'BEWARE&CLEANUP' will not reclaim the deleted node until it is safe to do so, ie, when there are no threads that could potentially access the node anymore, thus completely avoiding the possibility of dangling pointers.
	/// \* After the call of `DeleteNode`, concurrent operations may still use references to the deleted node and might even temporary add links to it, although concurrent operations that observe a deleted node are supposed to eventually remove all references and links to it.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn DeleteNode(&self)
	{
		let node = self;
		
		// DN1
		// This ReleaseRef() is wrong, as it happens just before DeleteNode() is called.
		// node.ReleaseRef();
		
		// DN2 - DN12
		// CleanUpLocal / Scan / CleanUpAll
	}
	
	#[inline(always)]
	pub(crate) fn move_value(self) -> Option<NonNull<T>>
	{
		if self.0.is_null()
		{
			None
		}
		else
		{
			let node: &Node<T> = self.0.as_reference();
			node.move_value()
		}
	}
	
	#[inline(always)]
	pub(crate) fn ref_value(self) -> Option<NonNull<T>>
	{
		if self.0.is_null()
		{
			None
		}
		else
		{
			let node: &Node<T> = self.0.as_reference();
			node.ref_value()
		}
	}
	
	#[inline(always)]
	fn has_delete_tag(&self) -> bool
	{
		self.0.has_delete_tag()
	}
	
	#[inline(always)]
	pub(crate) fn to_link_stack(self) -> StackLink<T>
	{
		self.0.without_lock_dereference_tag_but_with_delete_tag_if_set()
	}
	
	#[inline(always)]
	pub(crate) fn without_delete_tag_link_stack(self) -> StackLink<T>
	{
		self.0.without_delete_tag()
	}
	
	#[inline(always)]
	pub(crate) fn with_delete_tag_link_stack(self) -> StackLink<T>
	{
		self.0.with_delete_tag()
	}
	
	#[inline(always)]
	pub(crate) fn next_link_atomic<'a>(self) -> &'a AtomicLink<T>
	{
		let node: &Node<T> = self.0.as_reference();
		
		&node.next
	}
	
	#[inline(always)]
	pub(crate) fn next_link_stack(self) -> StackLink<T>
	{
		self.next_link_atomic().without_lock_dereference_tag_but_with_delete_tag_if_set()
	}
	
	#[inline(always)]
	pub(crate) fn previous_link_atomic<'a>(self) -> &'a AtomicLink<T>
	{
		let node: &Node<T> = self.0.as_reference();
		
		&node.previous
	}
	
	#[inline(always)]
	pub(crate) fn previous_link_stack(self) -> StackLink<T>
	{
		self.previous_link_atomic().without_lock_dereference_tag_but_with_delete_tag_if_set()
	}
	
	//noinspection SpellCheckingInspection
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CorrectPrev(self, node: Self) -> Self
	{
		let mut prev: DereferencedLink<T> = self;
		
		// CP1
		let mut lastlink: Option<DereferencedLink<T>> = None;
		
		// CP2
		loop
		{
			// CP3
			let link1: StackLink<T> = node.previous_link_stack();
			if link1.has_delete_tag()
			{
				break
			}
			
			// CP4
			let mut prev2: DereferencedLink<T> = prev.next_link_atomic().DeRefLink();
			
			// CP5
			if prev2.has_delete_tag()
			{
				// CP6
				if let Some(lastlink_) = lastlink
				{
					// CP7
					prev.previous_link_atomic().SetMark();
					
					// CP8
					lastlink_.next_link_atomic().CASRef(prev.to_link_stack(), prev2.without_delete_tag_link_stack());
					
					// CP9
					prev2.ReleaseRef();
					prev.ReleaseRef();
					
					// CP10
					prev = lastlink_;
					lastlink = None;
					
					// CP11
					continue
				}
				
				// CP12
				prev2.ReleaseRef();
				
				// CP13
				prev2 = prev.previous_link_atomic().DeRefLink();
				
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
				if let Some(lastlink) = lastlink
				{
					lastlink.ReleaseRef()
				}
				
				// CP18
				lastlink = if prev.0.is_null()
				{
					None
				}
				else
				{
					Some(prev)
				};
				
				// CP19
				prev = prev2;
				
				// CP20
				continue
			}
			
			// CP21
			prev2.ReleaseRef();
			
			// CP22
			if node.previous_link_atomic().CASRef(link1, prev.without_delete_tag_link_stack())
			{
				// CP23
				if prev.previous_link_stack().has_delete_tag()
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
		if let Some(lastlink) = lastlink
		{
			lastlink.ReleaseRef()
		}
		
		// CP27
		prev
	}
}
