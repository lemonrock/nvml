// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


pub(crate) trait TaggedPointer<P>: PartialEq + Eq
{
	const DeleteTagBit: usize = 0x01;
	
	const LockDereferenceTagBit: usize = 0x02;
	
	const AllTagBits: usize = Self::DeleteTagBit | Self::LockDereferenceTagBit;
	
	const MinimumPointerAlignment: usize = Self::AllTagBits + 1;
	
	const PointerBitMask: usize = !Self::AllTagBits;
	
	const DeleteTagBitMask: usize = !Self::DeleteTagBit;
	
	const LockDereferenceTagBitMask: usize = !Self::LockDereferenceTagBit;
	
	/// Does not have Delete mark or LockDereference mark.
	/// Is definitively non-null.
	#[inline(always)]
	fn as_reference<'a>(&self) -> &'a P
	{
		let potentially_null_pointer = self.as_potentially_null_pointer();
		
		debug_assert!(potentially_null_pointer.is_not_null(), "is null");
		
		unsafe { &* potentially_null_pointer }
	}
	
	/// Clears Delete mark and LockDereference mark before checking.
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.as_potentially_null_pointer().is_null()
	}
	
	/// Clears Delete mark and LockDereference mark before checking.
	#[inline(always)]
	fn is_not_null(&self) -> bool
	{
		self.as_potentially_null_pointer().is_not_null()
	}
	
	/// Does not have Delete mark or LockDereference mark.
	#[inline(always)]
	fn as_potentially_null_pointer(&self) -> *mut P
	{
		(self.value() & Self::PointerBitMask) as *mut P
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _without_lock_dereference_tag_but_with_delete_tag_if_set(&self) -> usize
	{
		self.value() & Self::LockDereferenceTagBitMask
	}
	
	#[inline(always)]
	fn has_delete_tag(&self) -> bool
	{
		self.value() & Self::DeleteTagBit == Self::DeleteTagBit
	}
	
	#[inline(always)]
	fn does_not_have_delete_tag(&self) -> bool
	{
		self.value() & Self::DeleteTagBit != Self::DeleteTagBit
	}
	
	#[inline(always)]
	fn has_lock_dereference_tag(&self) -> bool
	{
		self.value() & Self::LockDereferenceTagBit == Self::LockDereferenceTagBit
	}
	
	#[inline(always)]
	fn does_not_have_lock_dereference_tag(&self) -> bool
	{
		self.value() & Self::LockDereferenceTagBit != Self::LockDereferenceTagBit
	}
	
	#[inline(always)]
	fn without_lock_dereference_tag_but_with_delete_tag_if_set(&self) -> NonAtomicTaggedPointer<P>
	{
		NonAtomicTaggedPointer::new(self._without_lock_dereference_tag_but_with_delete_tag_if_set())
	}
	
	#[inline(always)]
	fn with_delete_tag(&self) -> NonAtomicTaggedPointer<P>
	{
		NonAtomicTaggedPointer::new(self.value() | Self::DeleteTagBit)
	}
	
	#[inline(always)]
	fn without_delete_tag(&self) -> NonAtomicTaggedPointer<P>
	{
		NonAtomicTaggedPointer::new(self.value() & Self::DeleteTagBitMask)
	}
	
	#[inline(always)]
	fn with_lock_dereference_tag(&self) -> NonAtomicTaggedPointer<P>
	{
		NonAtomicTaggedPointer::new(self.value() | Self::LockDereferenceTagBit)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn value(&self) -> usize;
}
