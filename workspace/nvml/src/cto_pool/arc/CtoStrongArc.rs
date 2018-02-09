// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Provides a strong atomically referenced counted ('Arc') wrapper around a thread-safe inner data structure.
/// When the last instance of `CtoStringArc` is dropped, the inner data structure is dropped.
pub struct CtoStrongArc<I: CtoStrongArcInner>(NonNull<I>);

unsafe impl<I: CtoStrongArcInner> Send for CtoStrongArc<I>
{
}

unsafe impl<I: CtoStrongArcInner> Sync for CtoStrongArc<I>
{
}

impl<I: CtoStrongArcInner> CtoSafe for CtoStrongArc<I>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.deref_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<I: CtoStrongArcInner> Drop for CtoStrongArc<I>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.deref().release_reference()
		{
			unsafe { drop_in_place(self.deref_mut()) }
		}
	}
}

impl<I: CtoStrongArcInner> Clone for CtoStrongArc<I>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		self.deref().acquire_reference();
		CtoStrongArc(self.0)
	}
}

impl<I: CtoStrongArcInner> Deref for CtoStrongArc<I>
{
	type Target = I;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { self.0.as_ref() }
	}
}

impl<I: CtoStrongArcInner> DerefMut for CtoStrongArc<I>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { self.0.as_mut() }
	}
}

impl<I: CtoStrongArcInner + PartialEq> PartialEq for CtoStrongArc<I>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		*(*self) == *(*other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		*(*self) != *(*other)
	}
}

impl<I: CtoStrongArcInner + PartialOrd> PartialOrd for CtoStrongArc<I>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		(**self).partial_cmp(&**other)
	}
	
	#[inline(always)]
	fn lt(&self, other: &Self) -> bool
	{
		*(*self) < *(*other)
	}
	
	#[inline(always)]
	fn le(&self, other: &Self) -> bool
	{
		*(*self) <= *(*other)
	}
	
	#[inline(always)]
	fn gt(&self, other: &Self) -> bool
	{
		*(*self) > *(*other)
	}
	
	#[inline(always)]
	fn ge(&self, other: &Self) -> bool
	{
		*(*self) >= *(*other)
	}
}

impl<I: CtoStrongArcInner + Ord> Ord for CtoStrongArc<I>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		(**self).cmp(&**other)
	}
}

impl<I: CtoStrongArcInner + Eq> Eq for CtoStrongArc<I>
{
}

impl<I: CtoStrongArcInner + Hash> Hash for CtoStrongArc<I>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state)
	}
}

impl<I: CtoStrongArcInner + Hasher> Hasher for CtoStrongArc<I>
{
	#[inline(always)]
	fn finish(&self) -> u64
	{
		(**self).finish()
	}
	
	#[inline(always)]
	fn write(&mut self, bytes: &[u8])
	{
		(**self).write(bytes)
	}
	
	#[inline(always)]
	fn write_u8(&mut self, i: u8)
	{
		(**self).write_u8(i)
	}
	
	#[inline(always)]
	fn write_u16(&mut self, i: u16)
	{
		(**self).write_u16(i)
	}
	
	#[inline(always)]
	fn write_u32(&mut self, i: u32)
	{
		(**self).write_u32(i)
	}
	
	#[inline(always)]
	fn write_u64(&mut self, i: u64)
	{
		(**self).write_u64(i)
	}
	
//	#[inline(always)]
//	fn write_u128(&mut self, i: u128)
//	{
//		(**self).write_u128(i)
//	}
	
	#[inline(always)]
	fn write_usize(&mut self, i: usize)
	{
		(**self).write_usize(i)
	}
	
	#[inline(always)]
	fn write_i8(&mut self, i: i8)
	{
		(**self).write_i8(i)
	}
	
	#[inline(always)]
	fn write_i16(&mut self, i: i16)
	{
		(**self).write_i16(i)
	}
	
	#[inline(always)]
	fn write_i32(&mut self, i: i32)
	{
		(**self).write_i32(i)
	}
	
	#[inline(always)]
	fn write_i64(&mut self, i: i64)
	{
		(**self).write_i64(i)
	}
	
//	#[inline(always)]
//	fn write_i128(&mut self, i: i128)
//	{
//		(**self).write_i128(i)
//	}
	
	#[inline(always)]
	fn write_isize(&mut self, i: isize)
	{
		(**self).write_isize(i)
	}
}

impl<I: CtoStrongArcInner + Display> Display for CtoStrongArc<I>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(&**self, f)
	}
}

impl<I: CtoStrongArcInner + Debug> Debug for CtoStrongArc<I>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(&**self, f)
	}
}

impl<I: CtoStrongArcInner> Pointer for CtoStrongArc<I>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&(&**self as *const I), f)
	}
}

impl<I: CtoStrongArcInner> Borrow<I> for CtoStrongArc<I>
{
	#[inline(always)]
	fn borrow(&self) -> &I
	{
		&**self
	}
}

impl<I: CtoStrongArcInner> BorrowMut<I> for CtoStrongArc<I>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut I
	{
		self.deref_mut()
	}
}

impl<I: CtoStrongArcInner> AsRef<I> for CtoStrongArc<I>
{
	#[inline(always)]
	fn as_ref(&self) -> &I
	{
		&**self
	}
}

impl<I: CtoStrongArcInner> AsMut<I> for CtoStrongArc<I>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut I
	{
		self.deref_mut()
	}
}

impl<I: CtoStrongArcInner> CtoStrongArc<I>
{
	/// Creates a new instance around a reference counted inner.
	/// Use with caution.
	#[inline(always)]
	pub fn new(inner: NonNull<I>) -> Self
	{
		CtoStrongArc(inner)
	}
}
