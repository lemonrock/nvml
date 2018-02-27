// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// -2 makes Node<T> exactly 8192 bytes, or 2 pages.
// -3 makes OwnedFreeListElement<Node<Value>> 8192 bytes (OwnedFreeListElement has a 8 byte next pointer for the first field).
const ExclusiveMaximumNumberOfItems: usize = 1024 - 3;

/// A node.
pub struct Node<Value: CtoSafe>
{
	dequeue_index_in_items: AtomicU32,
	items: [AtomicPtr<Value>; ExclusiveMaximumNumberOfItems],
	enqueue_index_in_items: AtomicU32,
	next: AtomicPtr<FreeListElement<Node<Value>>>,
}

impl<Value: CtoSafe> Debug for Node<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "Node<Value>")
	}
}

impl<Value: CtoSafe> CtoSafe for Node<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		// enqueue_index_in_items is really 'next_enqueue_index_in_items'.
		// It can also equal or exceed ExclusiveMaximumIndex.
		let enqueue_index_in_items = self.enqueue_index_in_items();
		let exclusive_maximum_index = min(enqueue_index_in_items, u32::ExclusiveMaximumIndex);
		
		let mut dequeue_index_in_items = self.dequeue_index_in_items();
		while dequeue_index_in_items < exclusive_maximum_index
		{
			let item = self.item(dequeue_index_in_items).load(Relaxed);
			item.to_non_null().mutable_reference().cto_pool_opened(cto_pool_arc);
			dequeue_index_in_items += 1
		}
		
		let next = self.next();
		if next.is_not_null()
		{
			OwnedFreeListElement::from_non_null_pointer(next).cto_pool_opened(cto_pool_arc)
		}
	}
}

impl<Value: CtoSafe> Node<Value>
{
	const ExclusiveMaximumNumberOfItems: usize = ExclusiveMaximumNumberOfItems;
	
	const TakenSentinel: *mut Value = !0 as *mut Value;
	
	// Starts with the first entry pre-filled and enqueue_index_in_items at 1.
	#[inline(always)]
	fn initialize_for_next(&mut self, item: NonNull<Value>)
	{
		self.initialize_internal(item.as_ptr(), 1)
	}
	
	// Starts with no first entry pre-filled and enqueue_index_in_items at 0.
	#[inline(always)]
	fn initialize_for_initial(&mut self)
	{
		self.initialize_internal(null_mut(), 0)
	}
	
	#[inline(always)]
	fn initialize_internal(&mut self, item: *mut Value, enqueue_index_in_items: u32)
	{
		debug_assert_ne!(item, Self::TakenSentinel, "item pointer can not be the TakenSentinel '0x{:X}'", Self::TakenSentinel as usize);
		
		self.dequeue_index_in_items.initialize(0);
		self.enqueue_index_in_items.initialize(enqueue_index_in_items);
		self.next.initialize(null_mut());
		
		debug_assert_ne!(Self::ExclusiveMaximumNumberOfItems, 0, "ExclusiveMaximumNumberOfItems should not be zero");
		self.store_relaxed_item(0, item);
		
		let mut item_index = 1;
		while item_index < (Self::ExclusiveMaximumNumberOfItems as u32)
		{
			self.store_relaxed_item(item_index, null_mut());
			item_index += 1;
		}
	}
	
	#[inline(always)]
	fn enqueue_index_in_items(&self) -> u32
	{
		self.enqueue_index_in_items.load(SeqCst)
	}
	
	#[inline(always)]
	fn fetch_then_increment_enqueue_index_in_items(&self) -> u32
	{
		self.enqueue_index_in_items.fetch_add(1, SeqCst)
	}
	
	#[inline(always)]
	fn dequeue_index_in_items(&self) -> u32
	{
		self.dequeue_index_in_items.load(SeqCst)
	}
	
	#[inline(always)]
	fn fetch_then_increment_dequeue_index_in_items(&self) -> u32
	{
		self.dequeue_index_in_items.fetch_add(1, SeqCst)
	}
	
	#[inline(always)]
	fn next(&self) -> *mut FreeListElement<Self>
	{
		self.next.load(SeqCst)
	}
	
	#[inline(always)]
	fn next_compare_and_swap_strong_sequentially_consistent_if_next_is_still_null(&self, value: NonNull<FreeListElement<Node<Value>>>) -> bool
	{
		self.next.compare_and_swap_strong_sequentially_consistent(null_mut(), value.as_ptr())
	}
	
	#[inline(always)]
	fn store_relaxed_item(&self, next_enqueue_index: u32, item: *mut Value)
	{
		self.item(next_enqueue_index).store(item, Relaxed);
	}
	
	#[inline(always)]
	fn compare_and_swap_strong_sequentially_consistent_item(&self, next_enqueue_index: u32, item: NonNull<Value>) -> bool
	{
		let item = item.as_ptr();
		debug_assert_ne!(item, Self::TakenSentinel, "item pointer can not be the TakenSentinel '0x{:X}'", Self::TakenSentinel as usize);
		self.item(next_enqueue_index).compare_and_swap_strong_sequentially_consistent(null_mut(), item)
	}
	
	#[inline(always)]
	fn swap_item_for_taken(&self, next_dequeue_index: u32) -> *mut Value
	{
		let item = self.item(next_dequeue_index).swap(Self::TakenSentinel, SeqCst);
		debug_assert_ne!(item, Self::TakenSentinel, "item pointer can not be the TakenSentinel '0x{:X}'", Self::TakenSentinel as usize);
		item
	}
	
	#[inline(always)]
	fn item(&self, item_index: u32) -> &AtomicPtr<Value>
	{
		debug_assert!((item_index as usize) < Self::ExclusiveMaximumNumberOfItems, "item_index '{}' exceeds Self::ExclusiveMaximumNumberOfItems '{}'", item_index, Self::ExclusiveMaximumNumberOfItems);
		
		unsafe { self.items.get_unchecked(item_index as usize) }
	}
}
