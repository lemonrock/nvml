// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.



// This works as long as block_size >= MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock && MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock >= 256 && is_a_power_of_two(MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock)
// Since pointers will have a minimum alignment of MinimumBlockSizeInBytesToCreateSpaceForAOneByteLock, the lowest 8 bits at least will be unoccupied.
// Union works on Intel x86-64.
// This data can be stored either within the chain itself (eg at the beginning or end of the block)
// Or it can be stored in a fixed size area as 'block / chain meta data', separate.
// We find it in the fixed size area by taking its pointer, subtracting the first pointer of the block allocator and dividing by block size to get block number (index) one based.
#[repr(C)]
#[derive(Copy, Clone)]
union PointerToNextChainOrLock<B: Block>
{
	next_chain_pointer_might_be_null: AtomicPtr<Chain<B>>,
	sentinel_byte: AtomicU8,
}

impl<B: Block> Default for PointerToNextChainOrLock<B>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::Empty
	}
}

impl<B: Block> Debug for PointerToNextChainOrLock<B>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.next_chain_pointer_might_be_null, f)
	}
}

impl<B: Block> Pointer for PointerToNextChainOrLock<B>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(self.next_chain_pointer_might_be_null, f)
	}
}

impl<B: Block> PointerToNextChainOrLock<B>
{
	// Conveniently, this does not interfere with any possible valid value of self.next_chain_pointer_or_null (including a null pointer of 0x00000000).
	const MemoryIsInUseSentinel: u8 = 0x00;
	
	const MemoryIsAvailableSentinel: u8 = 0x01;
	
	const NullPointer: *mut Chain<B> = null_mut();
	
	const NullPointerWithMemoryIsAvailable: *mut Chain<B> = Self::NullPointer + Self::MemoryIsAvailableSentinel;
	
	const NoNextChainPointer: AtomicPtr<Chain<B>> = AtomicPtr::new(Self::NullPointer);
	
	const Empty: Self = Self
	{
		next_chain_pointer_might_be_null: Self::NoNextChainPointer
	};
	
	#[inline(always)]
	const fn new(next_chain_pointer_might_be_null: *mut Chain<B>) -> Self
	{
		Self
		{
			next_chain_pointer_might_be_null: AtomicPtr::new(next_chain_pointer_might_be_null),
		}
	}
	
	#[inline(always)]
	unsafe fn memory_for_this_chain_is_in_use(&self) -> bool
	{
		self.sentinel_byte.load(Relaxed) == Self::MemoryIsInUseSentinel
	}
	
	#[inline(always)]
	unsafe fn memory_for_this_chain_is_available(&self) -> bool
	{
		self.sentinel_byte.load(Relaxed) == Self::MemoryIsAvailableSentinel
	}
	
	// After doing this, we need to remove it from the free_list of BlockAllocator
	// We then need to attach additional chains as required, or 'split_off' chains as appropriate.
	#[inline(always)]
	unsafe fn try_to_take(&self) -> bool
	{
		const success_ordering: Ordering = Acquire;
		const failure_ordering: Ordering = Release; // Must be less than success_ordering
		
		match self.sentinel_byte.compare_exchange(Self::MemoryIsAvailableSentinel, Self::MemoryIsInUseSentinel, success_ordering, release_ordering)
		{
			Ok(_current) => true,
			Err(_was) => false,
		}
	}
	
	// As part of doing this, we need to insert into the free_list of BlockAllocator
	#[inline(always)]
	unsafe fn make_available(&self)
	{
		debug_assert!(self.memory_for_this_chain_is_in_use(), "This memory is not in use");
		
		self.next_chain_pointer_might_be_null.store(Acquire, Self::NullPointerWithMemoryIsAvailable);
	}
	
	#[inline(always)]
	unsafe fn next_chain_pointer_might_be_null(&self) -> *mut Chain<B>
	{
		debug_assert!(self.memory_for_this_chain_is_in_use(), "This memory is not in use");
		
		self.next_chain_pointer_might_be_null.load(Relaxed)
	}
}
