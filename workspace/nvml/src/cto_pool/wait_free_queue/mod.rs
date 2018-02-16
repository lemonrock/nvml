// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]


use IsNotNull;
use ::std::cell::Cell as CopyCell;
use ::std::cell::UnsafeCell;
use ::std::intrinsics::atomic_load_acq;
use ::std::intrinsics::atomic_store_rel;
use ::std::intrinsics::atomic_cxchg;
use ::std::intrinsics::atomic_cxchg_acq_failrelaxed;
use ::std::intrinsics::atomic_cxchg_acqrel;
use ::std::intrinsics::atomic_cxchg_relaxed;
use ::std::intrinsics::atomic_xadd;
use ::std::intrinsics::atomic_xadd_relaxed;
use ::std::mem::uninitialized;
use ::std::mem::size_of;
use ::std::ops::Deref;
use ::std::ops::Neg;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::ptr::read;
use ::std::ptr::read_volatile;
use ::std::ptr::write;
use ::std::ptr::write_volatile;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::Ordering::SeqCst;
use ::std::sync::atomic::spin_loop_hint;





#[inline(always)]
fn free<T>(pointer: NonNull<T>)
{
	unimplemented!()
}

#[inline(always)]
fn page_size_align_malloc<T>() -> NonNull<T>
{
	#[inline(always)]
	fn align_malloc<T>(alignment: usize, size: usize) -> NonNull<T>
	{
		unimplemented!()
	}
	
	const PAGE_SIZE: usize = 4096;
	align_malloc(PAGE_SIZE, size_of::<T>())
}



macro_rules! do_while
{
    (
        do
        $body:block
        while $cond:expr
    ) =>
    {
        while
        {
            $body;
            $cond
        }
        {
        }
    };
}

/// A memory fence to ensure sequential consistency.
#[inline(always)]
fn sequentially_consistent_fence()
{
	//  __atomic_thread_fence(__ATOMIC_SEQ_CST)
	fence(SeqCst)
}

#[derive(Debug)]
struct ExtendedNonNullAtomicPointer<T>(UnsafeCell<NonNull<T>>);

impl<T> ExtendedNonNullAtomicPointer<T>
{
	#[inline(always)]
	fn get(&self) -> NonNull<T>
	{
		unsafe { read(self.0.get()) }
	}
	
	#[inline(always)]
	fn set(&self, value: NonNull<T>)
	{
		unsafe { write(self.0.get(), value) }
	}
	
	#[inline(always)]
	fn CASra(&self, cmp: &mut NonNull<T>, val: NonNull<T>) -> bool
	{
		// #define CASra(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_release, __ATOMIC_acquire)
		let (value, ok) = unsafe { atomic_cxchg_acqrel(self.0.get(), *cmp, val) };
		
		if !ok
		{
			*cmp = value;
		}
		ok
	}
}

enum void
{
}

trait PreFixOperators
{
	// the --self C operator
	#[inline(always)]
	fn pre_decrement(&mut self) -> Self;
	
	// the ++self C operator
	#[inline(always)]
	fn pre_increment(&mut self) -> Self;
}

impl PreFixOperators for isize
{
	#[inline(always)]
	fn pre_decrement(&mut self) -> Self
	{
		let old_value = *self;
		*self = old_value - 1;
		*self
	}

	#[inline(always)]
	fn pre_increment(&mut self) -> Self
	{
		let old_value = *self;
		*self = old_value + 1;
		*self
	}
}

impl PreFixOperators for usize
{
	#[inline(always)]
	fn pre_decrement(&mut self) -> Self
	{
		let old_value = *self;
		debug_assert_ne!(old_value, 0, "old_value was zero");
		*self = old_value - 1;
		*self
	}

	#[inline(always)]
	fn pre_increment(&mut self) -> Self
	{
		let old_value = *self;
		debug_assert_ne!(old_value, ::std::usize::MAX, "old_value was usize::MAX");
		*self = old_value + 1;
		*self
	}
}

trait PostFixOperators
{
	// the self-- C operator
	#[inline(always)]
	fn post_decrement(&mut self) -> Self;
	
	// the self++ C operator
	#[inline(always)]
	fn post_increment(&mut self) -> Self;
}

impl PostFixOperators for isize
{
	#[inline(always)]
	fn post_decrement(&mut self) -> Self
	{
		let old_value = *self;
		*self = old_value - 1;
		old_value
	}

	#[inline(always)]
	fn post_increment(&mut self) -> Self
	{
		let old_value = *self;
		*self = old_value + 1;
		old_value
	}
}

impl PostFixOperators for usize
{
	#[inline(always)]
	fn post_decrement(&mut self) -> Self
	{
		let old_value = *self;
		debug_assert_ne!(old_value, 0, "old_value was zero");
		*self = old_value - 1;
		old_value
	}

	#[inline(always)]
	fn post_increment(&mut self) -> Self
	{
		let old_value = *self;
		debug_assert_ne!(old_value, ::std::usize::MAX, "old_value was usize::MAX");
		*self = old_value + 1;
		old_value
	}
}

trait ToNonNull<T>
{
	#[inline(always)]
	fn to_non_null(self) -> NonNull<T>;
}

impl<T> ToNonNull<T> for *mut T
{
	#[inline(always)]
	fn to_non_null(self) -> NonNull<T>
	{
		debug_assert!(self.is_not_null(), "self is null");
		
		unsafe { NonNull::new_unchecked(self) }
	}
}

trait ExtendedNonNull<T>
{
	#[inline(always)]
	fn reference(&self) -> &T;
	
	#[inline(always)]
	fn mutable_reference(&mut self) -> &mut T;
	
	#[inline(always)]
	fn static_reference<'long>(self) -> &'long T;
}

impl<T> ExtendedNonNull<T> for NonNull<T>
{
	#[inline(always)]
	fn reference(&self) -> &T
	{
		unsafe { self.as_ref() }
	}
	
	#[inline(always)]
	fn mutable_reference(&mut self) -> &mut T
	{
		unsafe { self.as_mut() }
	}
	
	#[inline(always)]
	fn static_reference<'long>(self) -> &'long T
	{
		unsafe { &* self.as_ptr() }
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
pub(crate) struct CacheAligned<T>(T);

impl<T> Deref for CacheAligned<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<T: Default> Default for CacheAligned<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(T::default())
	}
}

impl<T> CacheAligned<T>
{
	#[inline(always)]
	pub(crate) const fn new(value: T) -> Self
	{
		CacheAligned(value)
	}
	
	#[inline(always)]
	fn initialize(&mut self, value: T)
	{
		unsafe { write(&mut self.0, value) }
	}
}

impl<T: Copy> CacheAligned<CopyCell<T>>
{
	#[inline(always)]
	pub(crate) fn get(&self) -> T
	{
		self.0.get()
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, value: T)
	{
		let pointer = self.0.as_ptr();
		unsafe { pointer.write(value) }
	}
}

impl<T: Copy> CacheAligned<volatile<T>>
{
	#[inline(always)]
	pub(crate) fn get(&self) -> T
	{
		self.0.get()
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, value: T)
	{
		self.0.set(value)
	}
	
	#[inline(always)]
	pub(crate) fn acquire(&self) -> T
	{
		self.0.acquire()
	}
	
	#[inline(always)]
	pub(crate) fn release(&self, value: T)
	{
		self.0.release(value)
	}
	
	#[inline(always)]
	pub(crate) fn relaxed_fetch_and_add(&self, increment: T) -> T
	{
		self.0.relaxed_fetch_and_add(increment)
	}
	
	#[inline(always)]
	pub(crate) fn sequentially_consistent_fetch_and_add(&self, increment: T) -> T
	{
		self.0.sequentially_consistent_fetch_and_add(increment)
	}
	
	#[inline(always)]
	pub(crate) fn relaxed_relaxed_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.relaxed_relaxed_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn sequentially_consistent_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.sequentially_consistent_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn release_acquire_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.release_acquire_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn acquire_relaxed_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.acquire_relaxed_compare_and_swap(compare, value)
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
pub(crate) struct DoubleCacheAligned<T>(T);

impl<T: Default> Default for DoubleCacheAligned<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::new(T::default())
	}
}

impl<T> DoubleCacheAligned<T>
{
	#[inline(always)]
	pub(crate) const fn new(value: T) -> Self
	{
		DoubleCacheAligned(value)
	}
}

impl<T: Copy> DoubleCacheAligned<T>
{
	#[inline(always)]
	pub(crate) fn get(&self) -> T
	{
		self.0
	}
	
	#[inline(always)]
	pub(crate) fn set(&mut self, value: T)
	{
		self.0 = value
	}
}

impl<T: Copy> DoubleCacheAligned<volatile<T>>
{
	#[inline(always)]
	pub(crate) fn get(&self) -> T
	{
		self.0.get()
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, value: T)
	{
		self.0.set(value)
	}
	
	#[inline(always)]
	pub(crate) fn acquire(&self) -> T
	{
		self.0.acquire()
	}
	
	#[inline(always)]
	pub(crate) fn release(&self, value: T)
	{
		self.0.release(value)
	}
	
	#[inline(always)]
	pub(crate) fn relaxed_fetch_and_add(&self, increment: T) -> T
	{
		self.0.relaxed_fetch_and_add(increment)
	}
	
	#[inline(always)]
	pub(crate) fn sequentially_consistent_fetch_and_add(&self, increment: T) -> T
	{
		self.0.sequentially_consistent_fetch_and_add(increment)
	}
	
	#[inline(always)]
	pub(crate) fn relaxed_relaxed_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.relaxed_relaxed_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn sequentially_consistent_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.sequentially_consistent_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn release_acquire_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.release_acquire_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn acquire_relaxed_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		self.0.acquire_relaxed_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn acquire_relaxed_compare_and_swap_do_not_mutate_compare(&self, compare: T, value: T) -> bool
	{
		self.0.acquire_relaxed_compare_and_swap_do_not_mutate_compare(compare, value)
	}
}

impl DoubleCacheAligned<volatile<PositionIndex>>
{
	const Increment: PositionIndex = PositionIndex(1);
	
	#[inline(always)]
	pub(crate) fn relaxed_fetch_and_increment(&self) -> PositionIndex
	{
		self.relaxed_fetch_and_add(Self::Increment)
	}
	
	#[inline(always)]
	pub(crate) fn sequentially_consistent_fetch_and_increment(&self) -> PositionIndex
	{
		self.sequentially_consistent_fetch_and_add(Self::Increment)
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct PositionIndex(isize);

impl Neg for PositionIndex
{
	type Output = Self;
	
	#[inline(always)]
	fn neg(self) -> Self::Output
	{
		debug_assert_ne!(self.0, ::std::isize::MIN, "Can not negate isize::MIN");
		
		PositionIndex(-self.0)
	}
}

impl PositionIndex
{
	const Zero: Self = PositionIndex(0);
	
	const InitialNext: Self = PositionIndex(1);
	
	const InitialIdx: Self = PositionIndex(-1);
	
	const SignedNumberOfCellsInANode: isize = Cells::<()>::NumberOfCellsInANode as isize;
	
	#[inline(always)]
	fn is_zero(self) -> bool
	{
		&self == &Self::Zero
	}
	
	#[inline(always)]
	fn is_not_zero(self) -> bool
	{
		&self != &Self::Zero
	}
	
	#[inline(always)]
	fn maximum_node_identifier(self) -> NodeIdentifier
	{
		NodeIdentifier(self.0 / Self::SignedNumberOfCellsInANode)
	}
	
	#[inline(always)]
	fn get_cell<Value>(self, node: &Node<Value>) -> &Cell<Value>
	{
		let cell_index = self.0 % Self::SignedNumberOfCellsInANode;
		
		node.get_cell(cell_index as usize)
	}
	
	#[inline(always)]
	fn increment(self) -> Self
	{
		debug_assert_ne!(self.0, ::std::isize::MAX, "self.0 is isize::MAX");
		
		PositionIndex(self.0 + 1)
	}
	
	#[inline(always)]
	fn increment_in_place(&mut self)
	{
		debug_assert_ne!(self.0, ::std::isize::MAX, "self.0 is isize::MAX");
		
		self.0 += 1
	}
}

#[derive(Debug)]
pub(crate) struct volatile<T: Copy>(UnsafeCell<T>);

impl volatile<NodePointerIdentifier>
{
	#[inline(always)]
	fn check<Value>(&self, current: NonNull<Node<Value>>, old_head_of_queue_node: NonNull<Node<Value>>) -> NonNull<Node<Value>>
	{
		let hazard_node_pointer_identifier = self.acquire();
		hazard_node_pointer_identifier.check(current, old_head_of_queue_node)
	}
}

impl<Value> volatile<NonNull<Node<Value>>>
{
	#[inline(always)]
	fn update(&self, mut current: NonNull<Node<Value>>, hazard_node_pointer_identifier: &volatile<NodePointerIdentifier>, old_head_of_queue_node: NonNull<Node<Value>>) -> NonNull<Node<Value>>
	{
		let mut node = self.acquire();
		
		if node.reference().identifier() < current.reference().identifier()
		{
			if !self.sequentially_consistent_compare_and_swap(&mut node, current)
			{
				if node.reference().identifier() < current.reference().identifier()
				{
					current = node;
				}
			}
			
			current = hazard_node_pointer_identifier.check(current, old_head_of_queue_node);
		}
		
		current
	}
	
	fn find_cell(&self, at_position_index: PositionIndex, this: &PerHyperThreadHandle<Value>) -> &Cell<Value>
	{
		let mut current = self.get().static_reference();
		
		let mut current_node_identifier = current.identifier();
		let maximum_node_identifier = at_position_index.maximum_node_identifier();
		while current_node_identifier < maximum_node_identifier
		{
			let mut next = current.next.get();
			
			if next.is_null()
			{
				let spare_node_to_use_for_next = this.get_non_null_spare_node();
				spare_node_to_use_for_next.reference().identifier.set(current_node_identifier.increment());
				
				if current.next.release_acquire_compare_and_swap(&mut next, spare_node_to_use_for_next.as_ptr())
				{
					next = spare_node_to_use_for_next.as_ptr();
					this.set_spare_to_null();
				}
			}
			
			current = next.to_non_null().static_reference();
			current_node_identifier.increment_in_place();
		}
		
		self.set(current.to_non_null());
		
		at_position_index.get_cell(current)
	}
}

impl<T: Copy> volatile<T>
{
	#[inline(always)]
	pub(crate) const fn new(value: T) -> Self
	{
		volatile(UnsafeCell::new(value))
	}
	
	#[inline(always)]
	pub(crate) fn get(&self) -> T
	{
		unsafe { read_volatile(self.0.get()) }
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, value: T)
	{
		unsafe { write_volatile(self.0.get(), value) }
	}
	
	/// primitives.h calls this `ACQUIRE`.
	#[inline(always)]
	pub(crate) fn acquire(&self) -> T
	{
		unsafe { atomic_load_acq(self.0.get() as *const T) }
	}
	
	/// primitives.h calls this `RELEASE`.
	#[inline(always)]
	pub(crate) fn release(&self, value: T)
	{
		unsafe { atomic_store_rel(self.0.get(), value) }
	}
	
	/// An atomic fetch-and-add that is relaxed.
	/// Returns previous value.
	/// primitives.h calls this `FAA`.
	#[inline(always)]
	pub(crate) fn relaxed_fetch_and_add(&self, increment: T) -> T
	{
		unsafe { atomic_xadd_relaxed(self.0.get(), increment) }
	}
	
	/// An atomic fetch-and-add that also ensures sequential consistency.
	/// Returns previous value.
	/// primitives.h calls this `FAAcs`.
	#[inline(always)]
	pub(crate) fn sequentially_consistent_fetch_and_add(&self, increment: T) -> T
	{
		unsafe { atomic_xadd(self.0.get(), increment) }
	}
	
	/// An atomic compare-and-swap that is completely relaxed.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	/// primitives.h calls this `CAS`.
	#[inline(always)]
	pub(crate) fn relaxed_relaxed_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		let (value, ok) = unsafe { atomic_cxchg_relaxed(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that also ensures sequential consistency.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	/// primitives.h calls this `CAScs`.
	#[inline(always)]
	pub(crate) fn sequentially_consistent_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		let (value, ok) = unsafe { atomic_cxchg(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures release semantic when succeed or acquire semantic when failed.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	/// primitives.h calls this `CASra`.
	#[inline(always)]
	pub(crate) fn release_acquire_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		let (value, ok) = unsafe { atomic_cxchg_acqrel(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures acquire semantic when succeed or relaxed semantic when failed.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	/// primitives.h calls this `CASa`.
	#[inline(always)]
	pub(crate) fn acquire_relaxed_compare_and_swap(&self, compare: &mut T, value: T) -> bool
	{
		let (value, ok) = unsafe { atomic_cxchg_acq_failrelaxed(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures acquire semantic when succeed or relaxed semantic when failed.
	/// true if successful.
	/// false if failed.
	/// compare is left unchanged.
	#[inline(always)]
	pub(crate) fn acquire_relaxed_compare_and_swap_do_not_mutate_compare(&self, compare: T, value: T) -> bool
	{
		let (value, ok) = unsafe { atomic_cxchg_acq_failrelaxed(self.0.get(), compare, value) };
		ok
	}
}

trait BottomAndTop
{
	const Bottom: Self;
	
	const Top: Self;
	
	#[inline(always)]
	fn is_bottom(self) -> bool;
	
	#[inline(always)]
	fn is_not_bottom(self) -> bool;
	
	#[inline(always)]
	fn is_top(self) -> bool;
	
	#[inline(always)]
	fn is_not_top(self) -> bool;
}

impl<T> BottomAndTop for *mut T
{
	// Works because the initial state of a Node or Cell is zeroed
	const Bottom: Self = 0 as Self;
	
	// Works because no valid pointer can currently by 2^64 - 1 (most pointers are exhausted at 2^48 - 1).
	const Top: Self = !0 as Self;
	
	#[inline(always)]
	fn is_bottom(self) -> bool
	{
		self == Self::Bottom
	}
	
	#[inline(always)]
	fn is_not_bottom(self) -> bool
	{
		self != Self::Bottom
	}
	
	#[inline(always)]
	fn is_top(self) -> bool
	{
		self == Self::Top
	}
	
	#[inline(always)]
	fn is_not_top(self) -> bool
	{
		self != Self::Top
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
struct Enqueuer<Value>
{
	// Was `id`.
	enqueue_position_index: volatile<PositionIndex>,
	
	// Was `val`.
	value_to_enqueue: volatile<*mut Value>,
}

impl<Value> Enqueuer<Value>
{
	#[inline(always)]
	fn initialize(&self)
	{
		self.enqueue_position_index.set(PositionIndex::Zero);
		self.value_to_enqueue.set(<*mut Value>::Bottom);
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self
	{
		self as *const _ as *mut _
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
struct Dequeuer
{
	// Was `id`.
	dequeue_position_index: volatile<PositionIndex>,
	
	// Was `idx`.
	dequeue_position_index_x: volatile<PositionIndex>,
}

impl Dequeuer
{
	#[inline(always)]
	fn initialize(&self)
	{
		self.dequeue_position_index.set(PositionIndex::Zero);
		self.dequeue_position_index_x.set(PositionIndex::InitialIdx);
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self
	{
		self as *const _ as *mut _
	}
}

// `pad` is to make this structure 64 bytes, ie one cache line.
// This structure is always initialized zeroed when creating a new node, ie all pointers are initially `null_mut()`.
#[repr(C, align(64))]
struct Cell<Value>
{
	value: volatile<*mut Value>,
	enqueuer: volatile<*mut Enqueuer<Value>>,
	dequeuer: volatile<*mut Dequeuer>,
	_pad: [*mut (); 5],
}

impl<Value> Cell<Value>
{
}

// 1022
// -2 presumably for the fields `next` and `id` which are also cache aligned, implying 1024 cache aligned 'lines' or 'fields' in the Node struct.
const NumberOfCellsInANode: usize = (1 << 10) - 2;

#[repr(C, align(64))]
struct Cells<Value>([Cell<Value>; NumberOfCellsInANode]);

impl<Value> Cells<Value>
{
	const NumberOfCellsInANode: usize = NumberOfCellsInANode;
	
	#[inline(always)]
	fn get_cell(&self, cell_index: usize) -> &Cell<Value>
	{
		debug_assert!(cell_index < Self::NumberOfCellsInANode, "cell_index is not less than NumberOfCellsInANode");
		unsafe { self.0.get_unchecked(cell_index) }
	}
}

// Was dimensioned by q->nprocs (as `handle_t *phs[q->nprocs]`), but variable stack arrays aren't supported by Rust ("alloca causes most optimizations to not be possible" is the unjustified reason most commonly given).
//
// We could use a Vec here but a heap allocation seems overkill and would impact performance.
// Even pushing a variable length array onto the end of a PerHyperThreadHandle still uses the heap somewhat and could easily cause cache eviction.
struct AllPerHyperThreadHandles<Value>
{
	per_thread_handles: [NonNull<PerHyperThreadHandle<Value>>; NumberOfHyperThreads::InclusiveMaximumNumberOfHyperThreads],
	index: isize,
}

impl<Value> AllPerHyperThreadHandles<Value>
{
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			per_thread_handles: unsafe { uninitialized() },
			index: 0
		}
	}
	
	#[inline(always)]
	fn check_and_update_all_hyper_thread_handle_hazard_pointers(initial_per_hyper_thread_handle: &PerHyperThreadHandle<Value>, mut potential_new_head_of_queue_node: NonNull<Node<Value>>, old_head_of_queue_node: NonNull<Node<Value>>, old_head_of_queue_node_identifier: NodeIdentifier) -> NonNull<Node<Value>>
	{
		let mut this = AllPerHyperThreadHandles::new();
		let mut each_threads_per_hyper_thread_handle = initial_per_hyper_thread_handle;
		do_while!
		{
			do
			{
				{
					let hazard_node_pointer_identifier = &each_threads_per_hyper_thread_handle.hazard_node_pointer_identifier;
				
					potential_new_head_of_queue_node = hazard_node_pointer_identifier.check(potential_new_head_of_queue_node, old_head_of_queue_node);
					potential_new_head_of_queue_node = each_threads_per_hyper_thread_handle.pointer_to_the_node_for_enqueue.update(potential_new_head_of_queue_node, hazard_node_pointer_identifier, old_head_of_queue_node);
					potential_new_head_of_queue_node = each_threads_per_hyper_thread_handle.pointer_to_the_node_for_dequeue.update(potential_new_head_of_queue_node, hazard_node_pointer_identifier, old_head_of_queue_node);
					
					this.set(each_threads_per_hyper_thread_handle.as_non_null());
				}
				each_threads_per_hyper_thread_handle = each_threads_per_hyper_thread_handle.next.get().static_reference();
			}
			while potential_new_head_of_queue_node.reference().identifier() > old_head_of_queue_node_identifier && each_threads_per_hyper_thread_handle.as_ptr() != initial_per_hyper_thread_handle.as_ptr()
		}
		this.check(&mut potential_new_head_of_queue_node, old_head_of_queue_node, old_head_of_queue_node_identifier);
		potential_new_head_of_queue_node
	}
	
	#[inline(always)]
	fn set(&mut self, another_wait_free_queue_per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>)
	{
		let index = self.index.post_increment();
		
		*(unsafe { self.per_thread_handles.get_unchecked_mut(index as usize) }) = another_wait_free_queue_per_hyper_thread_handle;
	}
	
	#[inline(always)]
	fn check(&mut self, new: &mut NonNull<Node<Value>>, old_head_of_queue_node: NonNull<Node<Value>>, old_head_of_queue_node_identifier: NodeIdentifier)
	{
		while (*new).reference().identifier() > old_head_of_queue_node_identifier && self.pre_decrement_index_is_not_negative()
		{
			*new = self.get_hazard_pointer_identifier().check(*new, old_head_of_queue_node);
		}
	}
	
	#[inline(always)]
	fn pre_decrement_index_is_not_negative(&mut self) -> bool
	{
		self.index.pre_decrement() >= 0
	}
	
	#[inline(always)]
	fn get_hazard_pointer_identifier(&mut self) -> &volatile<NodePointerIdentifier>
	{
		let element = *unsafe { self.per_thread_handles.get_unchecked(self.index as usize) };
		let element = element.static_reference();
		&element.hazard_node_pointer_identifier
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NodeIdentifier(isize);

impl NodeIdentifier
{
	const Initial: Self = NodeIdentifier(0);
	
	const NoHeadOfQueue: Self = NodeIdentifier(-1);
	
	#[inline(always)]
	fn to_node_pointer_identifier(self) -> NodePointerIdentifier
	{
		NodePointerIdentifier::from_node_identifier(self)
	}
	
	#[inline(always)]
	fn is_no_head_of_queue(&self) -> bool
	{
		self == &Self::NoHeadOfQueue
	}
	
	#[inline(always)]
	fn there_is_no_garbage_to_collect(&self) -> bool
	{
		self.is_no_head_of_queue()
	}
	
	#[inline(always)]
	fn there_is_not_yet_enough_garbage_to_collect<Value>(&self, pointer_to_the_node_for_dequeue: NonNull<Node<Value>>, maximum_garbage: MaximumGarbage) -> bool
	{
		(pointer_to_the_node_for_dequeue.reference().identifier().0 - self.0) < maximum_garbage.0
	}
	
	#[inline(always)]
	fn increment(self) -> Self
	{
		debug_assert_ne!(self.0, ::std::isize::MAX, "self.0 is isize::MAX");
		
		NodeIdentifier(self.0 + 1)
	}
	
	#[inline(always)]
	fn increment_in_place(&mut self)
	{
		debug_assert_ne!(self.0, ::std::isize::MAX, "self.0 is isize::MAX");
		
		self.0 += 1
	}
}

#[repr(C)]
struct Node<Value>
{
	next: CacheAligned<volatile<*mut Node<Value>>>,
	identifier: CacheAligned<CopyCell<NodeIdentifier>>,
	cells: Cells<Value>,
}

impl<Value> Node<Value>
{
	#[inline(always)]
	fn new_node() -> NonNull<Self>
	{
		let n = page_size_align_malloc();
		unsafe { n.as_ptr().write_bytes(0, 1) }
		n
	}
	
	#[inline(always)]
	fn get_cell(&self, cell_index: usize) -> &Cell<Value>
	{
		&self.cells.get_cell(cell_index)
	}
	
	#[inline(always)]
	fn free_garbage_nodes_excluding_upto(&self, newer_exclusive: NonNull<Self>)
	{
		let mut older = self;
	
		while older.as_ptr() != newer_exclusive.as_ptr()
		{
			let node = older.next.get();
			
			free(older.to_non_null());
			
			// TODO: Confirm this is actually possible?
			if node.is_null()
			{
				break
			}
			
			let node = node.to_non_null();
			older = node.static_reference()
		}
	}
	
	#[inline(always)]
	fn identifier(&self) -> NodeIdentifier
	{
		self.identifier.get()
	}
	
	#[inline(always)]
	fn to_non_null(&self) -> NonNull<Self>
	{
		self.as_ptr().to_non_null()
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self
	{
		self as *const _ as *mut _
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
struct WaitFreeQueueInner<Value>
{
	// Called in wfqueue.c code `Ei`.
	// Initially 1.
	enqueue_next_position_index: DoubleCacheAligned<volatile<PositionIndex>>,
	
	// Called in wfqueue.c code `Di`.
	// Initially 1.
	dequeue_next_position_index: DoubleCacheAligned<volatile<PositionIndex>>,
	
	// Index of the head of the queue.
	// Used only for garbage collection of Nodes.
	// Called in wfqueue.c code `Hi`.
	head_of_queue_node_identifier: DoubleCacheAligned<volatile<NodeIdentifier>>,
	// Pointer to the head node of the queue.
	// Used only for garbage collection of Nodes.
	// Called in wfqueue.c code `Hp`.
	head_of_queue_node_pointer: volatile<NonNull<Node<Value>>>,
	
	// A singularly-linked list of per-thread handles, atomically updated.
	// tail is only NULL before the very first PerHyperThreadHandle is created.
	// The list follow the `.next` pointer in each PerHyperThreadHandle.
	// The terminal PerHyperThreadHandle has a `.next` which points to itself.
	tail: volatile<*mut PerHyperThreadHandle<Value>>,
	
	maximum_garbage: MaximumGarbage,
}

impl<Value> WaitFreeQueueInner<Value>
{
	const MaximumPatienceForFastPath: isize = 10;
	
	pub(crate) fn new(maximum_garbage: MaximumGarbage) -> NonNull<Self>
	{
		let mut this = page_size_align_malloc();
		
		{
			let this: &Self = this.reference();
			this.enqueue_next_position_index.set(PositionIndex::InitialNext);
			this.dequeue_next_position_index.set(PositionIndex::InitialNext);
			this.set_initial_head_of_queue_node_identifier();
			this.set_head_of_queue_node_pointer(Node::new_node());
			this.tail.set(null_mut());
		}
		
		{
			let this: &mut Self = this.mutable_reference();
			unsafe { write(&mut this.maximum_garbage, maximum_garbage) };
		}
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn enqueue(&self, this: &PerHyperThreadHandle<Value>, value_to_enqueue: NonNull<Value>)
	{
		assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		this.set_hazard_node_pointer_identifier(this.enqueuer_node_pointer_identifier());
		
		let mut enqueue_position_index = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		while !self.enqueue_fast_path(this, value_to_enqueue, &mut enqueue_position_index) && remaining_patience_for_fast_path.post_decrement() > 0
		{
		}
		if remaining_patience_for_fast_path < 0
		{
			self.enqueue_slow_path(this, value_to_enqueue, enqueue_position_index)
		}
		
		this.set_enqueuer_node_pointer_identifier_using_value_of_node_pointer_identifier_for_node_for_enqueue();
		
		this.rerelease_hazard_node_pointer_identifier()
	}
	
	#[inline(always)]
	pub(crate) fn dequeue(&self, this: &PerHyperThreadHandle<Value>) -> *mut Value
	{
		this.set_hazard_node_pointer_identifier(this.dequeuer_node_pointer_identifier.get());
		
		let mut dequeued_value;
		let mut dequeue_position_index = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		
		do_while!
		{
			do
			{
				dequeued_value = self.dequeue_fast_path(this, &mut dequeue_position_index);
			}
			while dequeued_value.is_top() && remaining_patience_for_fast_path.post_decrement() > 0
		}
		
		if dequeued_value.is_top()
		{
			dequeued_value = self.dequeue_slow_path(this, dequeue_position_index);
		}
		
		// `EMPTY`: a value that will be returned if a `dequeue` fails.
		let EMPTY: *mut Value = 0 as *mut Value;
		if dequeued_value != EMPTY
		{
			self.dequeue_help(this, this.per_hyper_thread_handle_of_next_dequeuer_to_help.get().reference());
			this.per_hyper_thread_handle_of_next_dequeuer_to_help.set(this.per_hyper_thread_handle_of_next_dequeuer_to_help.get().reference().next());
		}
		
		this.dequeuer_node_pointer_identifier.set(this.pointer_to_the_node_for_dequeue.get().reference().identifier.get().to_node_pointer_identifier());
		this.release_hazard_node_pointer_identifier(NodePointerIdentifier::Null);
		
		if this.spare_is_null()
		{
			self.collect_node_garbage_after_dequeue(this);
			this.set_new_spare_node();
		}
		
		dequeued_value
	}
	
	#[inline(always)]
	fn enqueue_fast_path(&self, this: &PerHyperThreadHandle<Value>, value_to_enqueue: NonNull<Value>, enqueue_position_index: &mut PositionIndex) -> bool
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		let index_after_the_next_position_for_enqueue = self.sequentially_consistent_fetch_and_increment_enqueue_next_position_index();
		
		let cell = this.pointer_to_the_node_for_enqueue_reference().find_cell(index_after_the_next_position_for_enqueue, this);
		
		// Works because the initial state of a Cell is zeroed (Node::new_node() does write_bytes).
		let mut compare_to_value = <*mut Value>::Bottom;
		if cell.value.relaxed_relaxed_compare_and_swap(&mut compare_to_value, value_to_enqueue.as_ptr())
		{
			true
		}
		else
		{
			*enqueue_position_index = index_after_the_next_position_for_enqueue;
			false
		}
	}
	
	#[inline(always)]
	fn enqueue_slow_path(&self, this: &PerHyperThreadHandle<Value>, value_to_enqueue: NonNull<Value>, mut enqueue_position_index: PositionIndex)
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		let value_to_enqueue = value_to_enqueue.as_ptr();
		
		let enqueuer = this.enqueue_request.deref();
		enqueuer.value_to_enqueue.set(value_to_enqueue);
		enqueuer.enqueue_position_index.release(enqueue_position_index);

		let tail = this.pointer_to_the_node_for_enqueue_reference();
		let mut index_after_the_next_position_for_enqueue;
		let mut cell;
		
		'do_while: while
		{
			index_after_the_next_position_for_enqueue = self.relaxed_fetch_and_increment_enqueue_next_position_index();
			cell = tail.find_cell(index_after_the_next_position_for_enqueue, this);
			
			let mut expected_enqueuer = <*mut Enqueuer<Value>>::Bottom;
			if cell.enqueuer.sequentially_consistent_compare_and_swap(&mut expected_enqueuer, enqueuer.as_ptr()) && cell.value.get().is_not_top()
			{
				enqueuer.enqueue_position_index.relaxed_relaxed_compare_and_swap(&mut enqueue_position_index, -index_after_the_next_position_for_enqueue);
				break 'do_while;
			}
			enqueuer.enqueue_position_index.get() > PositionIndex::Zero
		}
		{
		}
		
		enqueue_position_index = -enqueuer.enqueue_position_index.get();
		cell = this.pointer_to_the_node_for_enqueue_reference().find_cell(enqueue_position_index, this);
		if enqueue_position_index > index_after_the_next_position_for_enqueue
		{
			let mut index_of_the_next_position_for_enqueue = self.enqueue_next_position_index();
			while index_of_the_next_position_for_enqueue <= enqueue_position_index && !self.relaxed_relaxed_compare_and_swap_enqueue_next_position_index(&mut index_of_the_next_position_for_enqueue, enqueue_position_index.increment())
			{
			}
		}
		cell.value.set(value_to_enqueue);
	}
	
	// Used only when dequeue() is called.
	#[inline(always)]
	fn enqueue_help(&self, this: &PerHyperThreadHandle<Value>, cell: &Cell<Value>, position_index: PositionIndex) -> *mut Value
	{
		#[inline(always)]
		fn spin<Value>(value_holder: &volatile<*mut Value>) -> *mut Value
		{
			const MaximumSpinPatience: usize = 100;
			
			let mut patience = MaximumSpinPatience;
			let mut value = value_holder.get();
			
			while value.is_not_null() && patience.post_decrement() > 0
			{
				value = value_holder.get();
				spin_loop_hint();
			}
			
			value
		}
		
		let mut value = spin(&cell.value);
		
		if (value.is_not_top() && value.is_not_bottom()) || (value.is_bottom() && !cell.value.sequentially_consistent_compare_and_swap(&mut value, <*mut Value>::Top) && value.is_not_top())
		{
			return value;
		}
		
		let mut enqueuer = cell.enqueuer.get();
		
		if enqueuer.is_bottom()
		{
			let mut ph = this.per_hyper_thread_handle_of_next_enqueuer_to_help.get();
			let (mut pe, mut id) =
			{
				let pe = ph.reference().enqueue_request.deref();
				(pe.as_ptr(), pe.enqueue_position_index.get())
			};
			
			if this.ei_is_not_initial_and_is_not_id(id)
			{
				this.reset_ei();
				this.per_hyper_thread_handle_of_next_enqueuer_to_help.set(ph.reference().next());
				
				ph = this.per_hyper_thread_handle_of_next_enqueuer_to_help.get();
				let (pe2, id2) =
				{
					let pe = ph.reference().enqueue_request.deref();
					(pe.as_ptr(), pe.enqueue_position_index.get())
				};
				pe = pe2;
				id = id2;
			}
			
			if id > PositionIndex::Zero && id <= position_index && !cell.enqueuer.relaxed_relaxed_compare_and_swap(&mut enqueuer, pe)
			{
				this.set_ei(id)
			}
			else
			{
				this.per_hyper_thread_handle_of_next_enqueuer_to_help.set(ph.reference().next())
			}
			
			if enqueuer.is_bottom() && cell.enqueuer.relaxed_relaxed_compare_and_swap(&mut enqueuer, <*mut Enqueuer<Value>>::Top)
			{
				enqueuer = <*mut Enqueuer<Value>>::Top
			}
		}
		
		if enqueuer.is_top()
		{
			return if self.enqueue_next_position_index() <= position_index
			{
				<*mut Value>::Bottom
			}
			else
			{
				<*mut Value>::Top
			}
		}
		let non_null_enqueuer = enqueuer.to_non_null();
		let enqueuer = non_null_enqueuer.reference();
		
		let mut enqueue_position_index = enqueuer.enqueue_position_index.acquire();
		let value_to_enqueue = enqueuer.value_to_enqueue.acquire();
		
		if enqueue_position_index > position_index
		{
			if cell.value.get().is_top() && self.enqueue_next_position_index() <= position_index
			{
				return <*mut Value>::Bottom
			}
		}
		else
		{
			if (enqueue_position_index > PositionIndex::Zero && enqueuer.enqueue_position_index.relaxed_relaxed_compare_and_swap(&mut enqueue_position_index, -position_index)) || (enqueue_position_index == -position_index && cell.value.get().is_top())
			{
				let mut index_of_the_next_position_for_enqueue = self.enqueue_next_position_index();
				while index_of_the_next_position_for_enqueue <= position_index && !self.relaxed_relaxed_compare_and_swap_enqueue_next_position_index(&mut index_of_the_next_position_for_enqueue, position_index.increment())
				{
				}
				cell.value.set(value_to_enqueue);
			}
		}
		
		cell.value.get()
	}
	
	#[inline(always)]
	fn dequeue_fast_path(&self, this: &PerHyperThreadHandle<Value>, position_index: &mut PositionIndex) -> *mut Value
	{
		let index_after_the_next_position_for_dequeue = self.sequentially_consistent_fetch_and_increment_dequeue_next_position_index();
		let cell = this.pointer_to_the_node_for_dequeue.find_cell(index_after_the_next_position_for_dequeue, this);
		let dequeued_value = self.enqueue_help(this, cell, index_after_the_next_position_for_dequeue);
		
		if dequeued_value.is_bottom()
		{
			return <*mut Value>::Bottom
		}
		
		let mut cd = <*mut Dequeuer>::Bottom;
		if dequeued_value.is_not_top() && cell.dequeuer.relaxed_relaxed_compare_and_swap(&mut cd, <*mut Dequeuer>::Top)
		{
			return dequeued_value
		}
		
		*position_index = index_after_the_next_position_for_dequeue;
		<*mut Value>::Top
	}
	
	#[inline(always)]
	fn dequeue_slow_path(&self, this: &PerHyperThreadHandle<Value>, position_index: PositionIndex) -> *mut Value
	{
		let dequeuer = this.dequeue_request.deref();
		dequeuer.dequeue_position_index.release(position_index);
		dequeuer.dequeue_position_index_x.release(position_index);
		
		self.dequeue_help(this, this);
		let position_index = -dequeuer.dequeue_position_index_x.get();
		let cell = this.pointer_to_the_node_for_dequeue.find_cell(position_index, this);
		let dequeued_value = cell.value.get();
		
		if dequeued_value.is_top()
		{
			<*mut Value>::Bottom
		}
		else
		{
			dequeued_value
		}
	}
	
	#[inline(always)]
	fn dequeue_help(&self, this: &PerHyperThreadHandle<Value>, other: &PerHyperThreadHandle<Value>)
	{
		let dequeuer = other.dequeue_request.deref();
		let mut idx = dequeuer.dequeue_position_index_x.acquire();
		let id = dequeuer.dequeue_position_index.get();
		
		if idx < id
		{
			return;
		}
		
		// ie, Read the value, then construct a new volatile reference used for `find_cell`.
		// NOTE: This is internally mutable, and calls to `find_cell` will mutate it.
		let Dp = volatile::new(other.pointer_to_the_node_for_dequeue.get());
		this.set_hazard_node_pointer_identifier(other.hazard_node_pointer_identifier());
		sequentially_consistent_fence();
		idx = dequeuer.dequeue_position_index_x.get();
		
		let mut position_index = id.increment();
		let mut old_id = id;
		let mut new_id = PositionIndex::Zero;
		
		loop
		{
			// NOTE: This is internally mutable, and calls to `find_cell` will mutate it.
			let h = volatile::new(Dp.get());
			
			while idx == old_id && new_id.is_zero()
			{
				let cell = h.find_cell(position_index, this);
				
				let mut index_of_the_next_position_for_dequeue = self.dequeue_next_position_index();
				while index_of_the_next_position_for_dequeue <= position_index && !self.relaxed_relaxed_compare_and_swap_dequeue_next_position_index(&mut index_of_the_next_position_for_dequeue, position_index.increment())
				{
				}
				
				let value = self.enqueue_help(this, cell, position_index);
				if value.is_bottom() || (value.is_not_top() && cell.dequeuer.get().is_bottom())
				{
					new_id = position_index;
				}
				else
				{
					idx = dequeuer.dequeue_position_index_x.acquire();
				}
				
				
				position_index.increment_in_place();
			}
			
			if new_id.is_not_zero()
			{
				if dequeuer.dequeue_position_index_x.release_acquire_compare_and_swap(&mut idx, new_id)
				{
					idx = new_id;
				}
				if idx >= new_id
				{
					new_id = PositionIndex::Zero;
				}
			}
			
			if idx < PositionIndex::Zero || dequeuer.dequeue_position_index.get() != id
			{
				break;
			}
			
			let cell = Dp.find_cell(idx, this);
			let mut cd = <*mut Dequeuer>::Bottom;
			if cell.value.get().is_top() || cell.dequeuer.relaxed_relaxed_compare_and_swap(&mut cd, dequeuer.as_ptr()) || cd == dequeuer.as_ptr()
			{
				let negative_idx = -idx;
				dequeuer.dequeue_position_index_x.relaxed_relaxed_compare_and_swap(&mut idx, negative_idx);
				break
			}
			
			old_id = idx;
			if idx >= position_index
			{
				position_index = idx.increment();
			}
		}
	}
	
	#[inline(always)]
	fn collect_node_garbage_after_dequeue(&self, this: &PerHyperThreadHandle<Value>)
	{
		let old_head_of_queue_node_identifier = self.acquire_head_of_queue_node_identifier();
		
		if old_head_of_queue_node_identifier.there_is_no_garbage_to_collect()
		{
			return;
		}
		
		let new = this.pointer_to_the_node_for_dequeue.get();
		
		if old_head_of_queue_node_identifier.there_is_not_yet_enough_garbage_to_collect(new, self.maximum_garbage)
		{
			return;
		}
		
		// Try to 'grab a lock' on the garbage nodes to collect.
		// This isn't a lock as such, but sets a special sentinel value (`NodeIdentifier::NoHeadOfQueue`) that tells other threads there is no garbage to collect (see `old_head_of_queue_node_identifier.there_is_no_garbage_to_collect()` above).
		if self.return_true_if_could_not_grab_head_of_queue_node_identifier(old_head_of_queue_node_identifier)
		{
			// Did not grab lock because someone else did - and they'll collect the node garbage.
			return;
		}
		// Once the 'lock' (sentinel) is released all garbage Nodes are free'd.
		// The 'lock' (sentinel) is released when `self.release_head_of_queue_node_identifier()` is called below.
		
		let old_head_of_queue_node = self.head_of_queue_node_pointer();
		
		let newer_head_of_queue_node = AllPerHyperThreadHandles::check_and_update_all_hyper_thread_handle_hazard_pointers(this, new, old_head_of_queue_node, old_head_of_queue_node_identifier);
		let new_head_of_queue_node_identifier = newer_head_of_queue_node.reference().identifier();
		
		if new_head_of_queue_node_identifier > old_head_of_queue_node_identifier
		{
			self.set_head_of_queue_node_pointer(newer_head_of_queue_node);
			
			self.release_head_of_queue_node_identifier(new_head_of_queue_node_identifier);
			
			old_head_of_queue_node.reference().free_garbage_nodes_excluding_upto(newer_head_of_queue_node)
		}
		else
		{
			self.release_head_of_queue_node_identifier(old_head_of_queue_node_identifier)
		}
	}
	
	#[inline(always)]
	fn enqueue_next_position_index(&self) -> PositionIndex
	{
		self.enqueue_next_position_index.get()
	}
	
	#[inline(always)]
	fn sequentially_consistent_fetch_and_increment_enqueue_next_position_index(&self) -> PositionIndex
	{
		self.enqueue_next_position_index.sequentially_consistent_fetch_and_increment()
	}
	
	#[inline(always)]
	fn relaxed_fetch_and_increment_enqueue_next_position_index(&self) -> PositionIndex
	{
		self.enqueue_next_position_index.relaxed_fetch_and_increment()
	}
	
	#[inline(always)]
	fn relaxed_relaxed_compare_and_swap_enqueue_next_position_index(&self, compare: &mut PositionIndex, value: PositionIndex) -> bool
	{
		self.enqueue_next_position_index.relaxed_relaxed_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	fn dequeue_next_position_index(&self) -> PositionIndex
	{
		self.dequeue_next_position_index.get()
	}
	
	#[inline(always)]
	fn sequentially_consistent_fetch_and_increment_dequeue_next_position_index(&self) -> PositionIndex
	{
		self.dequeue_next_position_index.sequentially_consistent_fetch_and_increment()
	}
	
	#[inline(always)]
	fn relaxed_relaxed_compare_and_swap_dequeue_next_position_index(&self, compare: &mut PositionIndex, value: PositionIndex) -> bool
	{
		self.dequeue_next_position_index.relaxed_relaxed_compare_and_swap(compare, value)
	}
	
	#[inline(always)]
	fn acquire_head_of_queue_node_identifier(&self) -> NodeIdentifier
	{
		self.head_of_queue_node_identifier.acquire()
	}
	
	#[inline(always)]
	fn return_true_if_could_not_grab_head_of_queue_node_identifier(&self, old_head_of_queue_node_identifier: NodeIdentifier) -> bool
	{
		!self.head_of_queue_node_identifier.acquire_relaxed_compare_and_swap_do_not_mutate_compare(old_head_of_queue_node_identifier, NodeIdentifier::NoHeadOfQueue)
	}
	
	#[inline(always)]
	fn release_head_of_queue_node_identifier(&self, node_identifier: NodeIdentifier)
	{
		self.head_of_queue_node_identifier.release(node_identifier)
	}
	
	#[inline(always)]
	fn set_initial_head_of_queue_node_identifier(&self)
	{
		self.head_of_queue_node_identifier.set(NodeIdentifier::Initial)
	}
	
	#[inline(always)]
	fn set_head_of_queue_node_pointer(&self, node_pointer: NonNull<Node<Value>>)
	{
		self.head_of_queue_node_pointer.set(node_pointer)
	}
	
	#[inline(always)]
	fn head_of_queue_node_pointer(&self) -> NonNull<Node<Value>>
	{
		self.head_of_queue_node_pointer.get()
	}
	
	#[inline(always)]
	fn tail(&self) -> *mut PerHyperThreadHandle<Value>
	{
		self.tail.get()
	}
	
	#[inline(always)]
	fn try_to_change_tail(&self, tail: &mut *mut PerHyperThreadHandle<Value>, per_hyper_thread_handle_non_null: NonNull<PerHyperThreadHandle<Value>>) -> bool
	{
		self.tail.release_acquire_compare_and_swap(tail, per_hyper_thread_handle_non_null.as_ptr())
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumberOfHyperThreads(u16);

impl NumberOfHyperThreads
{
	pub const InclusiveMaximumNumberOfHyperThreads: usize = 256;
	
	/// Panics if `number_of_hyper_threads` is 0 or exceeds InclusiveMaximumNumberOfHyperThreads.
	#[inline(always)]
	pub fn new(number_of_hyper_threads: u16) -> Self
	{
		assert_ne!(number_of_hyper_threads, 0, "number_of_hyper_threads can not be zero");
		assert!((number_of_hyper_threads as usize) <= Self::InclusiveMaximumNumberOfHyperThreads, "number_of_hyper_threads '{}' exceeds Self::InclusiveMaximumNumberOfHyperThreads '{}'", number_of_hyper_threads, Self::InclusiveMaximumNumberOfHyperThreads);
		
		NumberOfHyperThreads(number_of_hyper_threads)
	}
	
	#[inline(always)]
	pub fn maximum_garbage(&self) -> MaximumGarbage
	{
		let maximum_garbage = 2 * (self.0 as usize);
		debug_assert!(maximum_garbage <= ::std::isize::MAX as usize, "maximum_garbage exceeds isize::MAX");
		MaximumGarbage(maximum_garbage as isize)
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MaximumGarbage(isize);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NodePointerIdentifier(usize);

impl NodePointerIdentifier
{
	const Null: NodePointerIdentifier = NodePointerIdentifier(!0);
	
	#[inline(always)]
	fn from_node_identifier(node_identifier: NodeIdentifier) -> Self
	{
		NodePointerIdentifier(node_identifier.0 as usize)
	}
	
	#[inline(always)]
	fn check<Value>(self, mut current: NonNull<Node<Value>>, old_head_of_queue_node: NonNull<Node<Value>>) -> NonNull<Node<Value>>
	{
		if self < current.reference().identifier().to_node_pointer_identifier()
		{
			let mut node = old_head_of_queue_node;
			
			while node.reference().identifier().to_node_pointer_identifier() < self
			{
				node = node.reference().next.get().to_non_null();
			}
			
			current = node;
		}
		
		current
	}
}

struct PerHyperThreadHandle<Value>
{
	// Pointer to the next thread handle; a singularly linked list.
	// Can pointer to self if it is the last element in the singularly linked list.
	// The very first element in the list is pointed to by the value of `.tail` in WaitFreeQueueInner.
	next: ExtendedNonNullAtomicPointer<PerHyperThreadHandle<Value>>,
	
	hazard_node_pointer_identifier: volatile<NodePointerIdentifier>,
	
	pointer_to_the_node_for_enqueue: volatile<NonNull<Node<Value>>>,
	enqueuer_node_pointer_identifier: CopyCell<NodePointerIdentifier>,
	
	pointer_to_the_node_for_dequeue: volatile<NonNull<Node<Value>>>,
	dequeuer_node_pointer_identifier: CopyCell<NodePointerIdentifier>,
	
	enqueue_request: CacheAligned<Enqueuer<Value>>,
	
	dequeue_request: CacheAligned<Dequeuer>,
	
	per_hyper_thread_handle_of_next_enqueuer_to_help: CacheAligned<CopyCell<NonNull<PerHyperThreadHandle<Value>>>>,
	
	// Use only by `enqueue_help()`.
	// Initially 0.
	// Was `Ei`.
	enqueue_next_position_index: CopyCell<PositionIndex>,
	
	per_hyper_thread_handle_of_next_dequeuer_to_help: CopyCell<NonNull<PerHyperThreadHandle<Value>>>,
	
	// Pointer to a spare node to use, to speedup adding a new node.
	spare: CacheAligned<CopyCell<*mut Node<Value>>>,
}

impl<Value> PerHyperThreadHandle<Value>
{
	// A combination of `thread_init` and `queue_init`.
	pub(crate) fn new(queue: NonNull<WaitFreeQueueInner<Value>>) -> NonNull<Self>
	{
		let wait_free_queue_inner = queue.reference();
		
		let per_hyper_thread_handle_non_null = page_size_align_malloc();
		
		{
			let this: &PerHyperThreadHandle<Value> = per_hyper_thread_handle_non_null.reference();
			
			// Seems to be unnecessary as this value is always overwritten by `self.add_to_singularly_linked_list_of_per_hyper_thread_handles()`
			this.initialize_next();
			
			this.reset_hazard_node_pointer_identifier();
			
			this.set_pointer_to_the_node_for_enqueue(wait_free_queue_inner.head_of_queue_node_pointer());
			this.set_enqueuer_node_pointer_identifier_using_value_of_node_pointer_identifier_for_node_for_enqueue();
			
			this.pointer_to_the_node_for_dequeue.set(wait_free_queue_inner.head_of_queue_node_pointer());
			this.dequeuer_node_pointer_identifier.set(this.node_pointer_identifier_for_node_for_enqueue());
			
			this.enqueue_request.deref().initialize();
			
			this.dequeue_request.deref().initialize();
			
			this.reset_ei();
			
			this.initialize_spare_node();
			
			this.add_to_singularly_linked_list_of_per_hyper_thread_handles(wait_free_queue_inner, per_hyper_thread_handle_non_null);
			
			this.initialize_next_enqueuer_and_dequeuer_to_help();
		}
		
		per_hyper_thread_handle_non_null
	}
	
	#[inline(always)]
	fn add_to_singularly_linked_list_of_per_hyper_thread_handles(&self, wait_free_queue_inner: &WaitFreeQueueInner<Value>, per_hyper_thread_handle_non_null: NonNull<Self>)
	{
		let mut tail = wait_free_queue_inner.tail();
		
		if tail.is_null()
		{
			self.set_next(per_hyper_thread_handle_non_null);
			if wait_free_queue_inner.try_to_change_tail(&mut tail, per_hyper_thread_handle_non_null)
			{
				return
			}
			// NOTE: tail will have been updated by CASra; queue.tail will not longer have been null, hence tail will now no longer be null, so fall through to logic below.
		}
		let tail_non_null = tail.to_non_null();
		let tail = tail_non_null.reference();
		
		let mut next = tail.next();
		do_while!
		{
			do
			{
				self.set_next(next)
			}
			while !tail.next.CASra(&mut next, per_hyper_thread_handle_non_null)
		}
	}
	
	#[inline(always)]
	fn initialize_next(&self)
	{
		self.set_next(NonNull::dangling())
	}
	
	#[inline(always)]
	fn next(&self) -> NonNull<PerHyperThreadHandle<Value>>
	{
		self.next.get()
	}
	
	#[inline(always)]
	fn set_next(&self, next: NonNull<PerHyperThreadHandle<Value>>)
	{
		self.next.set(next)
	}
	
	#[inline(always)]
	fn initialize_next_enqueuer_and_dequeuer_to_help(&self)
	{
		debug_assert_ne!(self.next().as_ptr(), NonNull::dangling().as_ptr(), "self.next should have been set to something other than dangling");
		
		self.per_hyper_thread_handle_of_next_enqueuer_to_help.set(self.next());
		self.per_hyper_thread_handle_of_next_dequeuer_to_help.set(self.next());
	}
	
	#[inline(always)]
	fn rerelease_hazard_node_pointer_identifier(&self)
	{
		self.hazard_node_pointer_identifier.release(NodePointerIdentifier::Null)
	}
	
	#[inline(always)]
	fn release_hazard_node_pointer_identifier(&self, node_pointer_identifier: NodePointerIdentifier)
	{
		self.hazard_node_pointer_identifier.release(node_pointer_identifier)
	}
	
	#[inline(always)]
	fn reset_hazard_node_pointer_identifier(&self)
	{
		self.set_hazard_node_pointer_identifier(NodePointerIdentifier::Null)
	}
	
	#[inline(always)]
	fn set_hazard_node_pointer_identifier(&self, node_pointer_identifier: NodePointerIdentifier)
	{
		self.hazard_node_pointer_identifier.set(node_pointer_identifier)
	}
	
	#[inline(always)]
	fn hazard_node_pointer_identifier(&self) -> NodePointerIdentifier
	{
		self.hazard_node_pointer_identifier.get()
	}
	
	#[inline(always)]
	fn pointer_to_the_node_for_enqueue_reference(&self) -> &volatile<NonNull<Node<Value>>>
	{
		&self.pointer_to_the_node_for_enqueue
	}
	
	#[inline(always)]
	fn set_pointer_to_the_node_for_enqueue(&self, pointer_to_the_node_for_enqueue: NonNull<Node<Value>>)
	{
		self.pointer_to_the_node_for_enqueue.set(pointer_to_the_node_for_enqueue)
	}
	
	#[inline(always)]
	fn node_pointer_identifier_for_node_for_enqueue(&self) -> NodePointerIdentifier
	{
		self.pointer_to_the_node_for_enqueue.get().reference().identifier().to_node_pointer_identifier()
	}
	
	#[inline(always)]
	fn set_enqueuer_node_pointer_identifier_using_value_of_node_pointer_identifier_for_node_for_enqueue(&self)
	{
		self.enqueuer_node_pointer_identifier.set(self.node_pointer_identifier_for_node_for_enqueue())
	}
	
	#[inline(always)]
	fn enqueuer_node_pointer_identifier(&self) -> NodePointerIdentifier
	{
		self.enqueuer_node_pointer_identifier.get()
	}
	
	#[inline(always)]
	fn ei_is_not_initial_and_is_not_id(&self, id: PositionIndex) -> bool
	{
		let ei = self.enqueue_next_position_index.get();
		ei != PositionIndex::Zero && ei != id
	}
	
	#[inline(always)]
	fn set_ei(&self, ei: PositionIndex)
	{
		self.enqueue_next_position_index.set(ei);
	}
	
	#[inline(always)]
	fn reset_ei(&self)
	{
		self.enqueue_next_position_index.set(PositionIndex::Zero)
	}
	
	#[inline(always)]
	fn get_non_null_spare_node(&self) -> NonNull<Node<Value>>
	{
		let spare = self.spare();
		if spare.is_not_null()
		{
			spare.to_non_null()
		}
		else
		{
			self.set_new_spare_node()
		}
	}
	
	// This is only the case if the spare has been assigned into the queue, in which case, there might be garbage to collect.
	#[inline(always)]
	fn spare_is_null(&self) -> bool
	{
		self.spare().is_null()
	}
	
	#[inline(always)]
	fn initialize_spare_node(&self)
	{
		self.spare.set(null_mut());
		self.set_new_spare_node();
	}
	
	#[inline(always)]
	fn set_new_spare_node(&self) -> NonNull<Node<Value>>
	{
		// self.spare should be null EXCEPT when initially allocating.
		debug_assert!(self.spare_is_null(), "trying to set spare to a new node but self.spare is not null");
		
		let new_spare_node = Node::new_node();
		self.spare.set(new_spare_node.as_ptr());
		new_spare_node
	}
	
	#[inline(always)]
	fn set_spare_to_null(&self)
	{
		debug_assert!(self.spare.get().is_not_null(), "trying to set spare to null but self.spare is already null");
		
		self.spare.set(null_mut());
	}
	
	// Can be null, but never for long.
	#[inline(always)]
	fn spare(&self) -> *mut Node<Value>
	{
		self.spare.get()
	}
	
	#[inline(always)]
	fn as_non_null(&self) -> NonNull<Self>
	{
		unsafe { NonNull::new_unchecked(self.as_ptr()) }
	}
	
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self
	{
		self as *const _ as *mut _
	}
}
