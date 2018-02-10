// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]


use IsNotNull;
use ::std::cell::UnsafeCell;
use ::std::intrinsics::atomic_load_acq;
use ::std::mem::size_of;
use ::std::ptr::NonNull;
use ::std::ptr::read_volatile;
use ::std::ptr::write;
use ::std::ptr::write_volatile;
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

enum void
{
}

trait PostFixOperators
{
	// the self-- C operator
	#[inline(always)]
	fn post_decrement(&mut self) -> Self;
}

impl PostFixOperators for i32
{
	#[inline(always)]
	fn post_decrement(&mut self) -> Self
	{
		let value = *self;
		*self = value - 1;
		value
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
	pub(crate) fn set(&self, value: T)
	{
		self.0.set(value)
	}
	
	#[inline(always)]
	pub(crate) fn ACQUIRE(&self) -> T
	{
		self.0.ACQUIRE()
	}
	
	// true if successful
	// false if failed
	#[inline(always)]
	pub(crate) fn CAScs(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CAScs(cmp, val)
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
	
	// true if successful
	// false if failed
	#[inline(always)]
	pub(crate) fn CAScs(&self, cmp: &mut T, val: T) -> bool
	{
		self.0.CAScs(cmp, val)
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
	
	// true if successful
	// false if failed
	#[inline(always)]
	pub(crate) fn CAScs(&self, cmp: &mut T, val: T) -> bool
	{
		let ptr = self;
		// #define CAScs(ptr, cmp, val) __atomic_compare_exchange_n(ptr, cmp, val, 0, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST)
		unimplemented!();
	}
}




const PAGE_SIZE: usize = 4096;

const EMPTY: *mut void = 0 as *mut void;

const WFQUEUE_NODE_SIZE: usize = (1 << 10) - 2;

const N: usize = WFQUEUE_NODE_SIZE;

const BOT: *mut void = 0 as *mut void;

const TOP: *mut void = !0 as *mut void;

const MAX_SPIN: i32 = 100;

const MAX_PATIENCE: i32 = 10;

#[inline(always)]
fn MAX_GARBAGE(n: usize) -> usize
{
	2 * n
}

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

fn check(p_hzd_node_id: &volatile<usize>, mut cur: NonNull<node_t>, old: NonNull<node_t>) -> NonNull<node_t>
{
	let hzd_node_id = p_hzd_node_id.ACQUIRE();
	
	if hzd_node_id < cur.id()
	{
		let mut tmp = old;
		while tmp.id() < hzd_node_id
		{
			tmp = tmp.next_non_null();
		}
		cur = tmp;
	}
	
	cur
}

fn update(pPn: &volatile<NonNull<node_t>>, mut cur: NonNull<node_t>, p_hzd_node_id: &volatile<usize>, old: NonNull<node_t>) -> NonNull<node_t>
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

/*

static void cleanup(queue_t *q, handle_t *th) {
    long oid = ACQUIRE(&q->Hi);
    node_t *new = th->Dp;

    if (oid == -1) return;
    if (new->id - oid < MAX_GARBAGE(q->nprocs)) return;
    if (!CASa(&q->Hi, &oid, -1)) return;

    node_t *old = q->Hp;
    handle_t *ph = th;
    handle_t *phs[q->nprocs];
    int i = 0;

    do {
        new = check(&ph->hzd_node_id, new, old);
        new = update(&ph->Ep, new, &ph->hzd_node_id, old);
        new = update(&ph->Dp, new, &ph->hzd_node_id, old);

        phs[i++] = ph;
        ph = ph->next;
    } while (new->id > oid && ph != th);

    while (new->id > oid && --i >= 0) {
        new = check(&phs[i]->hzd_node_id, new, old);
    }

    long nid = new->id;

    if (nid <= oid) {
        RELEASE(&q->Hi, oid);
    } else {
        q->Hp = new;
        RELEASE(&q->Hi, nid);

        while (old != new) {
            node_t *tmp = old->next;
            free(old);
            old = tmp;
        }
    }
}

static cell_t *find_cell(node_t *volatile *ptr, long i, handle_t *th) {
    node_t *curr = *ptr;

    long j;
    for (j = curr->id; j < i / N; ++j) {
        node_t *next = curr->next;

        if (next == NULL) {
            node_t *temp = th->spare;

            if (!temp) {
                temp = new_node();
                th->spare = temp;
            }

            temp->id = j + 1;

            if (CASra(&curr->next, &next, temp)) {
                next = temp;
                th->spare = NULL;
            }
        }

        curr = next;
    }

    *ptr = curr;
    return &curr->cells[i % N];
}

static int enq_fast(queue_t *q, handle_t *th, void *v, long *id) {
    long i = FAAcs(&q->Ei, 1);
    cell_t *c = find_cell(&th->Ep, i, th);
    void *cv = BOT;

    if (CAS(&c->val, &cv, v)) {
        return 1;
    } else {
        *id = i;
        return 0;
    }
}

static void enq_slow(queue_t *q, handle_t *th, void *v, long id) {
    enq_t *enq = &th->Er;
    enq->val = v;
    RELEASE(&enq->id, id);

    node_t *tail = th->Ep;
    long i;
    cell_t *c;

    do {
        i = FAA(&q->Ei, 1);
        c = find_cell(&tail, i, th);
        enq_t *ce = BOT;

        if (CAScs(&c->enq, &ce, enq) && c->val != TOP) {
            if (CAS(&enq->id, &id, -i)) id = -i;
            break;
        }
    } while (enq->id > 0);

    id = -enq->id;
    c = find_cell(&th->Ep, id, th);
    if (id > i) {
        long Ei = q->Ei;
        while (Ei <= id && !CAS(&q->Ei, &Ei, id + 1))
            ;
    }
    c->val = v;
}

void enqueue(queue_t *q, handle_t *th, void *v) {
    th->hzd_node_id = th->enq_node_id;

    long id;
    int p = MAX_PATIENCE;
    while (!enq_fast(q, th, v, &id) && p-- > 0)
        ;
    if (p < 0) enq_slow(q, th, v, id);

    th->enq_node_id = th->Ep->id;
    RELEASE(&th->hzd_node_id, -1);
}

static void *help_enq(queue_t *q, handle_t *th, cell_t *c, long i) {
    void *v = spin(&c->val);

    if ((v != TOP && v != BOT) ||
        (v == BOT && !CAScs(&c->val, &v, TOP) && v != TOP)) {
        return v;
    }

    enq_t *e = c->enq;

    if (e == BOT) {
        handle_t *ph;
        enq_t *pe;
        long id;
        ph = th->Eh, pe = &ph->Er, id = pe->id;

        if (th->Ei != 0 && th->Ei != id) {
            th->Ei = 0;
            th->Eh = ph->next;
            ph = th->Eh, pe = &ph->Er, id = pe->id;
        }

        if (id > 0 && id <= i && !CAS(&c->enq, &e, pe))
            th->Ei = id;
        else
            th->Eh = ph->next;

        if (e == BOT && CAS(&c->enq, &e, TOP)) e = TOP;
    }

    if (e == TOP) return (q->Ei <= i ? BOT : TOP);

    long ei = ACQUIRE(&e->id);
    void *ev = ACQUIRE(&e->val);

    if (ei > i) {
        if (c->val == TOP && q->Ei <= i) return BOT;
    } else {
        if ((ei > 0 && CAS(&e->id, &ei, -i)) || (ei == -i && c->val == TOP)) {
            long Ei = q->Ei;
            while (Ei <= i && !CAS(&q->Ei, &Ei, i + 1))
                ;
            c->val = ev;
        }
    }

    return c->val;
}

static void help_deq(queue_t *q, handle_t *th, handle_t *ph) {
    deq_t *deq = &ph->Dr;
    long idx = ACQUIRE(&deq->idx);
    long id = deq->id;

    if (idx < id) return;

    node_t *Dp = ph->Dp;
    th->hzd_node_id = ph->hzd_node_id;
    FENCE();
    idx = deq->idx;

    long i = id + 1, old = id, new = 0;
    while (1) {
        node_t *h = Dp;
        for (; idx == old && new == 0; ++i) {
            cell_t *c = find_cell(&h, i, th);

            long Di = q->Di;
            while (Di <= i && !CAS(&q->Di, &Di, i + 1))
                ;

            void *v = help_enq(q, th, c, i);
            if (v == BOT || (v != TOP && c->deq == BOT))
                new = i;
            else
                idx = ACQUIRE(&deq->idx);
        }

        if (new != 0) {
            if (CASra(&deq->idx, &idx, new)) idx = new;
            if (idx >= new) new = 0;
        }

        if (idx < 0 || deq->id != id) break;

        cell_t *c = find_cell(&Dp, idx, th);
        deq_t *cd = BOT;
        if (c->val == TOP || CAS(&c->deq, &cd, deq) || cd == deq) {
            CAS(&deq->idx, &idx, -idx);
            break;
        }

        old = idx;
        if (idx >= i) i = idx + 1;
    }
}

static void *deq_fast(queue_t *q, handle_t *th, long *id) {
    long i = FAAcs(&q->Di, 1);
    cell_t *c = find_cell(&th->Dp, i, th);
    void *v = help_enq(q, th, c, i);
    deq_t *cd = BOT;

    if (v == BOT) return BOT;
    if (v != TOP && CAS(&c->deq, &cd, TOP)) return v;

    *id = i;
    return TOP;
}

static void *deq_slow(queue_t *q, handle_t *th, long id) {
    deq_t *deq = &th->Dr;
    RELEASE(&deq->id, id);
    RELEASE(&deq->idx, id);

    help_deq(q, th, th);
    long i = -deq->idx;
    cell_t *c = find_cell(&th->Dp, i, th);
    void *val = c->val;

    return val == TOP ? BOT : val;
}

void *dequeue(queue_t *q, handle_t *th) {
    th->hzd_node_id = th->deq_node_id;

    void *v;
    long id = 0;
    int p = MAX_PATIENCE;

    do
        v = deq_fast(q, th, &id);
    while (v == TOP && p-- > 0);
    if (v == TOP)
        v = deq_slow(q, th, id);
    else {
#ifdef RECORD
        th->fastdeq++;
#endif
    }

    if (v != EMPTY) {
        help_deq(q, th, th->Dh);
        th->Dh = th->Dh->next;
    }

    th->deq_node_id = th->Dp->id;
    RELEASE(&th->hzd_node_id, -1);

    if (th->spare == NULL) {
        cleanup(q, th);
        th->spare = new_node();
    }

    return v;
}


*/

#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
struct enq_t
{
	id: volatile<isize>,
	
	val: volatile<*mut void>,
}

impl enq_t
{
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(32)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(64)))]
struct deq_t
{
	id: volatile<isize>,
	
	idx: volatile<isize>,
}

impl deq_t
{
}

#[repr(C)]
struct cell_t
{
	val: volatile<*mut void>,
	
	enq: volatile<*mut enq_t>,
	
	deq: volatile<*mut deq_t>,

	pad: [*mut void; 5],
}

impl cell_t
{
}

#[repr(C)]
struct node_t
{
	next: CacheAligned<volatile<*mut node_t>>,
	id: CacheAligned<usize>,
	cells: CacheAligned<[cell_t; WFQUEUE_NODE_SIZE]>,
}

trait NodeNonNull<T>: SafeNonNull<T>
{
	#[inline(always)]
	fn id(self) -> usize;
	
	#[inline(always)]
	fn next(self) -> *mut node_t;
	
	#[inline(always)]
	fn next_non_null(self) -> NonNull<node_t>;
}

impl NodeNonNull<node_t> for NonNull<node_t>
{
	#[inline(always)]
	fn id(self) -> usize
	{
		self.reference().id.get()
	}
	
	#[inline(always)]
	fn next(self) -> *mut node_t
	{
		self.reference().next.get()
	}
	
	#[inline(always)]
	fn next_non_null(self) -> NonNull<node_t>
	{
		self.next().to_non_null()
	}
}

impl node_t
{
	#[inline(always)]
	fn new_node() -> NonNull<Self>
	{
		let n = align_malloc(PAGE_SIZE, size_of::<Self>());
		memset(n, 0, size_of::<Self>());
		n
	}
}

#[cfg_attr(target_pointer_width = "32", repr(C, align(64)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(128)))]
struct queue_t
{
	// Index of the next position for enqueue.
	Ei: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the next position for dequeue.
	Di: DoubleCacheAligned<volatile<isize>>,
	
	// Index of the head of the queue.
	Hi: DoubleCacheAligned<volatile<isize>>,
	
	// Pointer to the head node of the queue.
	Hp: volatile<*mut node_t>,
	
	// Number of processors.
	nprocs: usize,
}

impl queue_t
{
	pub(crate) fn queue_init(mut this: NonNull<Self>, nprocs: usize)
	{
		let this = this.mutable_reference();
		unsafe
		{
			write(&mut this.Hi, DoubleCacheAligned::new(volatile::new(0)));
			write(&mut this.Hp, volatile::new(node_t::new_node().as_ptr()));
			write(&mut this.Ei, DoubleCacheAligned::new(volatile::new(1)));
			write(&mut this.Di, DoubleCacheAligned::new(volatile::new(1)));
			write(&mut this.nprocs, nprocs);
		}
	}
	
	/// Called per thread exit. Does nothing for a wf-queue.
	#[inline(always)]
	pub(crate) fn queue_free(this: NonNull<Self>, handle: NonNull<handle_t>)
	{
	}
	
	/*
	
	void queue_register(queue_t *q, handle_t *th, int id) {
		th->next = NULL;
		th->hzd_node_id = -1;
		th->Ep = q->Hp;
		th->enq_node_id = th->Ep->id;
		th->Dp = q->Hp;
		th->deq_node_id = th->Dp->id;
	
		th->Er.id = 0;
		th->Er.val = BOT;
		th->Dr.id = 0;
		th->Dr.idx = -1;
	
		th->Ei = 0;
		th->spare = new_node();
	
		static handle_t *volatile _tail;
		handle_t *tail = _tail;
	
		if (tail == NULL) {
			th->next = th;
			if (CASra(&_tail, &tail, th)) {
				th->Eh = th->next;
				th->Dh = th->next;
				return;
			}
		}
	
		handle_t *next = tail->next;
		do
			th->next = next;
		while (!CASra(&tail->next, &next, th));
	
		th->Eh = th->next;
		th->Dh = th->next;
	}
	
	*/
	
}

struct handle_t
{
	// Pointer to the next handle.
	next: *mut handle_t,
	
	// Hazard pointer.
	hzd_node_id: volatile<usize>,
	
	// Pointer to the node for enqueue.
	Ep: volatile<*mut node_t>,
	enq_node_id: usize,
	
	// Pointer to the node for dequeue.
	Dp: volatile<*mut node_t>,
	deq_node_id: usize,
	
	// Enqueue request.
	Er: CacheAligned<enq_t>,
	
	// Dequeue request.
	Dr: CacheAligned<deq_t>,
	
	// Handle of the next enqueuer to help.
	Eh: CacheAligned<*mut handle_t>,
	
	Ei: isize,
	
	// Handle of the next dequeuer to help.
	Dh: *mut handle_t,
	
	// Pointer to a spare node to use, to speedup adding a new node.
	spare: CacheAligned<*mut node_t>,
	
	// Count the delay rounds of helping another dequeuer.
	delay: i32,
}

impl handle_t
{
}
