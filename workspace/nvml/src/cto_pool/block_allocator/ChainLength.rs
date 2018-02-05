// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Chain length
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct ChainLength(u10);

impl ChainLength
{
	#[inline(always)]
	fn from_length(length: usize) -> Self
	{
		debug_assert!(length != 0, "length can not be zero");
		debug_assert!(length <= InclusiveMaximumChainLength, "length is too large");
		Self::from_index(length - 1)
	}
	
	#[inline(always)]
	fn from_index(index: usize) -> Self
	{
		debug_assert!(index < InclusiveMaximumChainLength, "index is too large");
		ChainLength(index as u10)
	}
	
	#[inline(always)]
	fn get_bag<'bag, B: Block>(&self, bags: &'bag [Bag<B>; InclusiveMaximumChainLength]) -> &'bag Bag<B>
	{
		unsafe { bags.get_unchecked(self.as_index()) }
	}
	
	#[inline(always)]
	fn as_index(self) -> usize
	{
		self.0 as usize
	}
	
	#[inline(always)]
	pub(crate) fn as_length(self) -> usize
	{
		self.0 as usize + 1
	}
	
	#[inline(always)]
	pub(crate) fn as_capacity<B: Block>(self) -> usize
	{
		self.as_length() * B::BlockSizeInBytes
	}
	
	#[inline(always)]
	pub(crate) fn is_less_than_inclusive_maximum(self) -> bool
	{
		self.as_length() < InclusiveMaximumChainLength
	}
	
	#[inline(always)]
	pub(crate) fn add_if_maximum_length_not_exceeded(self, other: ChainLength) -> Option<Self>
	{
		let combined_length = self.as_length() + other.as_length();
		if combined_length <= InclusiveMaximumChainLength
		{
			Some(Self::from_length(combined_length))
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub(crate) fn subtract(self, shorter_chain_length: Self) -> Self
	{
		debug_assert!(&shorter_chain_length < &self, "shorter_chain_length is not less than self");
		ChainLength::from_length(self.as_length() - shorter_chain_length.as_length())
	}
	
	#[inline(always)]
	pub(crate) fn bytes<B: Block>(self) -> usize
	{
		self.as_length() * B::BlockSizeInBytes
	}
}
