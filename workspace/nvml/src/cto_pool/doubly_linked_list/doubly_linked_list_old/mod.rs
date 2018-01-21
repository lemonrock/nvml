// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// This implementation based on 2004 Paper: "Lock-Free and Practical Doubly Linked List-Based Deques Using Single-Word Compare-and-Swap", H ̊akan Sundell and Philippas Tsigas, T. Higashino (Ed.): OPODIS 2004, LNCS 3544, pp. 240–255, 2005.


use ::std::marker::PhantomData;
use ::std::ptr::null;
use ::std::ptr::write;
use ::std::sync::atomic::fence;
use ::std::sync::atomic::hint_core_should_pause;
use ::std::sync::atomic::AtomicUsize;
use ::std::sync::atomic::Ordering;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;


include!("BackOff.rs");
include!("IsNotNull.rs");


/// Most be the same size as a 'word'
#[derive(Debug, Copy, Clone)]
struct Link<T>
{
	tagged_pointer: usize,
	phantom_data: PhantomData<T>,
}

impl<T> Default for Link<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			tagged_pointer: 0,
			phantom_data: PhantomData,
		}
	}
}

impl<T> PartialEq for Link<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		self.tagged_pointer == other.tagged_pointer
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		self.tagged_pointer != other.tagged_pointer
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
	
	#[inline(always)]
	fn new_with_delete_mark(node_pointer: &Node<T>) -> Self
	{
		let pointer = Self::ref_to_ptr_to_usize(node_pointer);
		
		Self
		{
			tagged_pointer: pointer | Self::IsDeleteMarkBit,
			phantom_data: PhantomData,
		}
	}
	
	#[inline(always)]
	fn new_in_use(node_pointer: &Node<T>) -> Self
	{
		Self
		{
			tagged_pointer: Self::ref_to_ptr_to_usize(node_pointer),
			phantom_data: PhantomData,
		}
	}
	
	#[inline(always)]
	fn ref_to_ptr_to_usize(node_pointer: &Node<T>) -> usize
	{
		let pointer = node_pointer as *const Node<T> as usize;
		debug_assert_eq!(pointer % Self::MinimumPointerAlignment, 0, "pointer must not have its Least Significant Bit (LSB) set, ie it it most have an alignment of at least {}", Self::MinimumPointerAlignment);
		pointer
	}
	
	#[inline(always)]
	fn p<'a>(self) -> &'a Node<T>
	{
		let tagged_pointer = self.tagged_pointer;
		
		unsafe { &* ((tagged_pointer & Self::PointerMask) as *const Node<T>) }
	}
	
	#[inline(always)]
	fn d(self) -> bool
	{
		let tagged_pointer = self.tagged_pointer;
		
		tagged_pointer & Self::IsDeleteMarkBit == Self::IsDeleteMarkBit
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn CAS(&self, oldvalue: Self, newvalue: Self) -> bool
	{
		let address = self;
		
		unimplemented!();
	}
	
	/// The function `DEREF` atomically de-references the given link and increases the reference counter for the corresponding node.
	/// In case the deletion mark of the link is set, the `DEREF` function then returns `NULL`.
	#[inline(always)]
	fn DEREF(&self) -> *const Node<T>
	{
		let address = self;
		
		unimplemented!();
	}
	
	/// The function `DEREF_D` atomically de-references the given link and increases the reference counter for the corresponding node.
	#[inline(always)]
	fn DEREF_D<'a>(&self) -> &'a Node<T>
	{
		let address = self;
		
		unimplemented!();
	}
	
	#[inline(always)]
	fn to_usize_to_atomic_store(self) -> usize
	{
		self.tagged_pointer
	}
	
	#[inline(always)]
	fn to_atomic_usize_to_atomic_store(self) -> AtomicUsize
	{
		AtomicUsize::new(self.to_usize_to_atomic_store())
	}
	
	#[inline(always)]
	fn from_usize_from_atomic_load(tagged_pointer: usize) -> Self
	{
		Self
		{
			tagged_pointer,
			phantom_data: PhantomData,
		}
	}
}

#[derive(Debug)]
struct Node<T>
{
	value: Option<Box<T>>,
	_prev: AtomicUsize,
	_next: AtomicUsize,
	
	// Not part of algorithm per-se.
	reference_count: AtomicUsize,
}

impl<T> Node<T>
{
	/// After a new node has been successfully inserted, the algorithm tries in the loop P1 to update the `prev` pointer of the next node.
	/// It retries until either i) it succeeds with the update, ii) it detects that either the next or new node is deleted, or iii) the next node is no longer directly next of the new node.
	/// In any of the two latter, the changes are due to concurrent Pop or Push operations, and the responsibility to update the `prev` pointer is then left to those.
	/// If the update succeeds, there is though the possibility that the new node was deleted (and thus the `prev` pointer of the next node was possibly already updated by the concurrent Pop operation) directly before the CAS at P5, and then the prev pointer is updated by calling the `HelpInsert` function at P10.
	#[inline(always)]
	fn PushCommon(&self, next: &Self)
	{
		let node = self;
		
		// P1
		loop
		{
			let link1 = next.get_prev(Acquire);
			if link1.d() || node.get_next(Acquire) != Link::new_in_use(next)
			{
				break
			}
			
			// P5
			if next.prev.CAS(link1, Link::new_in_use(node))
			{
				node.COPY();
				link1.p().REL();
				if node.get_prev(Acquire).d()
				{
					let prev2 = node.COPY();
					
					// P10
					let prev2 = prev2.HelpInsert(next);
					prev2.REL();
				}
				break
			}
			
			BackOff()
		}
		
		next.REL();
		node.REL()
	}
	
	/// The `HelpInsert` function tries to update the `prev` pointer of a node and then return a reference to a possibly direct previous node, thus fulfilling step 2 of the overall insertion scheme or step 4 of the overall deletion scheme.
	/// The algorithm repeatedly tries loop HI2 to correct the `prev` pointer of the given node (node), given a suggestion of a previous (not necessarily the directly previous) node (`prev`).
	/// Before trying to update the `prev` pointer with the CAS operation at HI29, it assures at HI4 that the `prev` node is not marked, at HI19 that node is not marked, and at HI22 that `prev` is the previous node of `node`.
	/// If `prev` is marked we might need to delete it before we can update `prev` to one of its previous nodes and proceed with the current deletion.
	/// This extra deletion is only attempted if a `next` pointer from a non-marked node to `prev` has been observed (ie last is valid).
	/// If node is marked, the procedure is aborted.
	/// Otherwise if `prev` is not the previous node of `node` it is updated to be the next node.
	/// If the update at HI29 succeeds, there is though the possibility that the `prev` node was deleted (and thus the `prev` pointer of node was possibly already updated by the concurrent Pop operation) directly before the CAS operation.
	/// This is detected at HI32 and then the update is possibly retried with a new `prev` node.
	fn HelpInsert(&self, node: &Self) -> &Self
	{
		let mut prev = self;
		let mut last: *const Self = null();
		
		// HI2
		loop
		{
			let mut prev2 = prev.get_next(Acquire).DEREF();
			
			// HI4
			if prev2.is_null()
			{
				if last.is_not_null()
				{
					let last_non_null = Self::to_ref(last);
					prev.MarkPrev();
					let next2 = prev.get_next(Acquire).DEREF_D();
					if last_non_null.next.CAS(Link::new_in_use(prev), Link::new_in_use(next2))
					{
						prev.REL()
					}
					else
					{
						next2.REL()
					}
					prev.REL();
					prev = last_non_null;
					last = null();
				}
				else
				{
					/*
						Original algorithm is as below but re-used prev2 when it actually has changed type from being nullable to being non-nullable
						
						prev2 = prev.get_prev(Acquire).DEREF_D();
						prev.REL();
						prev = prev2;
						
						Since a 'continue' follows this else clause, prev2 is never re-used
					*/
					let actually_a_different_prev2 = prev.get_prev(Acquire).DEREF_D();
					prev.REL();
					prev = actually_a_different_prev2;
				}
				continue
			}
			
			let prev2 = Self::to_ref(prev2);
			
			let link1 = node.get_prev(Acquire);
			
			// HI19
			if link1.d()
			{
				prev2.REL();
				break
			}
			
			// HI22
			if prev2.pointers_are_not_the_same(node)
			{
				if last.is_not_null()
				{
					Self::to_ref(last).REL()
				}
				last = prev as *const Self;
				prev = prev2;
				continue
			}
			
			prev2.REL();
			
			if link1.p().pointers_are_the_same(prev)
			{
				break
			}
			
			// HI29
			// NOTE: Is it permissible (or even expected) that this is a short-cut and (&&)?
			if prev.get_next(Acquire) == node && node.prev.CAS(link1, Link::new_in_use(prev))
			{
				prev.COPY();
				link1.p().REL();
				if !prev.get_prev(Acquire).d()
				{
					break
				}
			}
			
			BackOff()
		}
		
		if last.is_not_null()
		{
			Self::to_ref(last).REL()
		}
		
		prev
	}
	
	/// The `HelpDelete` function tries to set the deletion mark of the `prev` pointer and then atomically update the `next` pointer of the previous node of the to-be-deleted node, thus fulfilling step 2 and 3 of the overall node deletion scheme.
	/// The algorithm first ensures at HD1 that the deletion mark on the `prev` pointer of the given node is set.
	/// It then repeatedly tries in loop HD5 to delete (in the sense of a chain of next pointers starting from the head node) the given marked node (node) by changing the `next` pointer from the previous non-marked node.
	/// First, we can safely assume that the next pointer of the marked node is always referring to a node (`next`) to the right and the `prev` pointer is always referring to a node (`prev`) to the left (not necessarily the first).
	/// Before trying to update the `next` pointer with the CAS operation at HD34, it assures at HD6 that node is not already deleted, at HD7 that the `next` node is not marked, at HD14 that the `prev` node is not marked, and at HD28 that `prev` is the previous node of `node`.
	/// If `next` is marked, it is updated to be the `next` node. If `prev` is marked we might need to delete it before we can update `prev` to one of its previous nodes and proceed with the current deletion.
	/// This extra deletion is only attempted if a `next` pointer from a non-marked node to `prev` has been observed (ie last is valid).
	/// Otherwise if `prev` is not the previous node of node it is updated to be the next node.
	#[inline(always)]
	fn HelpDelete(&self)
	{
		let node = self;
		
		// HD1
		node.MarkPrev();
		let mut last = null();
		let mut prev = node.get_prev(Acquire).DEREF_D();
		let mut next = node.get_next(Acquire).DEREF_D();
		
		// HD5
		loop
		{
			// HD6
			if Self::pointers_are_the_same(prev, next)
			{
				break
			}
			
			// HD7
			if next.get_next(Acquire).d()
			{
				next.MarkPrev();
				let next2 = next.get_next(Acquire).DEREF_D();
				next.REL();
				next = next2;
				continue
			}
			
			let mut prev2 = prev.get_next(Acquire).DEREF();
			
			// HD14
			if prev2.is_null()
			{
				if last.is_not_null()
				{
					let last_non_null = Self::to_ref(last);
					prev.MarkPrev();
					let next2 = prev.get_next(Acquire).DEREF_D();
					if last_non_null.next.CAS(Link::new_in_use(prev), Link::new_in_use(next2))
					{
						prev.REL()
					}
					else
					{
						next2.REL();
					}
					prev.REL();
					prev = last_non_null;
					last = null();
				}
				else
				{
					/*
						Original algorithm is as below but re-used prev2 when it actually has changed type from being nullable to being non-nullable
						
						prev2 = prev.get_prev(Acquire).DEREF_D();
						prev.REL();
						prev = prev2;
						
						Since a 'continue' follows this else clause, prev2 is never re-used
					*/
					let actually_a_different_prev2 = prev.get_prev(Acquire).DEREF_D();
					prev.REL();
					prev = actually_a_different_prev2;
				}
				continue
			}
			
			let prev2 = Self::to_ref(prev2);
			
			// HD28
			if prev2.pointers_are_not_the_same(node)
			{
				if last.is_not_null()
				{
					Self::to_ref(last).REL()
				}
				last = prev as *const Self;
				prev = prev2;
				continue
			}
			
			prev2.REL();
			
			if prev.next.CAS(Link::new_in_use(node), Link::new_in_use(next))
			{
				next.COPY();
				node.REL();
				break
			}
			BackOff()
		}
		
		if last.is_not_null()
		{
			Self::to_ref(last).REL()
		}
		prev.REL();
		next.REL();
	}
	
	/// Avoiding Cyclic Garbage
	/// The `RemoveCrossReference` function tries to break cross-references between the given node (`node`) and any of the nodes that it references, by repeatedly updating the `prev` and `next` pointer as long as they reference a fully marked node.
	/// First, we can safely assume that the `prev` or `next` field of `node` is not concurrently updated by any other operation, as this procedure is only called by the main operation that deleted the node and both the `next` and `prev` pointers are marked and thus any concurrent update using CAS will fail.
	/// Before the function is finished, it assures at RC3 that the previous node (`prev`) is not fully marked, and at RC9 that the next node (`next`) is not fully marked.
	/// As long as `prev` is marked it is traversed to the left, and as long as `next` is marked it is traversed to the right, while continuously updating the `prev` or `next` field of `node` at RC5 or RC11.
	#[inline(always)]
	fn RemoveCrossReference(&self)
	{
		let node = self;
		
		loop
		{
			let prev = node.get_prev(Acquire).p();
			
			// RC3
			if prev.get_prev(Acquire).d()
			{
				let prev2 = prev.get_prev(Acquire).DEREF_D();
				// RC5
				node.set_prev(Link::new_with_delete_mark(prev2), Release);
				prev.REL();
				continue;
			}
			
			let next = node.get_next(Acquire).p();
			
			// RC9
			if next.get_prev(Acquire).d()
			{
				let next2 = next.get_next(Acquire).DEREF_D();
				// RC11
				node.set_next(Link::new_with_delete_mark(next2), Release);
				next.REL();
				continue;
			}
			
			break;
		}
	}
	
	#[inline(always)]
	fn MarkPrev(&self)
	{
		let node = self;
		
		loop
		{
			let link1 = node.get_prev(Acquire);
			
			// NOTE: Assumption - the or in the Sundell & Tsigas paper is a short-cut or, ||.
			if link1.d() || node.prev.CAS(link1, Link::new_with_delete_mark(link1.p()))
			{
				break
			}
		}
	}
	
	#[inline(always)]
	fn CreateNode(value: Option<Box<T>>) -> *const Self
	{
		let mut node = Self::MALLOC_NODE
		(
			Self
			{
				value,
				_prev: Link::default().to_atomic_usize_to_atomic_store(),
				_next: Link::default().to_atomic_usize_to_atomic_store(),
				reference_count: AtomicUsize::new(1),
			}
		);
		Box::into_raw(node) as *const Self
	}
	
	/// The function `MALLOC NODE` allocates a new node from the memory pool.
	#[inline(always)]
	fn MALLOC_NODE(node: Self) -> Box<Node<T>>
	{
		Box::new(node)
	}
	
	/// The `COPY` function increases the reference counter for the corresponding given node.
	#[inline(always)]
	fn COPY(&self) -> &Self
	{
		let node = self;
		
		node.reference_count.fetch_add(1, Acquire);
		
		node
	}
	
	/// The function `REL` decrements the reference counter on the corresponding given node.
	/// If the reference counter reaches zero, the function then calls the `TerminateNode` function that will recursively call REL on the nodes that this node has owned pointers to, and then it reclaims the node (eg calls `free()`).
	#[inline(always)]
	fn REL(&self)
	{
		let node = self;
		
		// NOTE: This should never reach 1 for the dummy 'head' or 'tail' nodes.
		// 1, not zero, because we fetch the previous value.
		if node.reference_count.fetch_sub(1, Release) == 1
		{
			node.TerminateNode();
			
			let mut memory_allocation = Box::from_raw(node as *const Self as *mut Self);
			
			// We have passed out the reference to value, so we need to make sure we don't double-drop it.
			unsafe { write(&mut memory_allocation.value, None) }
			
			// Free memory
			drop(memory_allocation);
		}
	}
	
	#[inline(always)]
	fn TerminateNode(&self)
	{
		let node = self;
		
		node.get_prev(Acquire).p().REL();
		node.get_next(Acquire).p().REL();
	}
	
	#[inline(always)]
	fn get_next(&self, ordering: Ordering) -> Link<T>
	{
		Link::from_usize_from_atomic_load(self._next.load(ordering))
	}
	
	#[inline(always)]
	fn get_prev(&self, ordering: Ordering) -> Link<T>
	{
		Link::from_usize_from_atomic_load(self._prev.load(ordering))
	}
	
	#[inline(always)]
	fn set_next(&self, value: Link<T>, ordering: Ordering)
	{
		self._next.store(value.to_usize_to_atomic_store(), ordering)
	}
	
	#[inline(always)]
	fn set_prev(&self, value: Link<T>, ordering: Ordering)
	{
		self._prev.store(value.to_usize_to_atomic_store(), ordering)
	}
	
	#[inline(always)]
	fn to_ref<'a>(node: *const Self) -> &'a Self
	{
		debug_assert!(node.is_not_null(), "node is null");
		
		unsafe { &* node }
	}
	
	#[inline(always)]
	fn pointers_are_the_same(&self, right: &Self) -> bool
	{
		self as *const Self == right as *const Self
	}
	
	#[inline(always)]
	fn pointers_are_not_the_same(&self, right: &Self) -> bool
	{
		self as *const Self != right as *const Self
	}
}

/// The Sundell & Tsigas algorithm treats the deque structure as a singly-linked list with `prev` pointers that may not be up to date.
/// We wrap the algorithm's global variables of `head` and `tail` in this struct.
/// It also uses dummy nodes, with missing values, for head and tail nodes.
///
/// Need to also read Sundell, H.: Efficient and Practical Non-Blocking Data Structures. PhD thesis, Department of Computing Science, Chalmers University of Technology (2004).
/// Which seems to discuss iteration and arbitrary insert / delete.
#[derive(Debug)]
pub struct DequeOrDoublyLinkedList<T>
{
	head_and_tail_dummy_node: Node<T>,
}

impl<T> Default for DequeOrDoublyLinkedList<T>
{
	#[inline(always)]
	fn default() -> Self
	{
		let mut this = Self
		{
			head_and_tail_dummy_node: Node
			{
				value: None,
				_prev: Link::default().to_atomic_usize_to_atomic_store(),
				_next: Link::default().to_atomic_usize_to_atomic_store(),
				reference_count: AtomicUsize::new(2), // Since head_and_tail_dummy_node acts as before head and tail and should never be fully released in TerminateNode()
			},
		};
		
		this.head_and_tail_dummy_node.set_next(Link::new_in_use(&this.head_and_tail_dummy_node), Relaxed);
		this.head_and_tail_dummy_node.set_prev(Link::new_in_use(&this.head_and_tail_dummy_node), Relaxed);
		
		fence(Release);
		
		this
	}
}

impl<T> DequeOrDoublyLinkedList<T>
{
	/// The `PushLeft` operation inserts a new node at the leftmost position in the deque.
	/// The algorithm first repeatedly tries in the loop L4 to insert the new node (`node`) between the head node (`prev`) and the leftmost node (`next`), by atomically changing the next pointer of the head node.
	/// Before trying to update the next pointer, it assures in `L5` that the next node is still the very next node of head, otherwise next is updated in `L6`-`L7`.
	/// After the new node has been successfully inserted, it then tries to update the `prev` pointer of the next node using `PushCommon`.
	/// The linearizability point of the `PushLeft` operation is the successful CAS operation in L11.
	#[inline(always)]
	pub fn PushLeft(&self, value: Box<T>)
	{
		let node = unsafe { &* Node::CreateNode(Some(value)) };
		let prev = self.head().COPY();
		// WAS:-
		// 		let mut next = prev.get_next(Acquire).DEREF();
		// But this implies next can be null, which none of the following code allows.
		// See also PopLeft()
		let mut next = prev.get_next(Acquire).DEREF_D();
		
		// L4
		loop
		{
			// L5
			if prev.get_next(Acquire) != Link::new_in_use(next)
			{
				// L6
				next.REL();
				
				// L7
				// WAS:-
				// 		let mut next = prev.get_next(Acquire).DEREF();
				// But this implies next can be null, which none of the following code allows.
				// See also PopLeft()
				next = prev.get_next(Acquire).DEREF_D();
				continue
			}
			
			node.set_prev(Link::new_in_use(prev), Release);
			node.set_next(Link::new_in_use(next), Release);
			
			// L11
			if prev.next.CAS(Link::new_in_use(next), Link::new_in_use(node))
			{
				node.COPY();
				break
			}
			
			BackOff()
		}
		
		node.PushCommon(next)
	}
	
	/// The `PopLeft` operation tries to delete and return the value of the leftmost node in the deque.
	/// The algorithm first repeatedly tries in the loop PL2 to mark the leftmost node (node) as deleted.
	/// Before trying to update the `next` pointer, it first assures at PL4 that the deque is not empty, and secondly at PL9 that the node is not already marked for deletion.
	/// If the deque was detected to be empty, the function returns.
	/// If node was marked for deletion, it tries to update the `next` pointer of the prev node by calling the `HelpDelete` function, and then node is updated to be the leftmost node.
	/// If the `prev` pointer of node was incorrect, it tries to update it by calling the `HelpInsert` function.
	/// After the node has been successfully marked by the successful CAS operation at PL13, it tries at PL14 to update the `next` pointer of the prev node by calling the `HelpDelete` function, and at PL16 to update the `prev` pointer of the next node by calling the `HelpInsert` function.
	/// After this, it tries at PL23 to break possible cyclic references that includes node by calling the `RemoveCrossReference` function.
	/// The linearizability point of a PopLeft operation that fails, is the read operation of the next pointer at PL3.
	/// The linearizability point of a PopLeft operation that succeeds, is the read operation of the next pointer at PL3.
	#[inline(always)]
	pub fn PopLeft(&self) -> Option<Box<T>>
	{
		let mut value = None;
		
		let mut prev = self.head().COPY();
		
		let mut node;
		// PL2
		loop
		{
			// PL3
			//
			// WAS:
			// 		let node = prev.get_next(Acquire).DEREF();
			// But this implies next can be null, which none of the following code allows.
			// See also PushLeft
			node = prev.get_next(Acquire).DEREF_D();
			
			// PL4
			if node.pointers_are_the_same(self.tail())
			{
				node.REL();
				prev.REL();
				return None;
			}
			
			let link1 = node.get_next(Acquire);
			
			// PL9
			if link1.d()
			{
				node.HelpDelete();
				node.REL();
				continue;
			}
			
			// PL13
			if node.next.CAS(link1, Link::new_with_delete_mark(link1.p()))
			{
				// PL14
				node.HelpDelete();
				let next = node.get_next(Acquire).DEREF_D();
				// PL16
				prev = prev.HelpInsert(next);
				prev.REL();
				next.REL();
				value = node.value;
				break
			}
			
			node.REL();
			BackOff();
		}
		
		// PL23
		node.RemoveCrossReference();
		node.REL();
		value
	}
	
	// `head()` must be non-null.
	// `head()` is a dummy node.
	#[inline(always)]
	fn head(&self) -> &Node<T>
	{
		&self.head_and_tail_dummy_node
	}
	
	// `tail()` must be non-null.
	// `tail()` is a dummy node.
	#[inline(always)]
	fn tail(&self) -> &Node<T>
	{
		&self.head_and_tail_dummy_node
	}
}
