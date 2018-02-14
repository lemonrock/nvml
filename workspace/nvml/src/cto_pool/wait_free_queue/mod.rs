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
		
		NonNull::new_safe(self)
	}
}

trait SafeNonNull<T>
{
	#[inline(always)]
	fn reference(&self) -> &T;
	
	#[inline(always)]
	fn mutable_reference(&mut self) -> &mut T;
	
	#[inline(always)]
	fn new_safe(pointer: *mut T) -> NonNull<T>;
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
	
	#[inline(always)]
	fn new_safe(pointer: *mut T) -> NonNull<T>
	{
		debug_assert!(pointer.is_not_null(), "pointer was null");
		
		unsafe { NonNull::new_unchecked(pointer) }
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
const NumberOfCells: usize = (1 << 10) - 2;

#[repr(C, align(64))]
struct Cells<Value>([Cell<Value>; NumberOfCells]);

impl<Value> Cells<Value>
{
	const NumberOfCells: usize = NumberOfCells;
	
	const SignedNumberOfCells: isize = Self::NumberOfCells as isize;
	
	#[inline(always)]
	pub(crate) fn get_cell(&self, index: isize) -> &Cell<Value>
	{
		debug_assert!(index >= 0, "index is negative");
		debug_assert!(index < Self::SignedNumberOfCells, "index is N or greater");
		unsafe { self.0.get_unchecked(index as usize) }
	}
}

#[repr(C)]
struct Node<Value>
{
	next: CacheAligned<volatile<*mut Node<Value>>>,
	id: CacheAligned<isize>,
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
	
	fn find_cell<'node>(ptr: &'node volatile<NonNull<Node<Value>>>, i: isize, mut th: NonNull<WaitFreeQueuePerThreadHandle<Value>>) -> &'node Cell<Value>
	{
		let mut curr = ptr.get();
		
		let mut j = curr.reference().id.get();
		let x = i / Cells::<Value>::SignedNumberOfCells;
		while j < x
		{
			let mut next = curr.reference().next.get();
			
			if next.is_null()
			{
				let mut temp = th.reference().spare.get();
				if temp.is_null()
				{
					let new_node = Self::new_node();
					temp = new_node;
					th.mutable_reference().spare.set(temp);
				}
				
				temp.to_non_null().mutable_reference().id.set(j + 1);
				
				if curr.reference().next.CASra(&mut next, temp)
				{
					next = temp;
					th.mutable_reference().spare.set(null_mut());
				}
			}
			
			curr = next.to_non_null();
			
			j.pre_increment();
		}
		
		ptr.set(curr);
		
		unsafe { &* curr.as_ptr() }.cells.get_cell(i % Cells::<Value>::SignedNumberOfCells)
	}
	
	#[inline(always)]
	fn id(&self) -> isize
	{
		self.id.get()
	}
}

trait NodeNonNull<T>: SafeNonNull<T>
{
	#[inline(always)]
	fn id(self) -> isize;
}

impl<Value> NodeNonNull<Node<Value>> for NonNull<Node<Value>>
{
	#[inline(always)]
	fn id(self) -> isize
	{
		self.reference().id()
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
	Hi: DoubleCacheAligned<volatile<isize>>,
	
	// Pointer to the head node of the queue.
	Hp: volatile<*mut Node<Value>>,
	
	// Number of processors.
	number_of_hyper_threads: NumberOfHyperThreads,
	
	// A singularly-linked list of per-thread handles, atomically updated.
	tail: volatile<*mut WaitFreeQueuePerThreadHandle<Value>>,
}

impl<Value> WaitFreeQueueInner<Value>
{
	const MaximumPatienceForFastPath: isize = 10;
	
	pub(crate) fn new(number_of_hyper_threads: NumberOfHyperThreads) -> NonNull<Self>
	{
		let mut this = page_size_align_malloc();
		
		unsafe
		{
			let this: &mut Self = this.mutable_reference();
			
			this.Hi.set(0);
			this.Hp.set(Node::new_node());
			this.Ei.set(1);
			this.Di.set(1);
			write(&mut this.number_of_hyper_threads, number_of_hyper_threads);
			this.tail.set(null_mut());
		}
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn enqueue(&self, mut per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, value_to_enqueue: NonNull<Value>)
	{
		assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		per_thread_handle.reference().hzd_node_id.set(per_thread_handle.reference().enq_node_id);
		
		let mut id = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		while !self.enqueue_fast_path(per_thread_handle, value_to_enqueue, &mut id) && remaining_patience_for_fast_path.post_decrement() > 0
		{
		}
		if remaining_patience_for_fast_path < 0
		{
			self.enqueue_slow_path(per_thread_handle, value_to_enqueue, id)
		}
		
		per_thread_handle.mutable_reference().enq_node_id = per_thread_handle.reference().Ep.get().reference().id.get() as usize;
		per_thread_handle.reference().hzd_node_id.RELEASE(!0)
	}
	
	#[inline(always)]
	pub(crate) fn dequeue(&self, mut per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>) -> *mut Value
	{
		per_thread_handle.reference().hzd_node_id.set(per_thread_handle.reference().deq_node_id);
		
		let mut dequeued_value;
		let mut id = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		
		do_while!
		{
			do
			{
				dequeued_value = self.dequeue_fast_path(per_thread_handle, &mut id);
			}
			while dequeued_value.is_top() && remaining_patience_for_fast_path.post_decrement() > 0
		}
		
		if dequeued_value.is_top()
		{
			dequeued_value = self.dequeue_slow_path(per_thread_handle, id);
		}
		
		// `EMPTY`: a value that will be returned if a `dequeue` fails.
		let EMPTY: *mut Value = 0 as *mut Value;
		if dequeued_value != EMPTY
		{
			self.dequeue_help(per_thread_handle, per_thread_handle.reference().Dh);
			per_thread_handle.mutable_reference().Dh = per_thread_handle.reference().Dh.reference().next.get();
		}
		
		per_thread_handle.mutable_reference().deq_node_id = per_thread_handle.reference().Dp.get().reference().id.get() as usize;
		per_thread_handle.reference().hzd_node_id.RELEASE(!0);
		
		if per_thread_handle.reference().spare.get().is_null()
		{
			self.clean_up_garbage_after_dequeue(per_thread_handle);
			per_thread_handle.mutable_reference().spare.set(Node::new_node());
		}
		
		dequeued_value
	}
	
	#[inline(always)]
	fn enqueue_fast_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, value_to_enqueue: NonNull<Value>, id: &mut isize) -> bool
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		
		let i = self.Ei.FAAcs(1);
		
		let cell = Node::find_cell(&per_thread_handle.reference().Ep, i, per_thread_handle);
		
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
	fn enqueue_slow_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, value_to_enqueue: NonNull<Value>, mut id: isize)
	{
		debug_assert!(value_to_enqueue.as_ptr().is_not_top(), "value_to_enqueue is not allowed to be top");
		let value_to_enqueue = value_to_enqueue.as_ptr();
		
		let enqueuer = &per_thread_handle.reference().Er;
		enqueuer.value.set(value_to_enqueue);
		enqueuer.id.RELEASE(id);

		let tail = &per_thread_handle.reference().Ep;
		let mut i;
		let mut cell;
		
		'do_while: while
		{
			i = self.Ei.FAA(1);
			cell = Node::find_cell(tail, i, per_thread_handle);
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
		cell = Node::find_cell(&per_thread_handle.reference().Ep, id, per_thread_handle);
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
	fn enqueue_help(&self, mut per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, cell: &Cell<Value>, i: isize) -> *mut Value
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
			let mut ph = per_thread_handle.reference().Eh.get();
			let (mut pe, mut id) =
			{
				let pe = ph.reference().Er.deref();
				(pe.as_ptr(), pe.id.get())
			};
			
			if per_thread_handle.reference().Ei != 0 && per_thread_handle.reference().Ei != id
			{
				per_thread_handle.mutable_reference().Ei = 0;
				per_thread_handle.mutable_reference().Eh.set(ph.reference().next.get());
				
				ph = per_thread_handle.reference().Eh.get();
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
				per_thread_handle.mutable_reference().Ei = id
			}
			else
			{
				per_thread_handle.mutable_reference().Eh.set(ph.reference().next.get())
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
	fn dequeue_fast_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, id: &mut isize) -> *mut Value
	{
		let i = self.Di.FAAcs(1);
		let cell = Node::find_cell(&per_thread_handle.reference().Dp, i, per_thread_handle);
		let dequeued_value = self.enqueue_help(per_thread_handle, cell, i);
		
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
	fn dequeue_slow_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, id: isize) -> *mut Value
	{
		let dequeuer = &per_thread_handle.reference().Dr;
		dequeuer.id.RELEASE(id);
		dequeuer.idx.RELEASE(id);
		
		self.dequeue_help(per_thread_handle, per_thread_handle);
		let i = -dequeuer.idx.get();
		let cell = Node::find_cell(&per_thread_handle.reference().Dp, i, per_thread_handle);
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
	fn dequeue_help(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>, ph: NonNull<WaitFreeQueuePerThreadHandle<Value>>)
	{
		let dequeuer = ph.reference().Dr.deref();
		let mut idx = dequeuer.idx.ACQUIRE();
		let id = dequeuer.id.get();
		
		if idx < id
		{
			return;
		}
		
		// ie, Read the value, then construct a new volatile reference used for compatibility in find_cell.
		let Dp = volatile::new(ph.reference().Dp.get());
		per_thread_handle.reference().hzd_node_id.set(ph.reference().hzd_node_id.get());
		FENCE();
		idx = dequeuer.idx.get();
		
		let mut i = id + 1;
		let mut old = id;
		let mut new = 0;
		
		loop
		{
			while idx == old && new == 0
			{
				let cell = Node::find_cell(&Dp, i, per_thread_handle);
				
				let mut Di = self.Di.get();
				while Di <= i && !self.Di.CAS(&mut Di, i + 1)
				{
				}
				
				let value = self.enqueue_help(per_thread_handle, cell, i);
				if value.is_bottom() || (value.is_not_top() && cell.dequeuer.get().is_bottom())
				{
					new = i;
				}
				else
				{
					idx = dequeuer.idx.ACQUIRE();
				}
				
				
				i.pre_increment();
			}
			
			if new != 0
			{
				if dequeuer.idx.CASra(&mut idx, new)
				{
					idx = new;
				}
				if idx >= new
				{
					new = 0;
				}
			}
			
			if idx < 0 || dequeuer.id.get() != id
			{
				break;
			}
			
			let c = Node::find_cell(&Dp, idx, per_thread_handle);
			let mut cd = <*mut Dequeuer>::Bottom;
			if c.value.get().is_top() || c.dequeuer.CAS(&mut cd, dequeuer.as_ptr()) || cd == dequeuer.as_ptr()
			{
				let negative_idx = -idx;
				dequeuer.idx.CAS(&mut idx, negative_idx);
				break
			}
			
			old = idx;
			if idx >= i
			{
				i = idx + 1;
			}
		}
	}
	
	#[inline(always)]
	fn clean_up_garbage_after_dequeue(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle<Value>>)
	{
		#[inline(always)]
		fn check<Value>(p_hzd_node_id: &volatile<usize>, mut cur: NonNull<Node<Value>>, old: *mut Node<Value>) -> NonNull<Node<Value>>
		{
			let hzd_node_id: usize = p_hzd_node_id.ACQUIRE();
			
			if hzd_node_id < (cur.id() as usize)
			{
				let mut tmp = old.to_non_null();
				while (tmp.id() as usize) < hzd_node_id
				{
					tmp = tmp.reference().next.get().to_non_null();
				}
				cur = tmp;
			}
			
			cur
		}
		
		#[inline(always)]
		fn update<Value>(pPn: &volatile<NonNull<Node<Value>>>, mut cur: NonNull<Node<Value>>, p_hzd_node_id: &volatile<usize>, old: *mut Node<Value>) -> NonNull<Node<Value>>
		{
			let mut ptr = pPn.ACQUIRE();
			
			if ptr.id() < cur.id()
			{
				if !pPn.CAScs(&mut ptr, cur)
				{
					if ptr.id() < cur.id()
					{
						cur = ptr;
					}
				}
				
				cur = check(p_hzd_node_id, cur, old);
			}
			
			cur
		}
		
		const NoOid: isize = -1;
		
		let mut oid = self.Hi.ACQUIRE();
		
		let mut new = per_thread_handle.reference().Dp.get();
		
		if oid == NoOid
		{
			return;
		}
		
		if new.id() - oid < self.number_of_hyper_threads.maximum_garbage()
		{
			return;
		}
		
		if !self.Hi.CASa(&mut oid, NoOid)
		{
			return;
		}
		
		let mut old = self.Hp.get();
		let mut ph = per_thread_handle;
		
		// Was dimensioned by q.reference().number_of_hyper_threads, but variable stack arrays aren't supported by Rust.
		// handle_t *phs[q->number_of_hyper_threads];  ie let mut phs: [*mut handle_t; q.reference().number_of_hyper_threads]
		// We could use a Vec here but a heap allocation seems overkill, unless we keep it with the thread handle.
		let mut phs: [NonNull<WaitFreeQueuePerThreadHandle<Value>>; NumberOfHyperThreads::InclusiveMaximumNumberOfHyperThreads] = unsafe { uninitialized() };
		let mut i: isize = 0;
		do_while!
		{
			do
			{
				new = check(&ph.reference().hzd_node_id, new, old);
				new = update(&ph.reference().Ep, new, &ph.reference().hzd_node_id, old);
				new = update(&ph.reference().Dp, new, &ph.reference().hzd_node_id, old);
				*(unsafe { phs.get_unchecked_mut(i.post_increment() as usize) }) = ph;
				ph = ph.reference().next.get();
			}
			while new.id() > oid && ph.as_ptr() != per_thread_handle.as_ptr()
		}
		
		while new.id() > oid && i.pre_decrement() >= 0
		{
			new = check(&(unsafe { phs.get_unchecked(i as usize) }.reference().hzd_node_id), new, old);
		}
		
		let nid = new.reference().id.get() as isize;
		
		if nid <= oid
		{
			self.Hi.RELEASE(oid);
		}
		else
		{
			self.Hp.set(new.as_ptr());
			self.Hi.RELEASE(nid);
			
			while old != new.as_ptr()
			{
				let old_non_null = old.to_non_null();
				let tmp = old_non_null.reference().next.get();
				free(old_non_null);
				old = tmp;
			}
		}
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NumberOfHyperThreads(usize);

impl NumberOfHyperThreads
{
	pub const InclusiveMaximumNumberOfHyperThreads: usize = 256;
	
	/// Panics if `number_of_hyper_threads` is 0 or exceeds InclusiveMaximumNumberOfHyperThreads.
	#[inline(always)]
	pub fn new(number_of_hyper_threads: u16) -> Self
	{
		let number_of_hyper_threads = number_of_hyper_threads as usize;
		
		assert_ne!(number_of_hyper_threads, 0, "number_of_hyper_threads can not be zero");
		assert!(number_of_hyper_threads <= Self::InclusiveMaximumNumberOfHyperThreads, "number_of_hyper_threads '{}' exceeds Self::InclusiveMaximumNumberOfHyperThreads '{}'", number_of_hyper_threads, Self::InclusiveMaximumNumberOfHyperThreads);
		
		NumberOfHyperThreads(number_of_hyper_threads)
	}
	
	#[inline(always)]
	fn maximum_garbage(&self) -> isize
	{
		let maximum_garbage = 2 * self.0;
		debug_assert!(maximum_garbage <= ::std::isize::MAX as usize, "maximum_garbage exceeds isize::MAX");
		maximum_garbage as isize
	}
}

struct WaitFreeQueuePerThreadHandle<Value>
{
	// Pointer to the next thread handle; a singularly linked list.
	// Can pointer to self if the first item in the singularly linked list.
	next: ExtendedNonNullAtomicPointer<WaitFreeQueuePerThreadHandle<Value>>,
	
	// Hazard pointer.
	hzd_node_id: volatile<usize>,
	
	// Pointer to the node for enqueue.
	Ep: volatile<NonNull<Node<Value>>>,
	// Obtained originally from self.Ep.id, assigned to self.hzd_node_id; a kind of cache of the original value of id.
	enq_node_id: usize,
	
	// Pointer to the node for dequeue.
	Dp: volatile<NonNull<Node<Value>>>,
	// Obtained originally from self.Dp.id, assigned to hzd_node_id; a kind of cache of the original value of id.
	deq_node_id: usize,
	
	// Enqueue request.
	Er: CacheAligned<Enqueuer<Value>>,
	
	// Dequeue request.
	Dr: CacheAligned<Dequeuer>,
	
	// Handle of the next enqueuer to help.
	Eh: CacheAligned<NonNull<WaitFreeQueuePerThreadHandle<Value>>>,
	
	Ei: isize,
	
	// Handle of the next dequeuer to help.
	Dh: NonNull<WaitFreeQueuePerThreadHandle<Value>>,
	
	// Pointer to a spare node to use, to speedup adding a new node.
	spare: CacheAligned<*mut Node<Value>>,
}

impl<Value> WaitFreeQueuePerThreadHandle<Value>
{
	pub(crate) fn thread_init(q: NonNull<WaitFreeQueueInner<Value>>) -> NonNull<Self>
	{
		let th = page_size_align_malloc();
		Self::queue_register(q, th);
		th
	}
	
	/// Called once per thread from `thread_init`.
	#[inline(always)]
	fn queue_register(q: NonNull<WaitFreeQueueInner<Value>>, mut th: NonNull<WaitFreeQueuePerThreadHandle<Value>>)
	{
		let q = q.reference();
		let th = th.mutable_reference();
		
		unsafe
		{
			// Seems to be unnecessary as this value is always overwritten.
			// th.next.set(null_mut());
			th.hzd_node_id.set(!0);
			
			th.Ep.set(q.Hp.get().to_non_null());
			// enq_node_id can become a hazard node id. As such, the value can be -1 which converts to !0.
			write(&mut th.enq_node_id, th.Ep.get().id() as usize);
			
			th.Dp.set(q.Hp.get().to_non_null());
			// deq_node_id can become a hazard node id. As such, the value can be -1 which converts to !0.
			write(&mut th.deq_node_id, th.Dp.get().id() as usize);
			
			write_volatile(&mut th.Er, CacheAligned::default());
			
			write_volatile(&mut th.Dr, CacheAligned::default());
			
			write(&mut th.Ei, 0);
			
			write_volatile(&mut th.spare, CacheAligned::new(Node::new_node()));
			
			let mut tail = q.tail.get();
			
			if tail.is_null()
			{
				let th_self = NonNull::new_safe(th);
				th.next.set(th_self);
				if q.tail.CASra(&mut tail, th)
				{
					th.Eh.set(th.next.get());
					write(&mut th.Dh, th.next.get());
					return
				}
				// NOTE: tail will have been updated by CASra; q._tail will not longer have been null, hence tail will now no longer be null, so fall through to logic below.
			}
			let tail_non_null = tail.to_non_null();
			
			let tail = tail_non_null.reference();
			let mut next = tail.next.get();
			
			do_while!
			{
				do
				{
					th.next.set(next)
				}
				while !tail.next.CASra(&mut next, NonNull::new_safe(th))
			}
			
			th.Eh.set(th.next.get());
			write(&mut th.Dh, th.next.get());
		}
	}
}
