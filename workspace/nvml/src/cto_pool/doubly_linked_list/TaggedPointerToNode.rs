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
	
	const LockDereferenceBit: usize = 0x02;
	
	const AllMarkBits: usize = Self::LockDereferenceBit | Self::IsDeleteMarkBit;
	
	const PointerMask: usize = !Self::AllMarkBits;
	
	// Ideally, instead of 0, it should be `null::<T>() as usize`, but `null()` is not allowed in constants and const fn.
	pub(crate) const Null: Self = Self::new(/*null::<T>() as usize*/ 0);
	
	/// This function is a synthesis of Sundell & Tsigas, 2008's `CASRef` and Gildentam et al 2005 / 2008 (?2009) `CompareAndSwapRef`.
	/// The function `CASRef` is used to update a link for which there might be concurrent updates.
	/// It returns `true` if the update was successful and `false` otherwise.
	/// The thread calling `CASRef` should hold a spinlock'd 'LockDereference' flag\* to the node that is to be stored in the link.
	/// NOTE: The Sundell & Tsigas paper defines `old` as `old: pointer to Node` and `new` as `new: pointer to Node`, but they define `pointer` far more loosely than we do, ie it can be a reference (`&Node<T>`), raw and null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
	/// `node` is also called `new` in Sundell & Tsigas.
	/// This function *can* be called with `node` pointers **with** the delete flag set.
	/// \* The original algorithm in Gildentam et al used a hazard pointer and a tracing garbage collector.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CASRef(&self, old: Self, node: Self) -> bool
	{
		// Called `address` in Sundell & Tsigas.
		let link = self;
		
		// C1
		if link.CAS(old, node)
		{
			// C2
			let node = node.ptr();
			if node.is_not_null()
			{
				// C3
				unsafe { &*node }.increment_reference_count()
				
				// C4
				// (no-op for us as we do not use a tracing garbage collector)
			}
			
			// C5
			let old = old.ptr();
			if old.is_not_null()
			{
				unsafe { &*old }.decrement_reference_count()
			}
			
			// C6
			return true
		}
		
		// C7
		false
	}
	
	/// This function is a synthesis of Sundell & Tsigas, 2008's `StoreRef` and Gildentam et al 2005 / 2008 (?2009) `StoreRef`.
	/// NOTE: The Sundell & Tsigas paper defines `node` as `node: pointer to Node`, but they define `pointer` far more loosely than we do, ie it can be a reference (`&Node<T>`), raw and null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
	/// Definition in "Efficient and Reliable Lock-Free Memory Reclamation Based on Reference Counting", Gildentam et al 2005 / Gildentam et al 2005 / 2008 (?2009).
	/// To update a link for which there cannot be any concurrent updates the procedure `StoreRef` should be called.
	/// This procedure will make sure that any thread that then calls `DeRefLink` on the link (`TaggedPointerToNode`) can safely do so, if the thread has a spinlock'd 'LockDereference' flag\* to the node which contains the link.
	/// The requirements are that that no other thread than the calling thread will possibly write concurrently to the link (otherwise `CASRef` should be invoked instead).
	/// This function should only be called with `node` pointers **without** the delete flag set.
	/// \* The original algorithm in Gildentam et al used a hazard pointer and a tracing garbage collector.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn StoreRef(&self, node: Self)
	{
		// Called `address` in Sundell & Tsigas.
		let link = self;
		
		// S1
		let old = link.ptr();
		
		// S2
		link.set_tagged_pointer(node.tagged_pointer());
		
		// S3
		let node = node.ptr();
		if node.is_not_null()
		{
			// S4
			unsafe { &* node }.increment_reference_count()
			
			// S7
			// (no-op for us as we do not use a tracing garbage collector)
		}
		
		// S6
		if old.is_not_null()
		{
			unsafe { &* old }.decrement_reference_count();
		}
	}
	
	/// NOTE: The Sundell & Tsigas paper defines `address` as `address: pointer to word`, `old` as `old: word` and `new` as `new: word`.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CAS(&self, old: Self, new: Self) -> bool
	{
		let address = self;
		
		let tagged_pointer_atomic: &AtomicUsize = unsafe { transmute(&address.tagged_pointer) };
		
		tagged_pointer_atomic.compare_exchange(old.tagged_pointer, new.tagged_pointer, Acquire, Acquire).is_ok()
	}
	
	/// The function `DeRefLink` safely dereferences the given link, setting a spinlock'd 'LockDereference' flag inside the dereferenced node\*, thus guaranteeing the safety of future accesses to the returned node.
	/// In particular, the calling thread can safely dereference and/or update any links in the returned node subsequently.
	/// NOTE: The Sundell & Tsigas paper defines the result of this function as `:pointer to Node`, but they define `pointer` far more loosely than we do, ie it can be a reference (`&Node<T>`), raw and null (`*const Node<T>`) or tagged with a delete mark (`TaggedPointerToNode<T>`).
	/// \* The original algorithm in Gildentam et al used a hazard pointer and a tracing garbage collector.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn DeRefLink(&self) -> Self
	{
		// Called `address` in Sundell & Tsigas.
		let address = self;
		
		let tagged_pointer_atomic: &AtomicUsize = unsafe { transmute(&address.tagged_pointer) };
		
		let mut node_pointer: usize;
		while
		{
			node_pointer = tagged_pointer_atomic.fetch_or(Self::LockDereferenceBit, Acquire);
			node_pointer & Self::LockDereferenceBit != 0
		}
		{
			Back_Off()
		}
		
		let node = Self::new(node_pointer);
		
		// Purpose of this - increment reference count to prevent deletion?
		if node.ptr().is_not_null()
		{
			node.increment_reference_count()
		}
		
		let old = tagged_pointer_atomic.fetch_and(!Self::LockDereferenceBit, Release);
		
		debug_assert_eq!(old & Self::LockDereferenceBit, 0, "Did not set LockDereferenceBit spinlock somehow");
		
		node
	}
	
	/// The procedure ReleaseRef should be called when the given node will not be accessed by the current thread anymore. It clears the corresponding spinlock'd flag\*.
	/// \* The original algorithm in Gildentam et al used a hazard pointer and a tracing garbage collector.
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
		self.ReleaseRef();
		
		// Assumption that this is what we should do with the second argument.
		what_is_this.ReleaseRef();
	}
	
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
	
	/// Strictly speaking, a potentially null reference to a known, good pointer, so a subset of Self
	#[inline(always)]
	pub(crate) fn p<'a>(self) -> Self
	{
		Self::new(self.ptr() as usize)
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
	
	/// Strictly speaking, a potentially null reference to a known, good pointer, so a subset of Self.
	/// Can be null.
	#[inline(always)]
	fn ptr(self) -> *const Node<T>
	{
		(self.tagged_pointer() & Self::PointerMask) as *const Node<T>
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
	
	#[inline(always)]
	fn set_tagged_pointer(&self, tagged_pointer: usize)
	{
		let tagged_pointer_atomic: &AtomicUsize = unsafe { transmute(&self.tagged_pointer) };
		
		tagged_pointer_atomic.store(tagged_pointer, Release)
	}
}
