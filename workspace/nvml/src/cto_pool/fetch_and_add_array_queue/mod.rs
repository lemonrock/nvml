// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use ExtendedNonNull;
use ToNonNull;
use super::*;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::sync::atomic::AtomicI32;
use ::std::sync::atomic::AtomicPtr;
use ::std::sync::atomic::Ordering::Relaxed;


trait NonAtomicInitialization<T>
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: T);
}

impl NonAtomicInitialization<i32> for AtomicI32
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: i32)
	{
		unsafe { (self as *mut Self).write(Self::new(initial_value)) }
	}
}

impl<T> NonAtomicInitialization<*mut T> for AtomicPtr<T>
{
	#[inline(always)]
	fn initialize(&mut self, initial_value: *mut T)
	{
		unsafe { (self as *mut Self).write(Self::new(initial_value)) }
	}
}

const ExclusiveMaximumNumberOfItems: usize = 1024;

struct Node<T>
{
	deqidx: AtomicI32,
	items: [AtomicPtr<T>; ExclusiveMaximumNumberOfItems],
	enqidx: AtomicI32,
	next: AtomicPtr<Node<T>>,
}

impl<T> Node<T>
{
	const ExclusiveMaximumNumberOfItems: usize = ExclusiveMaximumNumberOfItems;
	
	// Start with the first entry pre-filled and enqidx at 1.
	#[inline(always)]
	fn new(item: *mut T, cto_pool_arc: &CtoPoolArc) -> Result<NonNull<Self>, PmdkError>
	{
		let mut this = match cto_pool_arc.pool_pointer().malloc::<Self>()
		{
			Err(pmdk_error) => return Err(pmdk_error),
			Ok(pointer) => pointer.to_non_null(),
		};
		
		// TODO: Replace with a memset of zero followed by this.enqidx, this.items[0], fence(Relaxed)?
		{
			let this = this.mutable_reference();
			this.deqidx.initialize(0);
			this.enqidx.initialize(1);
			this.next.initialize(null_mut());
			
			this.relaxed_store_of_item(0, item);
			
			let mut item_index = 1;
			while item_index < Self::ExclusiveMaximumNumberOfItems
			{
				this.relaxed_store_of_item(item_index, null_mut());
				item_index += 1;
			}
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	fn relaxed_store_of_item(&self, item_index: usize, item: *mut T)
	{
		self.item(item_index).store(item, Relaxed);
	}
	
	#[inline(always)]
	fn item(&self, item_index: usize) -> &AtomicPtr<T>
	{
		debug_assert!(item_index < Self::ExclusiveMaximumNumberOfItems, "item_index '{}' exceeds Self::ExclusiveMaximumNumberOfItems '{}'", item_index, Self::ExclusiveMaximumNumberOfItems);
		
		unsafe { self.items.get_unchecked(item_index) }
	}
}
/*
   struct Node {
        std::atomic<int>   deqidx;
        std::atomic<T*>    items[BUFFER_SIZE];
        std::atomic<int>   enqidx;
        std::atomic<Node*> next;

        // Start with the first entry pre-filled and enqidx at 1
        Node(T* item) : deqidx{0}, enqidx{1}, next{nullptr} {
            items[0].store(item, std::memory_order_relaxed);
            for (long i = 1; i < BUFFER_SIZE; i++) {
                items[i].store(nullptr, std::memory_order_relaxed);
            }
        }

        bool casNext(Node *cmp, Node *val) {
            return next.compare_exchange_strong(cmp, val);
        }
};
*/
