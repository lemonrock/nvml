// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


use super::*;
use ::std::mem::size_of;
use ::std::mem::transmute;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
use ::std::sync::atomic::AtomicU32;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::atomic::AtomicU128;
use ::std::sync::atomic::Ordering::AcqRel;
use ::std::sync::atomic::Ordering::Acquire;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Release;
use ::std::sync::atomic::spin_loop_hint;



pub trait Block
{
	//type P: Persistence;
	
	const BlockSizeInBytes: usize;
}

struct ABA(u32);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TwinBlockPointerWithAbaProtection<B: Block>(u128);

impl<B: Block> TwinBlockPointerWithAbaProtection<B>
{
	const HeadPointerIndex: usize = 0;
	
	const TailPointerIndex: usize = 1;
	
	const AbaIndex: usize = 2;
	
	#[inline(always)]
	fn new_head_and_tail_for_insert(&self, new_head: BlockPointer<B>) -> Self
	{
		let mut head_and_tail = self.clone();
		if self.tail_block_pointer().is_null()
		{
			head_and_tail.set_both_pointers_and_increment_aba(new_head, new_head)
		}
		else
		{
			head_and_tail.set_head_block_pointer_and_increment_aba(new_head)
		}
		head_and_tail
	}
	
	#[inline(always)]
	fn new_head_and_tail_for_remove(&self, new_tail: BlockPointer<B>) -> Self
	{
		let mut head_and_tail = self.clone();
		head_and_tail.set_tail_block_pointer_and_increment_aba(new_tail);
		head_and_tail
	}
	
	#[inline(always)]
	fn set_head_block_pointer_and_increment_aba(&mut self, head_block_pointer: BlockPointer<B>)
	{
		self.set_head_pointer_(head_block_pointer);
		self.increment_aba_()
	}
	
	#[inline(always)]
	fn set_tail_block_pointer_and_increment_aba(&mut self, tail_block_pointer: BlockPointer<B>)
	{
		self.set_tail_pointer_(tail_block_pointer);
		self.increment_aba_()
	}
	
	#[inline(always)]
	fn set_both_pointers_and_increment_aba(&mut self, head_block_pointer: BlockPointer<B>, tail_block_pointer: BlockPointer<B>)
	{
		self.set_head_pointer_(head_block_pointer);
		self.set_tail_pointer_(tail_block_pointer);
		self.increment_aba_()
	}
	
	#[inline(always)]
	fn head_block_pointer(self) -> BlockPointer<B>
	{
		BlockPointer(self.get_(Self::HeadPointerIndex))
	}
	
	#[inline(always)]
	fn tail_block_pointer(self) -> BlockPointer<B>
	{
		BlockPointer(self.get_(Self::TailPointerIndex))
	}
	
	#[inline(always)]
	fn set_head_pointer_(&mut self, new_head_block_pointer: BlockPointer<B>)
	{
		self.set_(Self::HeadPointerIndex, new_head_block_pointer);
	}
	
	#[inline(always)]
	fn set_tail_pointer_(&mut self, new_tail_block_pointer: BlockPointer<B>)
	{
		self.set_(Self::TailPointerIndex, new_tail_pointer);
	}
	
	#[inline(always)]
	fn increment_aba_(&mut self)
	{
		let new_aba = self.aba() + 1;
		self.set_(Self::AbaIndex, new_aba);
	}
	
	#[inline(always)]
	fn get_(self, index: usize) -> u32
	{
		const NumberOfValues: usize = 4;
		debug_assert!(index < NumberOfValues, "index must be less than {}", NumberOfValues);
		
		let pointer = &self.0 as *const u128 as *const u32;
		let offset = index * size_of::<u32>();
		let pointer = unsafe { pointer.offset(offset as isize) };
		unsafe { read(pointer) }
	}
	
	#[inline(always)]
	fn set_(&mut self, index: usize, value: u32)
	{
		const NumberOfValues: usize = 4;
		debug_assert!(index < NumberOfValues, "index must be less than {}", NumberOfValues);
		
		let pointer = &self.0 as *mut u128 as *mut u32;
		let offset = index * size_of::<u32>();
		let pointer = unsafe { pointer.offset(offset as isize) };
		unsafe { write(pointer, value) }
	}
}

#[derive(Debug)]
struct AtomicTwinBlockPointerWithAbaProtection<B: Block>(AtomicU128);

impl<B: Block> AtomicTwinBlockPointerWithAbaProtection<B>
{
	#[inline(always)]
	fn get(&self) -> TwinBlockPointerWithAbaProtection<B>
	{
		TwinBlockPointerWithAbaProtection(self.0.load(Acquire))
	}
	
	#[inline(always)]
	fn set(&self, new_value: TwinBlockPointerWithAbaProtection<B>)
	{
		self.0.store(new_value.0, Release)
	}
	
	#[inline(always)]
	fn compare_and_swap(&self, was_value: TwinBlockPointerWithAbaProtection<B>, new_value: TwinBlockPointerWithAbaProtection<B>) -> Result<(), TwinBlockPointerWithAbaProtection<B>>
	{
		// On success, we are release'ing the pointer. However, Rust forces us to specify `AcqRel`, rather than `Release`.
		// On failure, we acquire the current value of the pointer..
		match self.0.compare_exchange(was_value, new_value, AcqRel, Acquire)
		{
			Ok(_new_previous_block_pointer) => Ok(()),
			Err(updated_value) => Err(TwinBlockPointerWithAbaProtection(updated_value)),
		}
	}
}

/// A compressed pointer, representing an index.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BlockPointer<B: Block>(u32);

impl<B: Block> BlockPointer<B>
{
	const Null: u32 = BlockPointer(::std::u32::MAX);
	
	#[inline(always)]
	fn is_not_null(self) -> bool
	{
		self != Self::NullSentinel
	}
	
	#[inline(always)]
	fn is_null(self) -> bool
	{
		self == Self::NullSentinel
	}
	
	#[inline(always)]
	fn expand_to_pointer_to_memory(self, memory_base_pointer: *mut u8) -> *mut u8
	{
		if self.is_null()
		{
			null_mut()
		}
		else
		{
			self.expand_to_pointer_to_memory_unchecked(memory_base_pointer)
		}
	}
	
	#[inline(always)]
	fn expand_to_pointer_to_memory_unchecked(self, memory_base_pointer: *mut u8) -> *mut u8
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		
		unsafe { memory_base_pointer.offset(B::BlockSizeInBytes * self.0) }
	}
	
	#[inline(always)]
	fn expand_to_pointer_to_meta_data(self, block_meta_data_items: &[BlockMetaData<B>]) -> Option<&BlockMetaData<B>>
	{
		if self.is_null()
		{
			None
		}
		else
		{
			debug_assert!(self.0 < block_meta_data_items.len(), "block_meta_data_items is too short");
			unsafe { block_meta_data_items.get_unchecked(self.0) }
		}
	}
	
	#[inline(always)]
	fn expand_to_pointer_to_meta_data_unchecked(self, block_meta_data_items: &[BlockMetaData<B>]) -> &BlockMetaData<B>
	{
		debug_assert!(self.is_not_null(), "this pointer is null");
		debug_assert!(self.0 < block_meta_data_items.len(), "block_meta_data_items is too short");
		
		unsafe { block_meta_data_items.get_unchecked(self.0) }
	}
}

#[derive(Debug)]
struct AtomicBlockPointer<B: Block>(AtomicU32);

impl<B: Block> AtomicBlockPointer<B>
{
	#[inline(always)]
	fn get(&self) -> BlockPointer<B>
	{
		BlockPointer(self.0.load(Acquire))
	}
	
	#[inline(always)]
	fn set(&self, new_block_pointer: BlockPointer<B>)
	{
		self.0.store(new_block_pointer.0, Release)
	}
	
	#[inline(always)]
	fn set_non_atomically(&self, new_block_pointer: BlockPointer<B>)
	{
		self.0.store(new_block_pointer.0, Relaxed)
	}
	
	#[inline(always)]
	fn compare_and_swap(&self, was_block_pointer: BlockPointer<B>, new_block_pointer: BlockPointer<B>) -> Result<(), BlockPointer<B>>
	{
		// On success, we are release'ing the pointer. However, Rust forces us to specify `AcqRel`, rather than `Release`.
		// On failure, we acquire the current value of the pointer..
		match self.0.compare_exchange(was_block_pointer, new_block_pointer, AcqRel, Acquire)
		{
			Ok(_new_block_pointer) => Ok(()),
			Err(updated_block_pointer) => Err(BlockPointer(updated_block_pointer)),
		}
	}
	
	#[inline(always)]
	fn compare_and_swap_ignoring_failure(&self, was_block_pointer: BlockPointer<B>, new_block_pointer: BlockPointer<B>)
	{
		// On success, we are release'ing the pointer. However, Rust forces us to specify `AcqRel`, rather than `Release`.
		// On failure, we don't care, so Relaxed.
		self.0.compare_exchange(was_block_pointer, new_block_pointer, AcqRel, Relaxed);
	}
}

/// Represents state of a block
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
enum BlockState
{
	NotInBag,
	Transitioning,
	InBag,
}

impl BlockState
{
	#[inline(always)]
	fn is_not_in_bag(&self) -> bool
	{
		self == BlockState::NotInBag
	}
	
	#[inline(always)]
	fn is_transitioning(self) -> bool
	{
		self == BlockState::Transitioning
	}
	
	#[inline(always)]
	fn is_in_bag(self) -> bool
	{
		self == BlockState::InBag
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BlockPointerAndState<B: Block>(u128);

impl<B: Block> BlockPointerAndState<B>
{
	#[inline(always)]
	const fn new(bag_pointer_with_time_stamp: BagPointerWithTimeStamp, block_pointer: BlockPointer<B>, block_state: BlockState) -> Self
	{
	}
	
	#[inline(always)]
	const fn null_with_bag_pointer_with_time_stamp_in_bag(bag_pointer_with_time_stamp: BagPointerWithTimeStamp) -> Self
	{
		Self::new(bag_pointer_with_time_stamp, BlockPointer::Null, BlockState::InBag)
	}
	
	#[inline(always)]
	const fn null_with_bag_pointer_with_time_stamp_not_in_bag(bag_pointer_with_time_stamp: BagPointerWithTimeStamp) -> Self
	{
		Self::new(bag_pointer_with_time_stamp, BlockPointer::Null, BlockState::NotInBag)
	}
	
	#[inline(always)]
	fn head_is_stale(self, our_bag_pointer_with_time_stamp: BagPointerWithTimeStamp) -> bool
	{
		if self.block_pointer().is_not_null()
		{
			return true;
		}
		
		if self.is_not_in_bag()
		{
			return true;
		}
		
		if our_bag_pointer_with_time_stamp.has_different_bag_pointer(self.bag_pointer())
		{
			return true;
		}
		
		our_bag_pointer_with_time_stamp.has_older_time_stamp(self.time_stamp())
	}
	
	#[inline(always)]
	fn is_not_in_bag(self) -> bool
	{
		self.block_state().is_not_in_bag()
	}
	
	// u32
	#[inline(always)]
	fn block_pointer(self) -> BlockPointer<B>
	{
	
	}
	
	// u80
	#[inline(always)]
	fn bag_pointer_with_time_stamp(self) -> BagPointerWithTimeStamp
	{
	
	}
	
	// A u16?
	#[inline(always)]
	fn bag_pointer(self) -> BagPointer
	{
	
	}
	
	// A u64?
	#[inline(always)]
	fn time_stamp(self) -> TimeStamp
	{
	
	}
	
	// u8 or smaller; can be pushed into pointer bits.
	#[inline(always)]
	fn block_state(self) -> BlockState
	{
	
	}
}

/// AtomicU128 does not exist.
#[derive(Debug)]
struct AtomicPreviousBlockPointer<B: Block>(AtomicU128);

impl<B: Block> AtomicPreviousBlockPointer<B>
{
	#[inline(always)]
	fn get(&self) -> BlockPointerAndState<B>
	{
		BlockPointerAndState(self.0.load(Acquire))
	}
	
	#[inline(always)]
	fn set(&self, new_previous_block_pointer: BlockPointerAndState<B>)
	{
		self.0.store(new_previous_block_pointer.0, Release)
	}
	
	#[inline(always)]
	fn compare_and_swap(&self, was_previous_block_pointer: BlockPointerAndState<B>, new_previous_block_pointer: BlockPointerAndState<B>) -> Result<(), BlockPointerAndState<B>>
	{
		// On success, we are release'ing the pointer. However, Rust forces us to specify `AcqRel`, rather than `Release`.
		// On failure, we acquire the current value of the pointer..
		match self.0.compare_exchange(was_previous_block_pointer, new_previous_block_pointer, AcqRel, Acquire)
		{
			Ok(_new_previous_block_pointer) => Ok(()),
			Err(updated_previous_block_pointer) => Err(BlockPointerAndState(updated_previous_block_pointer)),
		}
	}
}

/// A compressed pointer, representing an index.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BagPointer(u16);

/// An ABA protection that also prevents old updates; a form of epoch.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct TimeStamp(u64);

impl TimeStamp
{
	// An older time stamp is a smaller value.
	#[inline(always)]
	fn has_older_time_stamp(&self, other_time_stamp: TimeStamp) -> bool
	{
		self.0 < other_time_stamp
	}
	
	// A younger time stamp is a larger value.
	#[inline(always)]
	fn has_younger_time_stamp(&self, other_time_stamp: TimeStamp) -> bool
	{
		other_time_stamp < self.0
	}
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct BagPointerWithTimeStamp(BagPointer, TimeStamp);

impl BagPointerWithTimeStamp
{
	#[inline(always)]
	fn has_different_bag_pointer(&self, other_bag_pointer: BagPointer) -> bool
	{
		self.0 != other_bag_pointer
	}
	
	#[inline(always)]
	fn has_older_time_stamp(&self, other_time_stamp: TimeStamp) -> bool
	{
		self.1.has_older_time_stamp(other_time_stamp)
	}
	
	#[inline(always)]
	fn has_younger_time_stamp(&self, other_time_stamp: TimeStamp) -> bool
	{
		self.1.has_younger_time_stamp(other_time_stamp)
	}
}

#[derive(Debug)]
struct AtomicTimeStamp(AtomicU64);

impl AtomicTimeStamp
{
	#[inline(always)]
	fn new() -> Self
	{
		AtomicTimeStamp(AtomicU64::new(0))
	}
	
	#[inline(always)]
	fn current_and_increment(&self) -> TimeStamp
	{
		TimeStamp(self.0.fetch_add(1, Relaxed))
	}
}

#[derive(Debug)]
struct BlockMetaData<B: Block>
{
	pointer_to_next_node_in_bag: AtomicBlockPointer<B>,
	pointer_to_previous_node_in_bag: AtomicPreviousBlockPointer<B>,
}

impl<B: Block> BlockMetaData<B>
{
	#[inline(always)]
	fn set_next_non_atomically(&self, next_block_pointer: BlockPointer<B>)
	{
		self.pointer_to_next_node_in_bag.set_non_atomically(next_block_pointer)
	}
	
	#[inline(always)]
	fn try_to_set_previous_to_new_head_assuming_previous_was_null_and_in_bag(&self, previous_bag_pointer_with_time_stamp: BagPointerWithTimeStamp, bag_pointer_with_time_stamp: BagPointerWithTimeStamp, new_head_block_pointer: BlockPointer<B>) -> Result<(), BlockPointerAndState<B>>
	{
		self.pointer_to_previous_node_in_bag.compare_and_swap(BlockPointerAndState::null_with_bag_pointer_with_time_stamp_in_bag(previous_bag_pointer_with_time_stamp), BlockPointerAndState::new(bag_pointer_with_time_stamp, new_head_block_pointer, BlockState::InBag))
	}
	
	#[inline(always)]
	fn set_previous_to_null_and_in_bag(&self, bag_pointer_with_time_stamp: BagPointerWithTimeStamp)
	{
		self.pointer_to_previous_node_in_bag.set(BlockPointerAndState::null_with_bag_pointer_with_time_stamp_in_bag(bag_pointer_with_time_stamp))
	}
	
	#[inline(always)]
	fn set_previous_to_null_and_not_in_bag(&self, bag_pointer_with_time_stamp: BagPointerWithTimeStamp)
	{
		self.pointer_to_previous_node_in_bag.set(BlockPointerAndState::null_with_bag_pointer_with_time_stamp_not_in_bag(bag_pointer_with_time_stamp))
	}
	
	#[inline(always)]
	fn get_previous(&self) -> BlockPointerAndState<B>
	{
		self.pointer_to_previous_node_in_bag.get()
	}
}

#[derive(Debug)]
struct Bag<B: Block>
{
	time_stamp: AtomicTimeStamp,
	head_and_tail: AtomicTwinBlockPointerWithAbaProtection<B>,
}

impl<B: Block> Bag<B>
{
	#[inline(always)]
	fn remove_from_anywhere(&self, bags: Bags<B>, block_meta_data_items: &[BlockMetaData<B>], may_not_be_present: BlockPointer<B>) -> Option<BlockPointer<B>>
	{
		let potential_node = may_not_be_present.expand_to_pointer_to_meta_data(block_meta_data_items);
		
		// This value can have been overwritten by another thread, ie it's Junk.
		let previous_block_pointer: BlockPointerAndState<B> = potential_node.get_previous();
		
		if previous_block_pointer.is_not_in_bag()
		{
			return None;
		}
		
		if previous_block_pointer.bag_pointer() != bags.calculate_bag_pointer(self)
		{
			return None;
		}
		
		// So - it is in the bag.
		
		
		
		xxxxx
	}
	
	#[inline(always)]
	fn remove_from_tail(&self, bags: Bags<B>, block_meta_data_items: &[BlockMetaData<B>]) -> Option<BlockPointer<B>>
	{
		let mut old_head_and_tail = self.get_head_and_tail();
		loop
		{
			let old_tail = old_head_and_tail.tail_block_pointer();
			match old_tail.expand_to_pointer_to_meta_data(block_meta_data_items)
			{
				None =>
				{
					let old_head = old_head_and_tail.head_block_pointer();
					
					// The bag is genuinely empty
					if old_head.is_null()
					{
						return None
					}
					
					// IAT-2 has occurred but not IAT-3 yet.
					// A thread, or even several threads, have enqueued new head nodes but have yet to update their previous values.
					spin_loop_hint();
					old_head_and_tail = self.get_head_and_tail();
					continue
				}
				
				Some(old_tail_block_meta_data) =>
				{
					// This value can have been overwritten by another thread, ie it's Junk.
					// However, if that is the case, then that other thread should have changed either head or tail.
					let previous_block_pointer = old_tail_block_meta_data.get_previous();
					
					// Do not use a junk value from old_tail_block_meta_data if it is not in the bag.
					// FIXME: Not sure this is correct, or in the correct place. Might need to move down below `if previous_block_pointer.is_null()`.
					if previous_block_pointer.is_not_in_bag()
					{
						spin_loop_hint();
						old_head_and_tail = self.get_head_and_tail();
						continue
					}
					
					// This is possible because either (a) the bag is genuinely empty or (b) IAT-2 has occurred, but not IAT-3 yet.
					// See IAT-3 in `insert_at_head`.
					if previous_block_pointer.is_null()
					{
						let old_head = old_head_and_tail.head_block_pointer();
						
						// The bag is genuinely empty
						if old_head.is_null()
						{
						}
						// IAT-2 has occurred but not IAT-3 yet.
						// A thread, or even several threads, have enqueued new head nodes but have yet to update their previous values.
						else
						{
							spin_loop_hint();
							old_head_and_tail = self.get_head_and_tail();
							continue
						}
					}
					
					let new_head_and_tail = old_head_and_tail.new_head_and_tail_for_remove(previous_block_pointer);
					match self.compare_and_swap_head_and_tail(old_head_and_tail, new_head_and_tail)
					{
						Ok(()) =>
						{
							// We have removed this node.
							//
							old_tail_block_meta_data.set_previous_to_null_and_not_in_bag(self.bag_pointer_with_time_stamp(bags));
							
							return Some(old_tail)
						},
						Err(updated_head_and_tail) =>
						{
							spin_loop_hint();
							old_head_and_tail = updated_head_and_tail;
						}
					}
				}
			}
		}
	}
	
	#[inline(always)]
	fn insert_at_head(&self, bags: Bags<B>, block_meta_data_items: &[BlockMetaData<B>], new_head: BlockPointer<B>)
	{
		let new_head_block_meta_data = new_head.expand_to_pointer_to_meta_data_unchecked(block_meta_data_items);
		
		// TODO: We may want to spin-lock here.
		debug_assert!(new_head_block_meta_data.pointer_to_previous_node_in_bag.get().is_not_in_bag());
		
		
		let mut old_head_and_tail = self.get_head_and_tail();
		loop
		{
			let old_head = old_head_and_tail.head_block_pointer();
			
			let our_bag_pointer_with_time_stamp = self.bag_pointer_with_time_stamp(bags);
			
			match old_head.expand_to_pointer_to_meta_data(block_meta_data_items)
			{
				None => match self.insert_at_head_common(new_head_block_meta_data, our_bag_pointer_with_time_stamp, old_head, &old_head_and_tail, new_head)
				{
					Ok(()) => (),
					Err(updated_head_and_tail) =>
					{
						spin_loop_hint();
						old_head_and_tail = updated_head_and_tail;
						continue
					}
				},
				
				Some(old_head_block_meta_data) =>
				{
					let previous_block_pointer: BlockPointerAndState<B> = old_head_block_meta_data.get_previous();
					
					if previous_block_pointer.head_is_stale(our_bag_pointer_with_time_stamp)
					{
						spin_loop_hint();
						old_head_and_tail = self.get_head_and_tail();
						continue
					}
					
					// IAT-2
					match self.insert_at_head_common(new_head_block_meta_data, our_bag_pointer_with_time_stamp, old_head, &old_head_and_tail, new_head)
					{
						Ok(()) => (),
						Err(updated_head_and_tail) =>
						{
							spin_loop_hint();
							old_head_and_tail = updated_head_and_tail;
							continue
						}
					}
					
					// IAT-3
					// We need to update `old_head.pointer_to_previous_node_in_bag` so removing works.
					// No other thread can now get, via `self.head`, to `old_head`.
					//
					// Problem 1
					// However, a thread which has just changed `tail` in a remove can get to `old_head`.
					// And it will want to change `old_head.pointer_to_previous_node_in_bag` so that the removed node can be merged or used for storage.
					// We can compare-and-swap `old_head.pointer_to_previous_node_in_bag`.
					// We know that before IAT-2, at IAT-1, we made our `head`'s `pointer_to_previous_node_in_bag` null and bagged, and that is the entry state for all such 'about to become head' nodes.
					// If not null + bagged then a thread doing a remove has changed it.
					// [In which case self.tail will now be null].
					//
					// Problem 2
					// The tail can now be removed.
					// The tail removal logic will see that old_head has a null previous pointer.
					// It will therefore not update `tail` correctly - it will update `tail` with null.
					// The tail removal logic MUST update `tail` with the value of `head` if previous is null.
					let previous_bag_pointer_with_time_stamp = previous_block_pointer.bag_pointer_with_time_stamp();
					old_head_block_meta_data.try_to_set_previous_to_new_head_assuming_previous_was_null_and_in_bag(previous_bag_pointer_with_time_stamp, our_bag_pointer_with_time_stamp, new_head)
				}
			}
			
			break
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn bag_pointer_with_time_stamp(&self, bags: Bags<B>) -> BagPointerWithTimeStamp
	{
		let our_bag_pointer = bags.calculate_bag_pointer(self);
		let time_stamp = self.time_stamp.current_and_increment();
		BagPointerWithTimeStamp(x, time_stamp)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn insert_at_head_common(&self, new_head_block_meta_data: &BlockMetaData<B>, our_bag_pointer_with_time_stamp: BagPointerWithTimeStamp, old_head: BlockPointer<B>, old_head_and_tail: &TwinBlockPointerWithAbaProtection<B>, new_head: BlockPointer<B>) -> Result<(), TwinBlockPointerWithAbaProtection<B>>
	{
		// TODO: Doesn't need to be atomic (currently is).
		new_head_block_meta_data.set_previous_to_null_and_in_bag(our_bag_pointer_with_time_stamp);
		
		// Write can occur any time before the memory barrier forced by the compare_and_swap of `self.head` below at IAT-2.
		new_head_block_meta_data.set_next_non_atomically(old_head);
		
		let new_head_and_tail = old_head_and_tail.new_head_and_tail_for_insert(new_head);
		
		self.compare_and_swap_head_and_tail(old_head_and_tail, new_head_and_tail)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn get_head_and_tail(&self) -> TwinBlockPointerWithAbaProtection<B>
	{
		self.head_and_tail.get()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn compare_and_swap_head_and_tail(&self, old_head_and_tail: TwinBlockPointerWithAbaProtection<B>, new_head_and_tail: TwinBlockPointerWithAbaProtection<B>) -> Result<(), TwinBlockPointerWithAbaProtection<B>>
	{
		self.head_and_tail.compare_and_swap(old_head_and_tail, new_head_and_tail)
	}
}

#[derive(Debug)]
struct Bags<B: Block>;

impl<B: Block> Bags<B>
{
	#[inline(always)]
	fn calculate_bag_pointer(&self, bag: &Bag<B>) -> BagPointer
	{
		let our_base_pointer = self as *const _ as *const u8 as usize;
		let bag_pointer = bag as *const _ as *const u8 as usize;
		let index = (bag_pointer - our_base_pointer) / size_of::<Bag<B>>();
		BagPointer(index as u16)
	}
}
