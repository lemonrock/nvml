// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#[derive(Debug)]
struct Link<T>
{
	tagged_pointer: AtomicUsize,
	phantom_data: PhantomData<T>,
}

impl<T> Clone for Link<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::new(self.tagged_pointer())
	}
}

impl<T> Default for Link<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Null
	}
}

impl<T> PartialEq for Link<T>
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

impl<T> Eq for Link<T>
{
}

impl<T> Link<T>
{
	const IsDeleteMarkBit: usize = 0x01;
	
	const AllMarkBits: usize = Self::IsDeleteMarkBit;
	
	const MinimumPointerAlignment: usize = Self::AllMarkBits + 1;
	
	const PointerMask: usize = !Self::AllMarkBits;
	
	pub(crate) const Null: Self = Self::new(null::<T>() as usize);
	
	/// This function ***MUST*** only ever called on `prev` pointers; perhaps we should specialise the `Link` type?
	/// NOTE: The Sundell & Tsigas paper defines `link` as `link: pointer to pointer to Node`, but this definition is wrong for a strong-typed scheme not using an union.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn SetMark(&self)
	{
		let link = self;
		
		// SM1
		loop
		{
			/*
				Algorithm is correct but we always atomically load, which, since we have a copy at SM2, is no longer required
				
				// SM2
				let node = *link;
				
				// SM3
				if node.d_is_true() || link.CAS(node, Link::new_with_delete_mark(node.p()))
				{
					break
				}
			*/
			
			// SM2
			let node = link.tagged_pointer();
			
			// SM3
			// NOTE: CAS, not CASRef
			if Self::d_is_true_(node) || link.CAS(Self::new(node), Link::new_with_delete_mark(Self::p_(node)))
			{
				break
			}
		}
	}
	
	/// NOTE: The Sundell & Tsigas paper defines `address` as `address: pointer to word`, `old` as `old: word` and `new` as `new: word`, but these definitions are wrong for a strong-typed scheme not using an union.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CAS(&self, old: Self, new: Self) -> bool
	{
		let address = self;
		
		unimplemented!("External definition required")
	}
	
	/// NOTE: The Sundell & Tsigas paper defines `old` as `old: pointer to Node` and `new` as `new: pointer to Node`, but these definitions are wrong for a strong-typed scheme not using an union.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn CASRef(&self, old: Self, new: Self) -> bool
	{
		let address = self;
		
		unimplemented!("External definition required")
	}
	
	/// NOTE: The Sundell & Tsigas paper defines `node` as `node: pointer to Node`, but this definition is wrong for a strong-typed scheme not using an union.
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn StoreRef(&self, node: Self)
	{
		let address = self;
		
		unimplemented!("External definition required")
	}
	
	#[allow(non_snake_case)]
	#[inline(always)]
	pub(crate) fn DeRefLink(&self) -> &Node<T>
	{
		let address = self;
		
		unimplemented!("External definition required")
	}
	
	#[inline(always)]
	pub(crate) const fn new_with_delete_mark(node_pointer: &Node<T>) -> Self
	{
		let tagged_pointer = Self::ref_to_ptr_to_usize(node_pointer);
		Self::new(Self::ref_to_ptr_to_usize(node_pointer) | Self::IsDeleteMarkBit)
	}
	
	#[inline(always)]
	pub(crate) const fn new_without_marks(node_pointer: &Node<T>) -> Self
	{
		let tagged_pointer = Self::ref_to_ptr_to_usize(node_pointer);
		Self::new(Self::ref_to_ptr_to_usize(node_pointer))
	}
	
	#[inline(always)]
	pub(crate) fn d(&self) -> bool
	{
		self.d_is_true()
	}
	
	#[inline(always)]
	pub(crate) fn d_is_true(&self) -> bool
	{
		Self::d_is_true_(self.tagged_pointer())
	}
	
	#[inline(always)]
	pub(crate) fn d_is_false(&self) -> bool
	{
		self.tagged_pointer() & Self::IsDeleteMarkBit != Self::IsDeleteMarkBit
	}
	
	#[inline(always)]
	pub(crate) fn d_is_true_(tagged_pointer: usize) -> bool
	{
		tagged_pointer & Self::IsDeleteMarkBit == Self::IsDeleteMarkBit
	}
	
	#[inline(always)]
	pub(crate) fn p<'a>(&self) -> &'a Node<T>
	{
		let tagged_pointer = self.tagged_pointer();
		
		Self::p_(self.tagged_pointer())
	}
	
	#[inline(always)]
	const fn new(tagged_pointer: usize) -> Self
	{
		Self
		{
			tagged_pointer: AtomicUsize::new(tagged_pointer),
			phantom_data: PhantomData,
		}
	}
	
	#[inline(always)]
	fn p_<'a>(tagged_pointer: usize) -> &'a Node<T>
	{
		let raw_pointer = (tagged_pointer & Self::PointerMask) as *const Node<T>;
		
		debug_assert!(raw_pointer.is_not_null(), "raw pointer is null; the only way this is possible is either an incompletely instantiated LockFreeDoublyLinkedListAndDeque OR use after link.StoreRef(Link::Null)");
		
		unsafe { &* raw_pointer }
	}
	
	#[inline(always)]
	fn tagged_pointer(&self) -> usize
	{
		self.tagged_pointer.load(Acquire)
	}
	
	#[inline(always)]
	const fn ref_to_ptr_to_usize(node_pointer: &Node<T>) -> usize
	{
		let pointer = node_pointer as *const Node<T> as usize;
		debug_assert_eq!(pointer % Self::MinimumPointerAlignment, 0, "pointer must not have its Least Significant Bit (LSB) set, ie it it most have an alignment of at least {}", Self::MinimumPointerAlignment);
		pointer
	}
}
