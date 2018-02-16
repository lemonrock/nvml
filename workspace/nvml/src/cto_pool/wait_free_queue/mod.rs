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

// A memory fence to ensure sequential consistency.
#[inline(always)]
fn FENCE()
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
}

impl DoubleCacheAligned<volatile<isize>>
{
	const Increment: isize = 1;
	
	#[inline(always)]
	pub(crate) fn relaxed_fetch_and_increment(&self) -> isize
	{
		self.relaxed_fetch_and_add(Self::Increment)
	}
	
	#[inline(always)]
	pub(crate) fn sequentially_consistent_fetch_and_increment(&self) -> isize
	{
		self.sequentially_consistent_fetch_and_add(Self::Increment)
	}
}

#[derive(Debug)]
pub(crate) struct volatile<T: Copy>(UnsafeCell<T>);

impl volatile<NodePointerIdentifier>
{
	#[inline(always)]
	fn check<Value>(&self, current: NonNull<Node<Value>>, old: *mut Node<Value>) -> NonNull<Node<Value>>
	{
		let hazard_node_pointer_identifier = self.acquire();
		hazard_node_pointer_identifier.check(current, old)
	}
}

impl<Value> volatile<NonNull<Node<Value>>>
{
	#[inline(always)]
	fn update(&self, mut current: NonNull<Node<Value>>, hazard_node_pointer_identifier: &volatile<NodePointerIdentifier>, old: *mut Node<Value>) -> NonNull<Node<Value>>
	{
		let mut node = self.acquire();
		
		if node.identifier() < current.identifier()
		{
			if !self.sequentially_consistent_compare_and_swap(&mut node, current)
			{
				if node.identifier() < current.identifier()
				{
					current = node;
				}
			}
			
			current = hazard_node_pointer_identifier.check(current, old);
		}
		
		current
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
	id: volatile<isize>,
	value: volatile<*mut Value>,
}

impl<Value> Enqueuer<Value>
{
	#[inline(always)]
	fn initialize(&self)
	{
		self.id.set(0);
		self.value.set(<*mut Value>::Bottom);
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
	id: volatile<isize>,
	idx: volatile<isize>,
}

impl Dequeuer
{
	#[inline(always)]
	fn initialize(&self)
	{
		self.id.set(0);
		self.idx.set(-1);
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
	
	const SignedNumberOfCellsInANode: isize = Self::NumberOfCellsInANode as isize;
	
	#[inline(always)]
	pub(crate) fn get_cell(&self, cell_index: isize) -> &Cell<Value>
	{
		debug_assert!(cell_index >= 0, "cell_index is negative");
		debug_assert!(cell_index < Self::SignedNumberOfCellsInANode, "cell_index is not less than SignedNumberOfCellsInANode");
		unsafe { self.0.get_unchecked(cell_index as usize) }
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
	fn check_and_update_all_hyper_thread_handle_hazard_pointers(our_per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, mut new: NonNull<Node<Value>>, old: *mut Node<Value>, old_head_of_queue_node_identifier: NodeIdentifier) -> NonNull<Node<Value>>
	{
		let mut all_per_hyper_thread_handles = AllPerHyperThreadHandles::new();
		let mut our_or_another_threads_per_hyper_thread_handle = our_per_hyper_thread_handle;
		do_while!
		{
			do
			{
				{
					let reference = our_or_another_threads_per_hyper_thread_handle.reference();
					let hazard_node_pointer_identifier = &reference.hazard_node_pointer_identifier;
				
					new = hazard_node_pointer_identifier.check(new, old);
					new = reference.pointer_to_the_node_for_enqueue.update(new, hazard_node_pointer_identifier, old);
					new = reference.pointer_to_the_node_for_dequeue.update(new, hazard_node_pointer_identifier, old);
					
					all_per_hyper_thread_handles.set(our_or_another_threads_per_hyper_thread_handle);
				}
				our_or_another_threads_per_hyper_thread_handle = our_or_another_threads_per_hyper_thread_handle.reference().next.get();
			}
			while new.identifier() > old_head_of_queue_node_identifier && our_or_another_threads_per_hyper_thread_handle.as_ptr() != our_per_hyper_thread_handle.as_ptr()
		}
		all_per_hyper_thread_handles.check(&mut new, old, old_head_of_queue_node_identifier);
		new
	}
	
	#[inline(always)]
	fn set(&mut self, another_wait_free_queue_per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>)
	{
		let index = self.index.post_increment();
		
		*(unsafe { self.per_thread_handles.get_unchecked_mut(index as usize) }) = another_wait_free_queue_per_hyper_thread_handle;
	}
	
	#[inline(always)]
	fn check(&mut self, new: &mut NonNull<Node<Value>>, old: *mut Node<Value>, old_head_of_queue_node_identifier: NodeIdentifier)
	{
		while new.identifier() > old_head_of_queue_node_identifier && self.pre_decrement_index_is_not_negative()
		{
			*new = self.get_hazard_pointer_identifier().check(*new, old);
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
		let element = unsafe { &* element.as_ptr() };
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
	fn to_node_identifier(self) -> NodePointerIdentifier
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
		(pointer_to_the_node_for_dequeue.identifier().0 - self.0) < maximum_garbage.0
	}
	
	#[inline(always)]
	fn increment(self) -> Self
	{
		NodeIdentifier(self.0 + 1)
	}
	
	#[inline(always)]
	fn increment_in_place(&mut self)
	{
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
	
	fn find_cell<'node>(pointer_to_node: &'node volatile<NonNull<Node<Value>>>, i: isize, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>) -> &'node Cell<Value>
	{
		let mut current = pointer_to_node.get();
		
		let mut current_node_identifier = current.reference().identifier.get();
		let current_maximum_node_identifier = NodeIdentifier(i / Cells::<Value>::SignedNumberOfCellsInANode);
		while current_node_identifier < current_maximum_node_identifier
		{
			let mut next = current.reference().next.get();
			
			if next.is_null()
			{
				let spare_node_to_use_for_next = per_hyper_thread_handle.reference().get_non_null_spare_node();
				spare_node_to_use_for_next.reference().identifier.set(current_node_identifier.increment());
				
				if current.reference().next.release_acquire_compare_and_swap(&mut next, spare_node_to_use_for_next.as_ptr())
				{
					next = spare_node_to_use_for_next.as_ptr();
					per_hyper_thread_handle.reference().set_spare_to_null();
				}
			}
			
			current = next.to_non_null();
			
			current_node_identifier.increment_in_place();
		}
		
		pointer_to_node.set(current);
		
		let cell_index = i % Cells::<Value>::SignedNumberOfCellsInANode;
		let borrow_checker_hack = unsafe { &*current.as_ptr() };
		borrow_checker_hack.get_cell(cell_index)
	}
	
	#[inline(always)]
	fn get_cell(&self, cell_index: isize) -> &Cell<Value>
	{
		&self.cells.get_cell(cell_index)
	}
	
	#[inline(always)]
	fn free_garbage_nodes(mut old: *mut Node<Value>, new: NonNull<Node<Value>>)
	{
		while old != new.as_ptr()
		{
			let old_non_null = old.to_non_null();
			let node = old_non_null.reference().next.get();
			free(old_non_null);
			old = node;
		}
	}
	
	#[inline(always)]
	fn identifier(&self) -> NodeIdentifier
	{
		self.identifier.get()
	}
}

trait NodeNonNull<T>: ExtendedNonNull<T>
{
	#[inline(always)]
	fn identifier(self) -> NodeIdentifier;
}

impl<Value> NodeNonNull<Node<Value>> for NonNull<Node<Value>>
{
	#[inline(always)]
	fn identifier(self) -> NodeIdentifier
	{
		self.reference().identifier()
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
struct WaitFreeQueueInner<Value>
{
	// Called in wfqueue.c code `Ei`.
	// Initially 1.
	index_of_the_next_position_for_enqueue: DoubleCacheAligned<volatile<isize>>,
	
	// Called in wfqueue.c code `Di`.
	// Initially 1.
	index_of_the_next_position_for_dequeue: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the head of the queue.
	// Used only for garbage collection of Nodes.
	// Called in wfqueue.c code `Hi`.
	head_of_queue_node_identifier: DoubleCacheAligned<volatile<NodeIdentifier>>,
	
	// Pointer to the head node of the queue.
	// When the queue is initially created, is not null.
	// Called in wfqueue.c code `Hp`.
	pointer_to_the_head_node: volatile<*mut Node<Value>>,
	
	maximum_garbage: MaximumGarbage,
	
	// A singularly-linked list of per-thread handles, atomically updated.
	// tail is only NULL before the very first PerHyperThreadHandle is created.
	// The list follow the `.next` pointer in each PerHyperThreadHandle.
	// The terminal PerHyperThreadHandle has a `.next` which points to itself.
	tail: volatile<*mut PerHyperThreadHandle<Value>>,
}

impl<Value> WaitFreeQueueInner<Value>
{
	const MaximumPatienceForFastPath: isize = 10;
	
	const InitialNextPositionIndex: isize = 1;
	
	pub(crate) fn new(maximum_garbage: MaximumGarbage) -> NonNull<Self>
	{
		let mut this = page_size_align_malloc();
		
		{
			let this: &Self = this.reference();
			this.head_of_queue_node_identifier.set(NodeIdentifier::Initial);
			this.pointer_to_the_head_node.set(Node::new_node().as_ptr());
			this.index_of_the_next_position_for_enqueue.set(Self::InitialNextPositionIndex);
			this.index_of_the_next_position_for_dequeue.set(Self::InitialNextPositionIndex);
			this.tail.set(null_mut());
		}
		
		{
			let this: &mut Self = this.mutable_reference();
			unsafe { write(&mut this.maximum_garbage, maximum_garbage) };
		}
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn enqueue(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, value_to_enqueue: NonNull<Value>)
	{
		assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.set(per_hyper_thread_handle.reference().enqueuer_node_pointer_identifier.get());
		
		let mut enqueue_index = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		while !self.enqueue_fast_path(per_hyper_thread_handle, value_to_enqueue, &mut enqueue_index) && remaining_patience_for_fast_path.post_decrement() > 0
		{
		}
		if remaining_patience_for_fast_path < 0
		{
			self.enqueue_slow_path(per_hyper_thread_handle, value_to_enqueue, enqueue_index)
		}
		
		per_hyper_thread_handle.reference().enqueuer_node_pointer_identifier.set(per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue.get().reference().identifier.get().to_node_identifier());
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.release(NodePointerIdentifier::Null)
	}
	
	#[inline(always)]
	pub(crate) fn dequeue(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>) -> *mut Value
	{
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.set(per_hyper_thread_handle.reference().dequeuer_node_pointer_identifier.get());
		
		let mut dequeued_value;
		let mut id = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		
		do_while!
		{
			do
			{
				dequeued_value = self.dequeue_fast_path(per_hyper_thread_handle, &mut id);
			}
			while dequeued_value.is_top() && remaining_patience_for_fast_path.post_decrement() > 0
		}
		
		if dequeued_value.is_top()
		{
			dequeued_value = self.dequeue_slow_path(per_hyper_thread_handle, id);
		}
		
		// `EMPTY`: a value that will be returned if a `dequeue` fails.
		let EMPTY: *mut Value = 0 as *mut Value;
		if dequeued_value != EMPTY
		{
			self.dequeue_help(per_hyper_thread_handle, per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_dequeuer_to_help.get());
			per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_dequeuer_to_help.set(per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_dequeuer_to_help.get().reference().next());
		}
		
		per_hyper_thread_handle.reference().dequeuer_node_pointer_identifier.set(per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue.get().reference().identifier.get().to_node_identifier());
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.release(NodePointerIdentifier::Null);
		
		if per_hyper_thread_handle.reference().spare_is_null()
		{
			self.collect_node_garbage_after_dequeue(per_hyper_thread_handle);
			per_hyper_thread_handle.reference().set_new_spare_node();
		}
		
		dequeued_value
	}
	
	#[inline(always)]
	fn enqueue_fast_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, value_to_enqueue: NonNull<Value>, enqueue_index: &mut isize) -> bool
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		let index_after_the_next_position_for_enqueue = self.index_of_the_next_position_for_enqueue.sequentially_consistent_fetch_and_increment();
		
		let cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue, index_after_the_next_position_for_enqueue, per_hyper_thread_handle);
		
		// Works because the initial state of a Cell is zeroed (Node::new_node() does write_bytes).
		let mut compare_to_value = <*mut Value>::Bottom;
		if cell.value.relaxed_relaxed_compare_and_swap(&mut compare_to_value, value_to_enqueue.as_ptr())
		{
			true
		}
		else
		{
			*enqueue_index = index_after_the_next_position_for_enqueue;
			false
		}
	}
	
	#[inline(always)]
	fn enqueue_slow_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, value_to_enqueue: NonNull<Value>, mut enqueue_index: isize)
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		let value_to_enqueue = value_to_enqueue.as_ptr();
		
		let enqueuer = per_hyper_thread_handle.reference().enqueue_request.deref();
		enqueuer.value.set(value_to_enqueue);
		enqueuer.id.release(enqueue_index);

		let tail = &per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue;
		let mut index_after_the_next_position_for_enqueue;
		let mut cell;
		
		'do_while: while
		{
			index_after_the_next_position_for_enqueue = self.index_of_the_next_position_for_enqueue.relaxed_fetch_and_increment();
			cell = Node::find_cell(tail, index_after_the_next_position_for_enqueue, per_hyper_thread_handle);
			
			let mut expected_enqueuer = <*mut Enqueuer<Value>>::Bottom;
			if cell.enqueuer.sequentially_consistent_compare_and_swap(&mut expected_enqueuer, enqueuer.as_ptr()) && cell.value.get().is_not_top()
			{
				enqueuer.id.relaxed_relaxed_compare_and_swap(&mut enqueue_index, -index_after_the_next_position_for_enqueue);
				break 'do_while;
			}
			enqueuer.id.get() > 0
		}
		{
		}
		
		enqueue_index = -enqueuer.id.get();
		cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue, enqueue_index, per_hyper_thread_handle);
		if enqueue_index > index_after_the_next_position_for_enqueue
		{
			let mut index_of_the_next_position_for_enqueue = self.index_of_the_next_position_for_enqueue.get();
			while index_of_the_next_position_for_enqueue <= enqueue_index && !self.index_of_the_next_position_for_enqueue.relaxed_relaxed_compare_and_swap(&mut index_of_the_next_position_for_enqueue, enqueue_index + 1)
			{
			}
		}
		cell.value.set(value_to_enqueue);
	}
	
	// Used only when dequeue() is called.
	#[inline(always)]
	fn enqueue_help(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, cell: &Cell<Value>, i: isize) -> *mut Value
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
			let mut ph = per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_enqueuer_to_help.get();
			let (mut pe, mut id) =
			{
				let pe = ph.reference().enqueue_request.deref();
				(pe.as_ptr(), pe.id.get())
			};
			
			if per_hyper_thread_handle.reference().ei_is_not_initial_and_is_not_id(id)
			{
				per_hyper_thread_handle.reference().reset_ei();
				per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_enqueuer_to_help.set(ph.reference().next());
				
				ph = per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_enqueuer_to_help.get();
				let (pe2, id2) =
				{
					let pe = ph.reference().enqueue_request.deref();
					(pe.as_ptr(), pe.id.get())
				};
				pe = pe2;
				id = id2;
			}
			
			if id > 0 && id <= i && !cell.enqueuer.relaxed_relaxed_compare_and_swap(&mut enqueuer, pe)
			{
				per_hyper_thread_handle.reference().set_ei(id)
			}
			else
			{
				per_hyper_thread_handle.reference().per_hyper_thread_handle_of_next_enqueuer_to_help.set(ph.reference().next())
			}
			
			if enqueuer.is_bottom() && cell.enqueuer.relaxed_relaxed_compare_and_swap(&mut enqueuer, <*mut Enqueuer<Value>>::Top)
			{
				enqueuer = <*mut Enqueuer<Value>>::Top
			}
		}
		
		if enqueuer.is_top()
		{
			return if self.index_of_the_next_position_for_enqueue.get() <= i
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
		
		let mut ei = enqueuer.id.acquire();
		let ev = enqueuer.value.acquire();
		
		if ei > i
		{
			if cell.value.get().is_top() && self.index_of_the_next_position_for_enqueue.get() <= i
			{
				return <*mut Value>::Bottom
			}
		}
		else
		{
			if (ei > 0 && enqueuer.id.relaxed_relaxed_compare_and_swap(&mut ei, -i)) || (ei == -i && cell.value.get().is_top())
			{
				let mut index_of_the_next_position_for_enqueue = self.index_of_the_next_position_for_enqueue.get();
				while index_of_the_next_position_for_enqueue <= i && !self.index_of_the_next_position_for_enqueue.relaxed_relaxed_compare_and_swap(&mut index_of_the_next_position_for_enqueue, i + 1)
				{
				}
				cell.value.set(ev);
			}
		}
		
		cell.value.get()
	}
	
	#[inline(always)]
	fn dequeue_fast_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, id: &mut isize) -> *mut Value
	{
		let index_after_the_next_position_for_dequeue = self.index_of_the_next_position_for_dequeue.sequentially_consistent_fetch_and_increment();
		let cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue, index_after_the_next_position_for_dequeue, per_hyper_thread_handle);
		let dequeued_value = self.enqueue_help(per_hyper_thread_handle, cell, index_after_the_next_position_for_dequeue);
		
		if dequeued_value.is_bottom()
		{
			return <*mut Value>::Bottom
		}
		
		let mut cd = <*mut Dequeuer>::Bottom;
		if dequeued_value.is_not_top() && cell.dequeuer.relaxed_relaxed_compare_and_swap(&mut cd, <*mut Dequeuer>::Top)
		{
			return dequeued_value
		}
		
		*id = 1;
		<*mut Value>::Top
	}
	
	#[inline(always)]
	fn dequeue_slow_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, id: isize) -> *mut Value
	{
		let dequeuer = per_hyper_thread_handle.reference().dequeue_request.deref();
		dequeuer.id.release(id);
		dequeuer.idx.release(id);
		
		self.dequeue_help(per_hyper_thread_handle, per_hyper_thread_handle);
		let i = -dequeuer.idx.get();
		let cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue, i, per_hyper_thread_handle);
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
	fn dequeue_help(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, ph: NonNull<PerHyperThreadHandle<Value>>)
	{
		let dequeuer = ph.reference().dequeue_request.deref();
		let mut idx = dequeuer.idx.acquire();
		let id = dequeuer.id.get();
		
		if idx < id
		{
			return;
		}
		
		// ie, Read the value, then construct a new volatile reference used for `find_cell`.
		// NOTE: This is internally mutable, and calls to `find_cell` will mutate it.
		let Dp = volatile::new(ph.reference().pointer_to_the_node_for_dequeue.get());
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.set(ph.reference().hazard_node_pointer_identifier.get());
		FENCE();
		idx = dequeuer.idx.get();
		
		let mut i = id + 1;
		let mut old_id = id;
		let mut new_id = 0;
		
		loop
		{
			// NOTE: This is internally mutable, and calls to `find_cell` will mutate it.
			let h = volatile::new(Dp.get());
			
			while idx == old_id && new_id == 0
			{
				let cell = Node::find_cell(&h, i, per_hyper_thread_handle);
				
				let mut index_of_the_next_position_for_dequeue = self.index_of_the_next_position_for_dequeue.get();
				while index_of_the_next_position_for_dequeue <= i && !self.index_of_the_next_position_for_dequeue.relaxed_relaxed_compare_and_swap(&mut index_of_the_next_position_for_dequeue, i + 1)
				{
				}
				
				let value = self.enqueue_help(per_hyper_thread_handle, cell, i);
				if value.is_bottom() || (value.is_not_top() && cell.dequeuer.get().is_bottom())
				{
					new_id = i;
				}
				else
				{
					idx = dequeuer.idx.acquire();
				}
				
				
				i.pre_increment();
			}
			
			if new_id != 0
			{
				if dequeuer.idx.release_acquire_compare_and_swap(&mut idx, new_id)
				{
					idx = new_id;
				}
				if idx >= new_id
				{
					new_id = 0;
				}
			}
			
			if idx < 0 || dequeuer.id.get() != id
			{
				break;
			}
			
			let cell = Node::find_cell(&Dp, idx, per_hyper_thread_handle);
			let mut cd = <*mut Dequeuer>::Bottom;
			if cell.value.get().is_top() || cell.dequeuer.relaxed_relaxed_compare_and_swap(&mut cd, dequeuer.as_ptr()) || cd == dequeuer.as_ptr()
			{
				let negative_idx = -idx;
				dequeuer.idx.relaxed_relaxed_compare_and_swap(&mut idx, negative_idx);
				break
			}
			
			old_id = idx;
			if idx >= i
			{
				i = idx + 1;
			}
		}
	}
	
	#[inline(always)]
	fn collect_node_garbage_after_dequeue(&self, our_per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>)
	{
		let mut old_head_of_queue_node_identifier = self.head_of_queue_node_identifier.acquire();
		
		if old_head_of_queue_node_identifier.there_is_no_garbage_to_collect()
		{
			return;
		}
		
		let new = our_per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue.get();
		
		if old_head_of_queue_node_identifier.there_is_not_yet_enough_garbage_to_collect(new, self.maximum_garbage)
		{
			return;
		}
		
		// Try to 'grab a lock' on the garbage nodes to collect.
		if !self.head_of_queue_node_identifier.acquire_relaxed_compare_and_swap(&mut old_head_of_queue_node_identifier, NodeIdentifier::NoHeadOfQueue)
		{
			// Did not grab lock because someone else did - and they'll do the clean up.
			return;
		}
		// 'Lock' is released when `self.head_of_queue_node_identifier.release()` is called below.
		// Once the 'lock' is released all garbage Nodes are free'd.
		
		let old = self.pointer_to_the_head_node.get();
		
		let new = AllPerHyperThreadHandles::check_and_update_all_hyper_thread_handle_hazard_pointers(our_per_hyper_thread_handle, new, old, old_head_of_queue_node_identifier);
		
		let new_head_of_queue_node_identifier = new.identifier();
		
		if new_head_of_queue_node_identifier <= old_head_of_queue_node_identifier
		{
			self.head_of_queue_node_identifier.release(old_head_of_queue_node_identifier);
		}
		else
		{
			self.pointer_to_the_head_node.set(new.as_ptr());
			
			self.head_of_queue_node_identifier.release(new_head_of_queue_node_identifier);
			
			Node::free_garbage_nodes(old, new)
		}
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
	fn check<Value>(self, mut current: NonNull<Node<Value>>, old: *mut Node<Value>) -> NonNull<Node<Value>>
	{
		if self < current.identifier().to_node_identifier()
		{
			let mut node = old.to_non_null();
			while node.identifier().to_node_identifier() < self
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
	// Compared to a value which is originally obtained, via transformations, from dequeuer.id.
	// Initially 0.
	Ei: CopyCell<isize>,
	
	per_hyper_thread_handle_of_next_dequeuer_to_help: CopyCell<NonNull<PerHyperThreadHandle<Value>>>,
	
	// Pointer to a spare node to use, to speedup adding a new node.
	spare: CacheAligned<CopyCell<*mut Node<Value>>>,
}

impl<Value> PerHyperThreadHandle<Value>
{
	const InitialEi: isize = 0;
	
	// A combination of `thread_init` and `queue_init`.
	pub(crate) fn new(queue: NonNull<WaitFreeQueueInner<Value>>) -> NonNull<Self>
	{
		let wait_free_queue_inner = queue.reference();
		
		let per_hyper_thread_handle_non_null = page_size_align_malloc();
		
		{
			let this: &PerHyperThreadHandle<Value> = per_hyper_thread_handle_non_null.reference();
			
			// Seems to be unnecessary as this value is always overwritten by `self.add_to_singularly_linked_list_of_per_hyper_thread_handles()`
			this.initialize_next();
			
			this.hazard_node_pointer_identifier.set(NodePointerIdentifier::Null);
			
			this.pointer_to_the_node_for_enqueue.set(wait_free_queue_inner.pointer_to_the_head_node.get().to_non_null());
			this.enqueuer_node_pointer_identifier.set(this.pointer_to_the_node_for_enqueue.get().identifier().to_node_identifier());
			
			this.pointer_to_the_node_for_dequeue.set(wait_free_queue_inner.pointer_to_the_head_node.get().to_non_null());
			this.dequeuer_node_pointer_identifier.set(this.pointer_to_the_node_for_dequeue.get().identifier().to_node_identifier());
			
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
	fn ei_is_not_initial_and_is_not_id(&self, id: isize) -> bool
	{
		let ei = self.Ei.get();
		ei != Self::InitialEi && ei != id
	}
	
	#[inline(always)]
	fn set_ei(&self, ei: isize)
	{
		self.Ei.set(ei);
	}
	
	#[inline(always)]
	fn reset_ei(&self)
	{
		self.Ei.set(Self::InitialEi)
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
}
