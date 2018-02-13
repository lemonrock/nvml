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
fn align_malloc<T>(alignment: usize, size: usize) -> NonNull<T>
{
	unimplemented!()
}

#[inline(always)]
fn memset<T>(pointer: NonNull<T>, byte: u8, copies: usize)
{
	unimplemented!()
}

#[inline(always)]
fn free<T>(pointer: NonNull<T>)
{
	unimplemented!()
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

impl CacheAligned<[Cell; WFQUEUE_NODE_SIZE]>
{
	#[inline(always)]
	pub(crate) fn get_cell(&self, index: isize) -> &Cell
	{
		debug_assert!(index >= 0, "index is negative");
		debug_assert!(index < N, "index is N or greater");
		unsafe { self.0.get_unchecked(index as usize) }
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
	pub(crate) fn CAS(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CAS(cmp, val)
	}
	
	#[inline(always)]
	pub(crate) fn CAScs(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CAScs(cmp, val)
	}
	
	#[inline(always)]
	pub(crate) fn CASra(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CASra(cmp, val)
	}
	
	#[inline(always)]
	pub(crate) fn CASa(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CASa(cmp, val)
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
pub(crate) struct DoubleCacheAligned<T>(T);

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
	pub(crate) fn CAS(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CAS(cmp, val)
	}
	
	#[inline(always)]
	pub(crate) fn CAScs(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CAScs(cmp, val)
	}
	
	#[inline(always)]
	pub(crate) fn CASra(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CASra(cmp, val)
	}
	
	#[inline(always)]
	pub(crate) fn CASa(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CASa(cmp, val)
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
		// FAA(ptr, val) __atomic_fetch_add(ptr, val, __ATOMIC_RELAXED)
		unsafe { atomic_xadd_relaxed(self.0.get(), increment) }
	}
	
	/// An atomic fetch-and-add that also ensures sequential consistency.
	/// Returns previous value.
	#[inline(always)]
	pub(crate) fn FAAcs(&self, increment: T) -> T
	{
		// __atomic_fetch_add(ptr, val, __ATOMIC_SEQ_CST)
		unsafe { atomic_xadd(self.0.get(), increment) }
	}
	
	/// An atomic compare-and-swap that is completely relaxed.
	/// true if successful
	/// false if failed
	#[inline(always)]
	pub(crate) fn CAS(&self, cmp: &mut T, val: T) -> bool
	{
		// #define CAS(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_RELAXED, __ATOMIC_RELAXED)
		let (value, ok) = unsafe { atomic_cxchg_relaxed(self.0.get(), *cmp, val) };
		
		if !ok
		{
			*cmp = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that also ensures sequential consistency.
	/// true if successful
	/// false if failed
	#[inline(always)]
	pub(crate) fn CAScs(&self, cmp: &mut T, val: T) -> bool
	{
		// #define CAScs(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST)
		let (value, ok) = unsafe { atomic_cxchg(self.0.get(), *cmp, val) };
		
		if !ok
		{
			*cmp = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures release semantic when succeed or acquire semantic when failed.
	/// true if successful
	/// false if failed
	#[inline(always)]
	pub(crate) fn CASra(&self, cmp: &mut T, val: T) -> bool
	{
		// #define CASra(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_RELEASE, __ATOMIC_ACQUIRE)
		let (value, ok) = unsafe { atomic_cxchg_acqrel(self.0.get(), *cmp, val) };
		
		if !ok
		{
			*cmp = value;
		}
		ok
	}
	
	/// An atomic compare-and-swap that ensures acquire semantic when succeed or relaxed semantic when failed.
	/// true if successful
	/// false if failed
	#[inline(always)]
	pub(crate) fn CASa(&self, cmp: &mut T, val: T) -> bool
	{
		// #define CASa(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_ACQUIRE, __ATOMIC_RELAXED)
		let (value, ok) = unsafe { atomic_cxchg_acq_failrelaxed(self.0.get(), *cmp, val) };
		
		if !ok
		{
			*cmp = value;
		}
		ok
	}
}




const PAGE_SIZE: usize = 4096;

const EMPTY: *mut void = 0 as *mut void;

const WFQUEUE_NODE_SIZE: usize = (1 << 10) - 2;

const N: isize = WFQUEUE_NODE_SIZE as isize;

const BOT: *mut void = 0 as *mut void;

const TOP: *mut void = !0 as *mut void;

const MAX_SPIN: usize = 100;

const MaximumNumberOfThreads: usize = 256;

#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
struct Enqueuer
{
	id: volatile<isize>,
	val: volatile<*mut void>,
}

impl Enqueuer
{
	#[inline(always)]
	fn new(id: isize, val: *mut void) -> Self
	{
		Self
		{
			id: volatile::new(id),
			val: volatile::new(val),
		}
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
	fn new(id: isize, idx: isize) -> Self
	{
		Self
		{
			id: volatile::new(id),
			idx: volatile::new(idx),
		}
	}
}

// `pad` is to make this structure 64 bytes, ie one cache line.
#[repr(C)]
struct Cell
{
	val: volatile<*mut void>,
	enq: volatile<*mut Enqueuer>,
	deq: volatile<*mut Dequeuer>,
	pad: [*mut void; 5],
}

impl Cell
{
}

#[repr(C)]
struct Node
{
	next: CacheAligned<volatile<*mut Node>>,
	id: CacheAligned<isize>,
	cells: CacheAligned<[Cell; WFQUEUE_NODE_SIZE]>,
}

impl Node
{
	#[inline(always)]
	fn new_node() -> NonNull<Self>
	{
		let n = align_malloc(PAGE_SIZE, size_of::<Self>());
		memset(n, 0, size_of::<Self>());
		n
	}
	
	fn find_cell<'node>(ptr: &'node volatile<NonNull<Node>>, i: isize, mut th: NonNull<WaitFreeQueuePerThreadHandle>) -> &'node Cell
	{
		let mut curr = ptr.get();
		
		let mut j = curr.reference().id.get();
		while j < i / N
		{
			let mut next: *mut Node = curr.reference().next.get();
			
			if next.is_null()
			{
				let mut temp = th.reference().spare.get();
				if temp.is_null()
				{
					let new_node = Self::new_node();
					temp = new_node.as_ptr();
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
		
		unsafe { &* curr.as_ptr() }.cells.get_cell(i % N)
	}
}

trait NodeNonNull<T>: SafeNonNull<T>
{
	#[inline(always)]
	fn id(self) -> isize;
	
	#[inline(always)]
	fn next(self) -> *mut Node;
	
	#[inline(always)]
	fn next_non_null(self) -> NonNull<Node>;
}

impl NodeNonNull<Node> for NonNull<Node>
{
	#[inline(always)]
	fn id(self) -> isize
	{
		self.reference().id.get()
	}
	
	#[inline(always)]
	fn next(self) -> *mut Node
	{
		self.reference().next.get()
	}
	
	#[inline(always)]
	fn next_non_null(self) -> NonNull<Node>
	{
		self.next().to_non_null()
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
struct WaitFreeQueueInner
{
	// Index of the next position for enqueue.
	Ei: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the next position for dequeue.
	Di: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the head of the queue.
	Hi: DoubleCacheAligned<volatile<isize>>,
	
	// Pointer to the head node of the queue.
	Hp: volatile<*mut Node>,
	
	// Number of processors.
	number_of_threads: usize,

	_tail: volatile<*mut WaitFreeQueuePerThreadHandle>,
}

impl WaitFreeQueueInner
{
	const MaximumPatienceForFastPath: isize = 10;
	
	pub(crate) fn new(number_of_threads: usize) -> NonNull<Self>
	{
		assert!(number_of_threads <= MaximumNumberOfThreads, "number_of_threads '{}' exceeds MaximumNumberOfThreads '{}'", number_of_threads, MaximumNumberOfThreads);
		
		let mut this = align_malloc(PAGE_SIZE, size_of::<Self>());
		
		unsafe
		{
			let this: &mut Self = this.mutable_reference();
			
			this.Hi.set(0);
			this.Hp.set(Node::new_node().as_ptr());
			this.Ei.set(1);
			this.Di.set(1);
			write(&mut this.number_of_threads, number_of_threads);
			this._tail.set(null_mut());
		}
		
		this
	}
	
	#[inline(always)]
	pub(crate) fn enqueue(&self, mut per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, v: *mut void)
	{
		per_thread_handle.reference().hzd_node_id.set(per_thread_handle.reference().enq_node_id);
		
		let mut id = unsafe { uninitialized() };
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		while !self.enqueue_fast_path(per_thread_handle, v, &mut id) && remaining_patience_for_fast_path.post_decrement() > 0
		{
		}
		if remaining_patience_for_fast_path < 0
		{
			self.enqueue_slow_path(per_thread_handle, v, id)
		}
		
		per_thread_handle.mutable_reference().enq_node_id = per_thread_handle.reference().Ep.get().reference().id.get() as usize;
		per_thread_handle.reference().hzd_node_id.RELEASE(!0)
	}
	
	#[inline(always)]
	pub(crate) fn dequeue(&self, mut per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>) -> *mut void
	{
		per_thread_handle.reference().hzd_node_id.set(per_thread_handle.reference().deq_node_id);
		
		let mut dequeued_value;
		let mut id = 0;
		let mut remaining_patience_for_fast_path = Self::MaximumPatienceForFastPath;
		
		do_while!
		{
			do
			{
				dequeued_value = self.dequeue_fast_path(per_thread_handle, &mut id);
			}
			while dequeued_value == TOP && remaining_patience_for_fast_path.post_decrement() > 0
		}
		
		if dequeued_value == TOP
		{
			dequeued_value = self.dequeue_slow_path(per_thread_handle, id);
		}
		
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
			per_thread_handle.mutable_reference().spare.set(Node::new_node().as_ptr());
		}
		
		dequeued_value
	}
	
	#[inline(always)]
	fn enqueue_fast_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, v: *mut void, id: &mut isize) -> bool
	{
		let i = self.Ei.FAAcs(1);
		
		let c = Node::find_cell(&per_thread_handle.reference().Ep, i, per_thread_handle);
		let mut cv: *mut void = BOT;
		
		if c.val.CAS(&mut cv, v)
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
	fn enqueue_slow_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, v: *mut void, mut id: isize)
	{
		let enq: &Enqueuer = &per_thread_handle.reference().Er;
		enq.val.set(v);
		enq.id.RELEASE(id);

		let tail = &per_thread_handle.reference().Ep;
		let mut i: isize;
		let mut c: &Cell;
		
		'do_while: while
		{
			i = self.Ei.FAA(1);
			c = Node::find_cell(tail, i, per_thread_handle);
			let mut ce = BOT as *mut Enqueuer; // TODO: null_mut()
			
			if c.enq.CAScs(&mut ce, enq as *const _ as *mut _) && c.val.get() != TOP
			{
				if enq.id.CAS(&mut id, -i)
				{
					// Rust compiler has determined that this value is never read.
					// id = -1;
				}
				break 'do_while;
			}
			enq.id.get() > 0
		}
		{
		}
		
		id = -enq.id.get();
		c = Node::find_cell(&per_thread_handle.reference().Ep, id, per_thread_handle);
		if id > i
		{
			let mut Ei: isize = self.Ei.get();
			while Ei <= id && !self.Ei.CAS(&mut Ei, id + 1)
			{
			}
		}
		c.val.set(v);
	}
	
	// Used only when dequeue() is called.
	#[inline(always)]
	fn enqueue_help(&self, mut per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, c: &Cell, i: isize) -> *mut void
	{
		#[inline(always)]
		fn spin(p: &volatile<*mut void>) -> *mut void
		{
			let mut patience = MAX_SPIN;
			let mut v = p.get();
			
			while v.is_not_null() && patience.post_decrement() > 0
			{
				v = p.get();
				spin_loop_hint();
			}
			
			v
		}
		
		let mut v: *mut void = spin(&c.val);
		
		if (v != TOP && v != BOT) || (v == BOT && !c.val.CAScs(&mut v, TOP) && v != TOP)
		{
			return v;
		}
		
		let mut e = c.enq.get();
		
		// TODO: null_mut()
		if e == (BOT as *mut Enqueuer)
		{
			let mut ph = per_thread_handle.reference().Eh.get();
			let (mut pe, mut id) =
				{
					let pe = ph.reference().Er.deref();
					(pe as *const _ as *mut _, pe.id.get())
				};
			
			if per_thread_handle.reference().Ei != 0 && per_thread_handle.reference().Ei != id
			{
				per_thread_handle.mutable_reference().Ei = 0;
				per_thread_handle.mutable_reference().Eh.set(ph.reference().next.get());
				
				ph = per_thread_handle.reference().Eh.get();
				let (pe2, id2) =
				{
					let pe = ph.reference().Er.deref();
					(pe as *const _ as *mut _, pe.id.get())
				};
				pe = pe2;
				id = id2;
			}
			
			if id > 0 && id <= i && !c.enq.CAS(&mut e, pe)
			{
				per_thread_handle.mutable_reference().Ei = id
			}
			else
			{
				per_thread_handle.mutable_reference().Eh.set(ph.reference().next.get())
			}
			
			// TODO: null_mut()
			if e == (BOT as *mut Enqueuer) && c.enq.CAS(&mut e, TOP as *mut Enqueuer)
			{
				e = TOP as *mut Enqueuer;
			}
		}
		
		if e == (TOP as *mut Enqueuer)
		{
			return if self.Ei.get() <= i
			{
				BOT
			}
			else
			{
				TOP
			}
		}
		
		let mut ei = e.to_non_null().reference().id.ACQUIRE();
		let ev = e.to_non_null().reference().val.ACQUIRE();
		
		if ei > i
		{
			if c.val.get() == TOP && self.Ei.get() <= i
			{
				return BOT
			}
		}
		else
		{
			if (ei > 0 && e.to_non_null().reference().id.CAS(&mut ei, -i)) || (ei == -i && c.val.get() == TOP)
			{
				let mut Ei = self.Ei.get();
				while Ei <= i && !self.Ei.CAS(&mut Ei, i + 1)
				{
				}
				c.val.set(ev);
			}
		}
		
		c.val.get()
	}
	
	#[inline(always)]
	fn dequeue_fast_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, id: &mut isize) -> *mut void
	{
		let i = self.Di.FAAcs(1);
		let c = Node::find_cell(&per_thread_handle.reference().Dp, i, per_thread_handle);
		let dequeued_value = self.enqueue_help(per_thread_handle, c, i);
		let mut cd = BOT as *mut Dequeuer;
		
		if dequeued_value == BOT
		{
			return BOT
		}
		
		if dequeued_value != TOP && c.deq.CAS(&mut cd, TOP as *mut Dequeuer)
		{
			return dequeued_value
		}
		
		*id = 1;
		TOP
	}
	
	#[inline(always)]
	fn dequeue_slow_path(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, id: isize) -> *mut void
	{
		let deq = per_thread_handle.reference().Dr.deref();
		deq.id.RELEASE(id);
		deq.idx.RELEASE(id);
		
		self.dequeue_help(per_thread_handle, per_thread_handle);
		let i = -deq.idx.get();
		let c = Node::find_cell(&per_thread_handle.reference().Dp, i, per_thread_handle);
		let dequeued_value = c.val.get();
		
		if dequeued_value == TOP
		{
			BOT
		}
		else
		{
			dequeued_value
		}
	}
	
	#[inline(always)]
	fn dequeue_help(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>, ph: NonNull<WaitFreeQueuePerThreadHandle>)
	{
		let deq = ph.reference().Dr.deref();
		let mut idx = deq.idx.ACQUIRE();
		let id = deq.id.get();
		
		if idx < id
		{
			return;
		}
		
		let Dp = &ph.reference().Dp;
		per_thread_handle.reference().hzd_node_id.set(ph.reference().hzd_node_id.get());
		FENCE();
		idx = deq.idx.get();
		
		let mut i = id + 1;
		let mut old = id;
		let mut new = 0;
		
		loop
		{
			let h = Dp;
			
			while idx == old && new == 0
			{
				let c = Node::find_cell(h, i, per_thread_handle);
				
				let mut Di = self.Di.get();
				while Di <= i && !self.Di.CAS(&mut Di, i + 1)
				{
				}
				
				let v = self.enqueue_help(per_thread_handle, c, i);
				if v == BOT || (v != TOP && c.deq.get() == (BOT as *mut Dequeuer))
				{
					new = i;
				}
				else
				{
					idx = deq.idx.ACQUIRE();
				}
				
				
				i.pre_increment();
			}
			
			if new != 0
			{
				if deq.idx.CASra(&mut idx, new)
				{
					idx = new;
				}
				if idx >= new
				{
					new = 0;
				}
			}
			
			if idx < 0 || deq.id.get() != id
			{
				break;
			}
			
			let c = Node::find_cell(Dp, idx, per_thread_handle);
			let mut cd = BOT as *mut Dequeuer;
			if c.val.get() == TOP || c.deq.CAS(&mut cd, deq as *const _ as *mut _) || cd == (deq as *const _ as *mut _)
			{
				let negative_idx = -idx;
				deq.idx.CAS(&mut idx, negative_idx);
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
	fn clean_up_garbage_after_dequeue(&self, per_thread_handle: NonNull<WaitFreeQueuePerThreadHandle>)
	{
		#[inline(always)]
		fn check(p_hzd_node_id: &volatile<usize>, mut cur: NonNull<Node>, old: *mut Node) -> NonNull<Node>
		{
			let hzd_node_id: usize = p_hzd_node_id.ACQUIRE();
			
			if hzd_node_id < (cur.id() as usize)
			{
				let mut tmp = old.to_non_null();
				while (tmp.id() as usize) < hzd_node_id
				{
					tmp = tmp.next_non_null();
				}
				cur = tmp;
			}
			
			cur
		}
		
		#[inline(always)]
		fn update(pPn: &volatile<NonNull<Node>>, mut cur: NonNull<Node>, p_hzd_node_id: &volatile<usize>, old: *mut Node) -> NonNull<Node>
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
		
		if new.id() - oid < self.maximum_garbage()
		{
			return;
		}
		
		if !self.Hi.CASa(&mut oid, NoOid)
		{
			return;
		}
		
		let mut old = self.Hp.get();
		let mut ph = per_thread_handle;
		
		// Was dimensioned by q.reference().nprocs, but variable stack arrays aren't supported by Rust.
		// handle_t *phs[q->nprocs];  ie let mut phs: [*mut handle_t; q.reference().nprocs]
		// We could use a Vec here but a heap allocation seems overkill, unless we keep it with the thread handle.
		let mut phs: [NonNull<WaitFreeQueuePerThreadHandle>; MaximumNumberOfThreads] = unsafe { uninitialized() };
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
	
	#[inline(always)]
	fn maximum_garbage(&self) -> isize
	{
		let result = 2 * self.number_of_threads;
		debug_assert!(result <= ::std::isize::MAX as usize, "maximum_garbage exceeds isize::MAX");
		result as isize
	}
}

struct WaitFreeQueuePerThreadHandle
{
	// Pointer to the next thread handle; a singularly linked list.
	// Can pointer to self if the first item in the singularly linked list.
	next: ExtendedNonNullAtomicPointer<WaitFreeQueuePerThreadHandle>,
	
	// Hazard pointer.
	hzd_node_id: volatile<usize>,
	
	// Pointer to the node for enqueue.
	Ep: volatile<NonNull<Node>>,
	// Obtained originally from self.Ep.id, assigned to self.hzd_node_id; a kind of cache of the original value of id.
	enq_node_id: usize,
	
	// Pointer to the node for dequeue.
	Dp: volatile<NonNull<Node>>,
	// Obtained originally from self.Dp.id, assigned to hzd_node_id; a kind of cache of the original value of id.
	deq_node_id: usize,
	
	// Enqueue request.
	Er: CacheAligned<Enqueuer>,
	
	// Dequeue request.
	Dr: CacheAligned<Dequeuer>,
	
	// Handle of the next enqueuer to help.
	Eh: CacheAligned<NonNull<WaitFreeQueuePerThreadHandle>>,
	
	Ei: isize,
	
	// Handle of the next dequeuer to help.
	Dh: NonNull<WaitFreeQueuePerThreadHandle>,
	
	// Pointer to a spare node to use, to speedup adding a new node.
	spare: CacheAligned<*mut Node>,
}

impl WaitFreeQueuePerThreadHandle
{
	pub(crate) fn thread_init(q: NonNull<WaitFreeQueueInner>, nprocs: usize) -> NonNull<Self>
	{
		assert!(nprocs <= MaximumNumberOfThreads, "nprocs '{}' exceeds MaximumNumberOfThreads '{}'", nprocs, MaximumNumberOfThreads);
		
		let th = align_malloc(PAGE_SIZE, size_of::<Self>());
		Self::queue_register(q, th);
		th
	}
	
	/// Called once per thread from `thread_init`.
	/// id is a thread id, but not derived from the thread itself - it is assumed to start at 1.
	fn queue_register(q: NonNull<WaitFreeQueueInner>, mut th: NonNull<WaitFreeQueuePerThreadHandle>)
	{
		let q = q.reference();
		let th = th.mutable_reference();
		
		unsafe
		{
			// Seems to be unnecessary as this value is always overwritten.
			// th.next.set(null_mut());
			th.hzd_node_id.set(!0); // Was -1
			
			th.Ep.set(q.Hp.get().to_non_null());
			// enq_node_id can become a hazard node id. As such, the value can be -1 which converts to !0.
			write(&mut th.enq_node_id, th.Ep.get().id() as usize);
			
			th.Dp.set(q.Hp.get().to_non_null());
			// deq_node_id can become a hazard node id. As such, the value can be -1 which converts to !0.
			write(&mut th.deq_node_id, th.Dp.get().id() as usize);
			
			write(&mut th.Er, CacheAligned::new(Enqueuer::new(0, BOT)));
			
			write(&mut th.Dr, CacheAligned::new(Dequeuer::new(0, -1)));
			
			write(&mut th.Ei, 0);
			
			write(&mut th.spare, CacheAligned::new(Node::new_node().as_ptr()));
			
			let mut tail = q._tail.get();
			
			if tail.is_null()
			{
				let th_self = NonNull::new_safe(th);
				th.next.set(th_self);
				if q._tail.CASra(&mut tail, th)
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
