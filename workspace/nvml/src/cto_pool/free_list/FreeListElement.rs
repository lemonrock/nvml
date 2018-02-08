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
	pub fn value(&mut self) -> &T
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
		
		cto_pool_arc.pool_pointer().free(self);
	}
	
	#[inline(always)]
	fn new(cto_pool_arc: &CtoPoolArc, initial_value: T, trailing_additional_size_in_value_in_bytes: usize) -> NonNull<Self>
	{
		let mut this: NonNull<Self> = cto_pool_arc.aligned_allocate_or_panic_of_type(AtomicIsolationSize, size_of::<Self>() + trailing_additional_size_in_value_in_bytes);
		unsafe
		{
			let this = this.as_mut();
			
			// Not required, as always overwritten on push.
			// write(&mut this.next, null_mut());
			
			write(&mut this.value, initial_value)
		}
		this
	}
}
