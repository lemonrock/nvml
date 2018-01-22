// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A `TaggedPointerToNode` represents a `pointer to Node` in the Sundell & Tsigas paper.
/// Hence `&TaggedPointerToNode` is the same as `pointer to a pointer to a Node`
/// This paper defines `pointer` far more loosely than Rust might. In Rust, it can be a reference (`&Node<T>`), raw and possibly null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
#[derive(Debug)]
struct TaggedPointerToNode<T>
{
	tagged_pointer: usize,
	phantom_data: PhantomData<T>,
}

impl<T> Clone for TaggedPointerToNode<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self
		{
			tagged_pointer: self.tagged_pointer(),
			phantom_data: PhantomData,
		}
	}
}

impl<T> Copy for TaggedPointerToNode<T>
{
}

impl<T> PartialEq for TaggedPointerToNode<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.tagged_pointer() == other.tagged_pointer()
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		self.tagged_pointer() != other.tagged_pointer()
	}
}

impl<T> Eq for TaggedPointerToNode<T>
{
}

impl<T> Deref for TaggedPointerToNode<T>
{
	type Target = Node<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		let raw_pointer = (self.tagged_pointer() & Self::PointerMask) as *const Node<T>;
		unsafe { & * raw_pointer }
	}
}

impl<T> DerefMut for TaggedPointerToNode<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		// TODO: This can probably violate alias g'tees.
		
		let raw_pointer = (self.tagged_pointer() & Self::PointerMask) as *mut Node<T>;
		unsafe { &mut * raw_pointer }
	}
}

impl<T> TaggedPointerToNode<T>
{
	const IsDeleteMarkBit: usize = 0x01;
	
	const AllMarkBits: usize = Self::IsDeleteMarkBit;
	
	const PointerMask: usize = !Self::AllMarkBits;
	
	// Ideally, instead of 0, it should be `null::<T>() as usize`, but `null()` is not allowed in constants and const fn.
	pub(crate) const Null: Self = Self::new(/*null::<T>() as usize*/ 0);
	
	/// This function ***MUST*** only ever called on `prev` pointers
	/// NOTE: The Sundell & Tsigas paper defines `link` as `link: pointer to pointer to Node`,
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn SetMark(&self)
	{
		let link = self;
		
		// SM1
		loop
		{
			// SM2
			let node = *link;
			
			// SM3
			if node.d_is_true() || link.CAS(node, node.p().with_delete_mark())
			{
				break
			}
		}
	}
	
	/// NOTE: The Sundell & Tsigas paper defines `address` as `address: pointer to word`, `old` as `old: word` and `new` as `new: word`.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CAS(&self, _old: Self, _new: Self) -> bool
	{
		let _address = self;
		
		unimplemented!("External definition required")
	}
	
	/// The function `CASRef` (also called `CompareAndSwapRef` in related papers) is used to update a link for which there might be concurrent updates.
	/// It returns `true` if the update was successful and `false` otherwise.
	/// The thread calling `CASRef` should have an HP to the node that is to be stored in the link.
	/// NOTE: The Sundell & Tsigas paper defines `old` as `old: pointer to Node` and `new` as `new: pointer to Node`, but they define `pointer` far more loosely than we do, ie it can be a reference (`&Node<T>`), raw and null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CASRef(&self, _old: Self, _new: Self) -> bool
	{
		let _address = self;
		
		unimplemented!("External definition required")
	}
	
	/// NOTE: The Sundell & Tsigas paper defines `node` as `node: pointer to Node`, but they define `pointer` far more loosely than we do, ie it can be a reference (`&Node<T>`), raw and null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn StoreRef(&self, _node: Self)
	{
		let _address = self;
		
		unimplemented!("External definition required")
	}
	
	
	// ?Hence this function is defined for *both* `Link` and `Node`?
	
	/// The function `DeRefLink` safely dereferences the given link, setting an HP to the dereferenced node, thus guaranteeing the safety of future accesses to the returned node.
	/// In particular, the calling thread can safely dereference and/or update any links in the returned node subsequently.
	/// NOTE: The Sundell & Tsigas paper defines the result of this function as `:pointer to Node`, but they define `pointer` far more loosely than we do, ie it can be a reference (`&Node<T>`), raw and null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn DeRefLink(&self) -> Self
	{
		let _address = self;
		
		unimplemented!("External definition required")
	}
	
	/// The procedure ReleaseRef should be called when the given node will not be accessed by the current thread anymore. It clears the corresponding HP.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn ReleaseRef(self)
	{
		let _node = self;
		
		unimplemented!("External definition required")
	}
	
	// Problem: some parts of the algorithm call `ReleaseRef()` with 2 (two) arguments! WTF?
	// Hence this overloaded variant, `ReleaseRef2()`.
	#[allow(non_snake_case)]
	#[inline(always)]
	fn ReleaseRef2(self, what_is_this: Self)
	{
		// Assumption about ReleaseRef2
		self.ReleaseRef();
		what_is_this.ReleaseRef();
	}
	
	//noinspection SpellCheckingInspection
	#[allow(non_snake_case)]
	#[inline(always)]
	fn CorrectPrev(self, node: TaggedPointerToNode<T>) -> Self
	{
		let mut prev = self;
		
		// CP1
		let mut lastlink: *const Node<T> = null();
		
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
					lastlink_ref.next.CASRef(prev, prev2.p().without_delete_mark());
					
					// CP9
					prev2.p().ReleaseRef2(prev);
					
					// CP10
					prev = Self
					{
						tagged_pointer: lastlink as usize,
						phantom_data: PhantomData,
					};
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
					let lastlink_ref = Self::new(lastlink as usize);
					lastlink_ref.ReleaseRef()
				}
				
				// CP18
				lastlink = prev.tagged_pointer() as *const Node<T>;
				
				// CP19
				prev = prev2;
				
				// CP20
				continue
			}
			
			// CP21
			prev2.ReleaseRef();
			
			// CP22
			if node.prev.CASRef(link1, prev.without_delete_mark())
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
			let lastlink_ref = Self::new(lastlink as usize);
			lastlink_ref.ReleaseRef()
		}
		
		// CP27
		prev
	}
	
	#[inline(always)]
	pub(crate) fn without_delete_mark(self) -> Self
	{
		let tagged_pointer = self.tagged_pointer();
		Self::new(tagged_pointer | !Self::IsDeleteMarkBit)
	}
	
	#[inline(always)]
	pub(crate) fn with_delete_mark(self) -> Self
	{
		let tagged_pointer = self.tagged_pointer();
		Self::new(tagged_pointer | Self::IsDeleteMarkBit)
	}
	
	#[inline(always)]
	pub(crate) fn d(self) -> bool
	{
		self.d_is_true()
	}
	
	#[inline(always)]
	pub(crate) fn d_is_true(self) -> bool
	{
		Self::d_is_true_(self.tagged_pointer())
	}
	
	#[inline(always)]
	pub(crate) fn d_is_false(self) -> bool
	{
		self.tagged_pointer() & Self::IsDeleteMarkBit != Self::IsDeleteMarkBit
	}
	
	#[inline(always)]
	pub(crate) fn d_is_true_(tagged_pointer: usize) -> bool
	{
		tagged_pointer & Self::IsDeleteMarkBit == Self::IsDeleteMarkBit
	}
	
	/// Strictly speaking, a potentially null reference to a known, good pointer, so a subset of Self
	#[inline(always)]
	pub(crate) fn p<'a>(self) -> Self
	{
		Self::new(self.tagged_pointer() & Self::PointerMask)
	}
	
	#[inline(always)]
	pub(crate) const fn new(tagged_pointer: usize) -> Self
	{
		Self
		{
			tagged_pointer,
			phantom_data: PhantomData,
		}
	}
	
	#[inline(always)]
	fn tagged_pointer(self) -> usize
	{
		let tagged_pointer_atomic: &AtomicUsize = unsafe { transmute(&self.tagged_pointer) };
		
		tagged_pointer_atomic.load(Acquire)
	}
}
