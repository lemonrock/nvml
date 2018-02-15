// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]


use IsNotNull;
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
use ::std::ops::DerefMut;
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
		// #define CASra(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE)
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

trait SafeNonNull<T>
{
	#[inline(always)]
	fn reference(&self) -> &T;
	
	#[inline(always)]
	fn mutable_reference(&mut self) -> &mut T;
}

impl<T> SafeNonNull<T> for NonNull<T>
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

impl<T> DerefMut for CacheAligned<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
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
}

impl<T: Copy> CacheAligned<T>
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

impl<T: Copy> CacheAligned<volatile<T>>
{
	#[inline(always)]
	pub(crate) fn get(&self) -> T
	{
		self.0.get()
	}
	
	#[inline(always)]
	pub(crate) fn set(&self, valueue: T)
	{
		self.0.set(valueue)
	}
	
	#[inline(always)]
	pub(crate) fn ACQUIRE(&self) -> T
	{
		self.0.ACQUIRE()
	}
	
	#[inline(always)]
	pub(crate) fn RELEASE(&self, value: T)
	{
		self.0.RELEASE(value)
	}
	
	#[inline(always)]
	pub(crate) fn FAA(&self, increment: T) -> T
	{
		self.0.FAA(increment)
	}
	
	#[inline(always)]
	pub(crate) fn FAAcs(&self, increment: T) -> T
	{
		self.0.FAAcs(increment)
	}
	
	#[inline(always)]
	pub(crate) fn CAS(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CAS(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn CAScs(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CAScs(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn CASra(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CASra(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn CASa(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CASa(compare, value)
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
	pub(crate) fn set(&self, valueue: T)
	{
		self.0.set(valueue)
	}
	
	#[inline(always)]
	pub(crate) fn ACQUIRE(&self) -> T
	{
		self.0.ACQUIRE()
	}
	
	#[inline(always)]
	pub(crate) fn RELEASE(&self, value: T)
	{
		self.0.RELEASE(value)
	}
	
	#[inline(always)]
	pub(crate) fn FAA(&self, increment: T) -> T
	{
		self.0.FAA(increment)
	}
	
	#[inline(always)]
	pub(crate) fn FAAcs(&self, increment: T) -> T
	{
		self.0.FAAcs(increment)
	}
	
	#[inline(always)]
	pub(crate) fn CAS(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CAS(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn CAScs(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CAScs(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn CASra(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CASra(compare, value)
	}
	
	#[inline(always)]
	pub(crate) fn CASa(&self, compare: &mut T, value: T) -> bool
	{
		self.0.CASa(compare, value)
	}
}

#[derive(Debug)]
pub(crate) struct volatile<T: Copy>(UnsafeCell<T>);

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
	
	#[inline(always)]
	pub(crate) fn ACQUIRE(&self) -> T
	{
		unsafe { atomic_load_acq(self.0.get() as *const T) }
	}
	
	#[inline(always)]
	pub(crate) fn RELEASE(&self, value: T)
	{
		unsafe { atomic_store_rel(self.0.get(), value) }
	}
	
	/// An atomic fetch-and-add.
	/// Returns previous value.
	#[inline(always)]
	pub(crate) fn FAA(&self, increment: T) -> T
	{
		unsafe { atomic_xadd_relaxed(self.0.get(), increment) }
	}
	
	/// An atomic fetch-and-add that also ensures sequential consistency.
	/// Returns previous value.
	#[inline(always)]
	pub(crate) fn FAAcs(&self, increment: T) -> T
	{
		unsafe { atomic_xadd(self.0.get(), increment) }
	}
	
	/// An atomic compare-and-swap that is completely relaxed.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	#[inline(always)]
	pub(crate) fn CAS(&self, compare: &mut T, value: T) -> bool
	{
		let (valueue, ok) = unsafe { atomic_cxchg_relaxed(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = valueue;
		}
		ok
	}
	
	/// An atomic compare-and-swap that also ensures sequential consistency.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	#[inline(always)]
	pub(crate) fn CAScs(&self, compare: &mut T, value: T) -> bool
	{
		let (valueue, ok) = unsafe { atomic_cxchg(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = valueue;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures release semantic when succeed or acquire semantic when failed.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	#[inline(always)]
	pub(crate) fn CASra(&self, compare: &mut T, value: T) -> bool
	{
		let (valueue, ok) = unsafe { atomic_cxchg_acqrel(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = valueue;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures acquire semantic when succeed or relaxed semantic when failed.
	/// true if successful.
	/// false if failed.
	/// compare is updated if failed.
	#[inline(always)]
	pub(crate) fn CASa(&self, compare: &mut T, value: T) -> bool
	{
		let (valueue, ok) = unsafe { atomic_cxchg_acq_failrelaxed(self.0.get(), *compare, value) };
		
		if !ok
		{
			*compare = valueue;
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

impl<Value> Default for Enqueuer<Value>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			id: volatile::new(0),
			value: volatile::new(<*mut Value>::Bottom),
		}
	}
}

impl<Value> Enqueuer<Value>
{
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

impl Default for Dequeuer
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			id: volatile::new(0),
			idx: volatile::new(-1),
		}
	}
}

impl Dequeuer
{
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self
	{
		self as *const _ as *mut _
	}
}

// `pad` is to make this structure 64 bytes, ie one cache line.
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
struct AllWaitFreeQueuePerThreadHandles<Value>([NonNull<PerHyperThreadHandle<Value>>; NumberOfHyperThreads::InclusiveMaximumNumberOfHyperThreads]);

impl<Value> AllWaitFreeQueuePerThreadHandles<Value>
{
	#[inline(always)]
	fn new() -> Self
	{
		unsafe { uninitialized() }
	}
	
	#[inline(always)]
	fn set(&mut self, index: isize, another_wait_free_queue_per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>)
	{
		*(unsafe { self.0.get_unchecked_mut(index as usize) }) = another_wait_free_queue_per_hyper_thread_handle;
	}
	
	#[inline(always)]
	fn get(&mut self, index: isize) -> NonNull<PerHyperThreadHandle<Value>>
	{
		*unsafe { self.0.get_unchecked(index as usize) }
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
	identifier: CacheAligned<NodeIdentifier>,
	cells: Cells<Value>,
}

impl<Value> Node<Value>
{
	// Result is never null
	#[inline(always)]
	fn new_node() -> *mut Self
	{
		let n = page_size_align_malloc();
		unsafe { n.as_ptr().write_bytes(0, 1) }
		n.as_ptr()
	}
	
	fn find_cell<'node>(pointer_to_node: &'node volatile<NonNull<Node<Value>>>, i: isize, mut per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>) -> &'node Cell<Value>
	{
		let mut current = pointer_to_node.get();
		
		let mut current_node_identifier = current.reference().identifier.get();
		let current_maximum_node_identifier = NodeIdentifier(i / Cells::<Value>::SignedNumberOfCellsInANode);
		while current_node_identifier < current_maximum_node_identifier
		{
			let mut next = current.reference().next.get();
			
			if next.is_null()
			{
				let mut spare_node_to_use_for_next = Self::get_non_null_spare_node(per_hyper_thread_handle);
				
				spare_node_to_use_for_next.mutable_reference().identifier.set(current_node_identifier.increment());
				
				if current.reference().next.CASra(&mut next, spare_node_to_use_for_next.as_ptr())
				{
					next = spare_node_to_use_for_next.as_ptr();
					per_hyper_thread_handle.mutable_reference().spare.set(null_mut());
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
	fn get_non_null_spare_node(mut per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>) -> NonNull<Node<Value>>
	{
		let spare = per_hyper_thread_handle.reference().spare.get();
		let spare = if spare.is_not_null()
		{
			spare
		}
		else
		{
			let new_spare = Self::new_node();
			per_hyper_thread_handle.mutable_reference().spare.set(new_spare);
			new_spare
		};
		spare.to_non_null()
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

trait NodeNonNull<T>: SafeNonNull<T>
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
	// Index of the next position for enqueue.
	Ei: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the next position for dequeue.
	Di: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the head of the queue.
	// Used only for garbage collection of Nodes.
	// Called in wfqueue.c code `Hi`.
	head_of_queue_node_identifier: DoubleCacheAligned<volatile<NodeIdentifier>>,
	
	// Pointer to the head node of the queue.
	Hp: volatile<*mut Node<Value>>,
	
	maximum_garbage: MaximumGarbage,
	
	// A singularly-linked list of per-thread handles, atomically updated.
	tail: volatile<*mut PerHyperThreadHandle<Value>>,
}

impl<Value> WaitFreeQueueInner<Value>
{
	const MaximumPatienceForFastPath: isize = 10;
	
	pub(crate) fn new(maximum_garbage: MaximumGarbage) -> NonNull<Self>
	{
		let mut this = page_size_align_malloc();
		
		unsafe
		{
			let this: &mut Self = this.mutable_reference();
			
			this.head_of_queue_node_identifier.set(NodeIdentifier::Initial);
			this.Hp.set(Node::new_node());
			this.Ei.set(1);
			this.Di.set(1);
			write(&mut this.maximum_garbage, maximum_garbage);
			this.tail.set(null_mut());
		}
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn enqueue(&self, mut per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, value_to_enqueue: NonNull<Value>)
	{
		assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.set(per_hyper_thread_handle.reference().enqueuer_node_pointer_identifier);
		
		let mut id = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		while !self.enqueue_fast_path(per_hyper_thread_handle, value_to_enqueue, &mut id) && remaining_patience_for_fast_path.post_decrement() > 0
		{
		}
		if remaining_patience_for_fast_path < 0
		{
			self.enqueue_slow_path(per_hyper_thread_handle, value_to_enqueue, id)
		}
		
		per_hyper_thread_handle.mutable_reference().enqueuer_node_pointer_identifier = per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue.get().reference().identifier.get().to_node_identifier();
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.RELEASE(NodePointerIdentifier::Null)
	}
	
	#[inline(always)]
	pub(crate) fn dequeue(&self, mut per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>) -> *mut Value
	{
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.set(per_hyper_thread_handle.reference().dequeuer_node_pointer_identifier);
		
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
			self.dequeue_help(per_hyper_thread_handle, per_hyper_thread_handle.reference().Dh);
			per_hyper_thread_handle.mutable_reference().Dh = per_hyper_thread_handle.reference().Dh.reference().next.get();
		}
		
		per_hyper_thread_handle.mutable_reference().dequeuer_node_pointer_identifier = per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue.get().reference().identifier.get().to_node_identifier();
		per_hyper_thread_handle.reference().hazard_node_pointer_identifier.RELEASE(NodePointerIdentifier::Null);
		
		if per_hyper_thread_handle.reference().spare.get().is_null()
		{
			self.collect_node_garbage_after_dequeue(per_hyper_thread_handle);
			per_hyper_thread_handle.mutable_reference().spare.set(Node::new_node());
		}
		
		dequeued_value
	}
	
	#[inline(always)]
	fn enqueue_fast_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, value_to_enqueue: NonNull<Value>, id: &mut isize) -> bool
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		let i = self.Ei.FAAcs(1);
		
		let cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue, i, per_hyper_thread_handle);
		
		// Works because the initial state of a Cell is zeroed (Node::new_node() does write_bytes).
		let mut compare_to_value = <*mut Value>::Bottom;
		if cell.value.CAS(&mut compare_to_value, value_to_enqueue.as_ptr())
		{
			true
		}
		else
		{
			*id = i;
			false
		}
	}
	
	#[inline(always)]
	fn enqueue_slow_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, value_to_enqueue: NonNull<Value>, mut id: isize)
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		let value_to_enqueue = value_to_enqueue.as_ptr();
		
		let enqueuer = &per_hyper_thread_handle.reference().Er;
		enqueuer.value.set(value_to_enqueue);
		enqueuer.id.RELEASE(id);

		let tail = &per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue;
		let mut i;
		let mut cell;
		
		'do_while: while
		{
			i = self.Ei.FAA(1);
			cell = Node::find_cell(tail, i, per_hyper_thread_handle);
			let mut ce = <*mut Enqueuer<Value>>::Bottom;
			
			if cell.enqueuer.CAScs(&mut ce, enqueuer.as_ptr()) && cell.value.get().is_not_top()
			{
				enqueuer.id.CAS(&mut id, -i);
				break 'do_while;
			}
			enqueuer.id.get() > 0
		}
		{
		}
		
		id = -enqueuer.id.get();
		cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_enqueue, id, per_hyper_thread_handle);
		if id > i
		{
			let mut Ei = self.Ei.get();
			while Ei <= id && !self.Ei.CAS(&mut Ei, id + 1)
			{
			}
		}
		cell.value.set(value_to_enqueue);
	}
	
	// Used only when dequeue() is called.
	#[inline(always)]
	fn enqueue_help(&self, mut per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, cell: &Cell<Value>, i: isize) -> *mut Value
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
		
		if (value.is_not_top() && value.is_not_bottom()) || (value.is_bottom() && !cell.value.CAScs(&mut value, <*mut Value>::Top) && value.is_not_top())
		{
			return value;
		}
		
		let mut enqueuer = cell.enqueuer.get();
		
		if enqueuer.is_bottom()
		{
			let mut ph = per_hyper_thread_handle.reference().Eh.get();
			let (mut pe, mut id) =
			{
				let pe = ph.reference().Er.deref();
				(pe.as_ptr(), pe.id.get())
			};
			
			if per_hyper_thread_handle.reference().Ei != 0 && per_hyper_thread_handle.reference().Ei != id
			{
				per_hyper_thread_handle.mutable_reference().Ei = 0;
				per_hyper_thread_handle.mutable_reference().Eh.set(ph.reference().next.get());
				
				ph = per_hyper_thread_handle.reference().Eh.get();
				let (pe2, id2) =
				{
					let pe = ph.reference().Er.deref();
					(pe.as_ptr(), pe.id.get())
				};
				pe = pe2;
				id = id2;
			}
			
			if id > 0 && id <= i && !cell.enqueuer.CAS(&mut enqueuer, pe)
			{
				per_hyper_thread_handle.mutable_reference().Ei = id
			}
			else
			{
				per_hyper_thread_handle.mutable_reference().Eh.set(ph.reference().next.get())
			}
			
			if enqueuer.is_bottom() && cell.enqueuer.CAS(&mut enqueuer, <*mut Enqueuer<Value>>::Top)
			{
				enqueuer = <*mut Enqueuer<Value>>::Top
			}
		}
		
		if enqueuer.is_top()
		{
			return if self.Ei.get() <= i
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
		
		let mut ei = enqueuer.id.ACQUIRE();
		let ev = enqueuer.value.ACQUIRE();
		
		if ei > i
		{
			if cell.value.get().is_top() && self.Ei.get() <= i
			{
				return <*mut Value>::Bottom
			}
		}
		else
		{
			if (ei > 0 && enqueuer.id.CAS(&mut ei, -i)) || (ei == -i && cell.value.get().is_top())
			{
				let mut Ei = self.Ei.get();
				while Ei <= i && !self.Ei.CAS(&mut Ei, i + 1)
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
		let i = self.Di.FAAcs(1);
		let cell = Node::find_cell(&per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue, i, per_hyper_thread_handle);
		let dequeued_value = self.enqueue_help(per_hyper_thread_handle, cell, i);
		
		if dequeued_value.is_bottom()
		{
			return <*mut Value>::Bottom
		}
		
		let mut cd = <*mut Dequeuer>::Bottom;
		if dequeued_value.is_not_top() && cell.dequeuer.CAS(&mut cd, <*mut Dequeuer>::Top)
		{
			return dequeued_value
		}
		
		*id = 1;
		<*mut Value>::Top
	}
	
	#[inline(always)]
	fn dequeue_slow_path(&self, per_hyper_thread_handle: NonNull<PerHyperThreadHandle<Value>>, id: isize) -> *mut Value
	{
		let dequeuer = &per_hyper_thread_handle.reference().Dr;
		dequeuer.id.RELEASE(id);
		dequeuer.idx.RELEASE(id);
		
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
		let dequeuer = ph.reference().Dr.deref();
		let mut idx = dequeuer.idx.ACQUIRE();
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
				
				let mut Di = self.Di.get();
				while Di <= i && !self.Di.CAS(&mut Di, i + 1)
				{
				}
				
				let value = self.enqueue_help(per_hyper_thread_handle, cell, i);
				if value.is_bottom() || (value.is_not_top() && cell.dequeuer.get().is_bottom())
				{
					new_id = i;
				}
				else
				{
					idx = dequeuer.idx.ACQUIRE();
				}
				
				
				i.pre_increment();
			}
			
			if new_id != 0
			{
				if dequeuer.idx.CASra(&mut idx, new_id)
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
			if cell.value.get().is_top() || cell.dequeuer.CAS(&mut cd, dequeuer.as_ptr()) || cd == dequeuer.as_ptr()
			{
				let negative_idx = -idx;
				dequeuer.idx.CAS(&mut idx, negative_idx);
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
		#[inline(always)]
		fn check<Value>(hazard_node_pointer_identifier: &volatile<NodePointerIdentifier>, mut current: NonNull<Node<Value>>, old: *mut Node<Value>) -> NonNull<Node<Value>>
		{
			let hazard_node_pointer_identifier = hazard_node_pointer_identifier.ACQUIRE();
			
			if hazard_node_pointer_identifier < current.identifier().to_node_identifier()
			{
				let mut node = old.to_non_null();
				while node.identifier().to_node_identifier() < hazard_node_pointer_identifier
				{
					node = node.reference().next.get().to_non_null();
				}
				current = node;
			}
			
			current
		}
		
		#[inline(always)]
		fn update<Value>(pPn: &volatile<NonNull<Node<Value>>>, mut current: NonNull<Node<Value>>, hazard_node_pointer_identifier: &volatile<NodePointerIdentifier>, old: *mut Node<Value>) -> NonNull<Node<Value>>
		{
			let mut node = pPn.ACQUIRE();
			
			if node.identifier() < current.identifier()
			{
				if !pPn.CAScs(&mut node, current)
				{
					if node.identifier() < current.identifier()
					{
						current = node;
					}
				}
				
				current = check(hazard_node_pointer_identifier, current, old);
			}
			
			current
		}
		
		
		let mut old_head_of_queue_node_identifier = self.head_of_queue_node_identifier.ACQUIRE();
		
		if old_head_of_queue_node_identifier.there_is_no_garbage_to_collect()
		{
			return;
		}
		
		let mut new = our_per_hyper_thread_handle.reference().pointer_to_the_node_for_dequeue.get();
		
		if old_head_of_queue_node_identifier.there_is_not_yet_enough_garbage_to_collect(new, self.maximum_garbage)
		{
			return;
		}
		
		// Try to 'grab a lock' on the garbage nodes to collect.
		if !self.head_of_queue_node_identifier.CASa(&mut old_head_of_queue_node_identifier, NodeIdentifier::NoHeadOfQueue)
		{
			// Did not grab lock because someone else did - and they'll do the clean up.
			return;
		}
		
		// Lock if released when `self.head_of_queue_node_identifier.RELEASE()` is called.
		
		let old = self.Hp.get();
		let mut our_or_another_threads_per_hyper_thread_handle = our_per_hyper_thread_handle;
		
		let mut phs = AllWaitFreeQueuePerThreadHandles::new();
		
		let mut index = 0;
		do_while!
		{
			do
			{
				{
					let reference = our_or_another_threads_per_hyper_thread_handle.reference();
					let hazard_node_pointer_identifier = &reference.hazard_node_pointer_identifier;
				
					new = check(hazard_node_pointer_identifier, new, old);
					new = update(&reference.pointer_to_the_node_for_enqueue, new, hazard_node_pointer_identifier, old);
					new = update(&reference.pointer_to_the_node_for_dequeue, new, hazard_node_pointer_identifier, old);
					
					phs.set(index.post_increment(), our_or_another_threads_per_hyper_thread_handle);
				}
				our_or_another_threads_per_hyper_thread_handle = our_or_another_threads_per_hyper_thread_handle.reference().next.get();
			}
			while new.identifier() > old_head_of_queue_node_identifier && our_or_another_threads_per_hyper_thread_handle.as_ptr() != our_per_hyper_thread_handle.as_ptr()
		}
		
		while new.identifier() > old_head_of_queue_node_identifier && index.pre_decrement() >= 0
		{
			new = check(&phs.get(index).reference().hazard_node_pointer_identifier, new, old);
		}
		
		let new_head_of_queue_node_identifier = new.reference().identifier();
		
		if new_head_of_queue_node_identifier <= old_head_of_queue_node_identifier
		{
			self.head_of_queue_node_identifier.RELEASE(old_head_of_queue_node_identifier);
		}
		else
		{
			self.Hp.set(new.as_ptr());
			self.head_of_queue_node_identifier.RELEASE(new_head_of_queue_node_identifier);
			
			Node::free_garbage_nodes(old, new)
		}
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
}

impl NodePointerIdentifier
{
	#[inline(always)]
	fn from_node_identifier(node_identifier: NodeIdentifier) -> Self
	{
		NodePointerIdentifier(node_identifier.0 as usize)
	}
}

struct PerHyperThreadHandle<Value>
{
	// Pointer to the next thread handle; a singularly linked list.
	// Can pointer to self if the first item in the singularly linked list.
	next: ExtendedNonNullAtomicPointer<PerHyperThreadHandle<Value>>,
	
	// Hazard pointer.
	hazard_node_pointer_identifier: volatile<NodePointerIdentifier>,
	
	pointer_to_the_node_for_enqueue: volatile<NonNull<Node<Value>>>,
	// Obtained originally from self.pointer_to_the_node_for_enqueue.id, assigned to self.hazard_node_pointer_identifier; a kind of cache of the original value of id.
	enqueuer_node_pointer_identifier: NodePointerIdentifier,
	
	pointer_to_the_node_for_dequeue: volatile<NonNull<Node<Value>>>,
	// Obtained originally from self.pointer_to_the_node_for_dequeue.id, assigned to self.hazard_node_pointer_identifier; a kind of cache of the original value of id.
	dequeuer_node_pointer_identifier: NodePointerIdentifier,
	
	// Enqueue request.
	Er: CacheAligned<Enqueuer<Value>>,
	
	// Dequeue request.
	Dr: CacheAligned<Dequeuer>,
	
	// Handle of the next enqueuer to help.
	Eh: CacheAligned<NonNull<PerHyperThreadHandle<Value>>>,
	
	Ei: isize,
	
	// Handle of the next dequeuer to help.
	Dh: NonNull<PerHyperThreadHandle<Value>>,
	
	// Pointer to a spare node to use, to speedup adding a new node.
	spare: CacheAligned<*mut Node<Value>>,
}

impl<Value> PerHyperThreadHandle<Value>
{
	// A combination of `thread_init` and `queue_init`.
	pub(crate) fn new(queue: NonNull<WaitFreeQueueInner<Value>>) -> NonNull<Self>
	{
		let wait_free_queue_inner = queue.reference();
		
		let mut per_hyper_thread_handle_non_null = page_size_align_malloc();
		unsafe
		{
			let this_copy_borrow_checker_hack = per_hyper_thread_handle_non_null;
			let this: &mut PerHyperThreadHandle<Value> = per_hyper_thread_handle_non_null.mutable_reference();
			
			// Seems to be unnecessary as this value is always overwritten.
			// this.next.set(NonNull::dangling());
			
			this.hazard_node_pointer_identifier.set(NodePointerIdentifier::Null);
			
			this.pointer_to_the_node_for_enqueue.set(wait_free_queue_inner.Hp.get().to_non_null());
			write(&mut this.enqueuer_node_pointer_identifier, this.pointer_to_the_node_for_enqueue.get().identifier().to_node_identifier());
			
			this.pointer_to_the_node_for_dequeue.set(wait_free_queue_inner.Hp.get().to_non_null());
			write(&mut this.dequeuer_node_pointer_identifier, this.pointer_to_the_node_for_dequeue.get().identifier().to_node_identifier());
			
			write_volatile(&mut this.Er, CacheAligned::default());
			
			write_volatile(&mut this.Dr, CacheAligned::default());
			
			write(&mut this.Ei, 0);
			
			write_volatile(&mut this.spare, CacheAligned::new(Node::new_node()));
			
			let mut tail = wait_free_queue_inner.tail.get();
			
			if tail.is_null()
			{
				this.next.set(this_copy_borrow_checker_hack);
				if wait_free_queue_inner.tail.CASra(&mut tail, this)
				{
					this.Eh.set(this.next.get());
					write(&mut this.Dh, this.next.get());
					return this_copy_borrow_checker_hack
				}
				// NOTE: tail will have been updated by CASra; queue.tail will not longer have been null, hence tail will now no longer be null, so fall through to logic below.
			}
			let tail_non_null = tail.to_non_null();
			
			let tail = tail_non_null.reference();
			let mut next = tail.next.get();
			
			do_while!
			{
				do
				{
					this.next.set(next)
				}
				while !tail.next.CASra(&mut next, this_copy_borrow_checker_hack)
			}
			
			this.Eh.set(this.next.get());
			write(&mut this.Dh, this.next.get());
		}
		
		per_hyper_thread_handle_non_null
	}
}
