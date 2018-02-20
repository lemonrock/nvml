// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A free list element, once `push()'d`, can never be freed until the FreeList is dropped.
/// Wraps a value of type `T` that is being pushed and popped.
/// It is recommended that the value `T` is `Option<SomeValue>`, to make it easier to take the value.
/// Dropping this FreeListElement ***will not drop*** the value it contains and ***will not free*** the heap memory used by it.
/// `T` can be a variable-length array.
#[derive(Debug)]
pub struct FreeListElement<T>
{
	next: *mut FreeListElement<T>,
	value: T,
}

impl<T> Deref for FreeListElement<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.value()
	}
}

impl<T> DerefMut for FreeListElement<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.value_mut()
	}
}

impl<T> CtoSafe for FreeListElement<T>
{
	#[inline(always)]
	default fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.cto_pool_opened_always(cto_pool_arc)
	}
}

impl<T: CtoSafe> CtoSafe for FreeListElement<T>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.cto_pool_opened_always(cto_pool_arc);
		self.value.cto_pool_opened(cto_pool_arc)
	}
}

impl<T: Copy> FreeListElement<T>
{
	/// Returns a copy of the value.
	/// Useful if T is a raw pointer or `NonNull`.
	#[inline(always)]
	pub fn copy_value(&mut self) -> T
	{
		self.value
	}
}

impl<T: Clone> FreeListElement<T>
{
	/// Returns a clone of the value.
	/// Useful if T is an Arc.
	/// Note that the original value will not be dropped until the FreeList itself is dropped, once this FreeListElement has been `push()'d`.
	#[inline(always)]
	pub fn clone_value(&mut self) -> T
	{
		self.value.clone()
	}
}

impl<V> FreeListElement<Option<V>>
{
	/// Takes the value held by this FreeListElement, replacing it with `None`, and dropping neither.
	#[inline(always)]
	pub fn take_value(&mut self) -> Option<V>
	{
		self.replace_value(None)
	}
	
	/// Takes the value held by this FreeListElement, replacing it with `None`, and dropping neither.
	/// Panics if the value is already `None`, ie it was already taken.
	#[inline(always)]
	pub fn take_value_once(&mut self) -> V
	{
		self.take_value().expect("value was already taken")
	}
}

impl<T> FreeListElement<T>
{
	/// Read-only reference to value.
	/// Useful if value represents an `UnsafeCell` or `RefCell`.
	#[inline(always)]
	pub fn value(&self) -> &T
	{
		&self.value
	}
	
	/// Mutable reference to value for in-place modification.
	/// Useful if initializing an initial value representing a variable-length array.
	#[inline(always)]
	pub fn value_mut(&mut self) -> &mut T
	{
		&mut self.value
	}
	
	/// Returns the value held by this FreeListElement, replacing it with the `replacement` and dropping neither.
	/// The replacement value will not be dropped until the FreeList itself is dropped, once this FreeListElement has been `push()'d`.
	#[inline(always)]
	pub fn replace_value(&mut self, replacement: T) -> T
	{
		unsafe { replace(&mut self.value, replacement) }
	}
	
	#[inline(always)]
	fn free_list_is_being_dropped_or_was_never_pushed_ever_so_free(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		unsafe { drop_in_place(&mut self.value) };
		
		cto_pool_arc.free_pointer(self);
	}
	
	#[inline(always)]
	fn new(cto_pool_arc: &CtoPoolArc, initial_value: T, trailing_additional_size_in_value_in_bytes: usize) -> NonNull<Self>
	{
		let mut this: NonNull<Self> = cto_pool_arc.aligned_allocate_or_panic_of_type(AtomicIsolationSize, size_of::<Self>() + trailing_additional_size_in_value_in_bytes);
		unsafe
		{
			let this = this.as_mut();
			
			this.reset_next_to_null_so_cto_pool_opened_can_not_read_junk();
			
			write(&mut this.value, initial_value)
		}
		this
	}
	
	#[inline(always)]
	fn next_is_null(&self) -> bool
	{
		self.next.is_null()
	}
	
	// Always overwritten by `FreeList.push()`, but used by `cto_pool_opened()`.
	// Affects any FreeListElement in the elimination array.
	#[inline(always)]
	fn reset_next_to_null_so_cto_pool_opened_can_not_read_junk(&mut self)
	{
		// Always overwritten by `FreeList.push()`, but used by `cto_pool_opened()`.
		unsafe { write(&mut self.next, null_mut()) };
	}
	
	#[inline(always)]
	fn cto_pool_opened_always(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		let next = self.next;
		if next.is_not_null()
		{
			unsafe { &mut * next }.cto_pool_opened(cto_pool_arc)
		}
	}
}
