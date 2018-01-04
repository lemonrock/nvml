// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Identical concept to a regular Rust Box but exists in a persistent object pool.
pub struct CtoBox<T: CtoSafe>(*mut T, Arc<CtoPoolInner>);

impl<T: CtoSafe> CtoSafe for CtoBox<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.deref_mut().reinitialize(cto_pool_inner);
		self.1 = cto_pool_inner.clone();
	}
}

impl<T: CtoSafe> Drop for CtoBox<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		CtoPoolInner::free(&self.1, self.0)
	}
}

impl<T: CtoSafe + Clone> Clone for CtoBox<T>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		let pointer = (self.1).0.malloc::<T>().unwrap();
		unsafe { copy_nonoverlapping(self.0, pointer, 1) };
		CtoBox(pointer, self.1.clone())
	}
}

impl<T: CtoSafe + PartialEq> PartialEq for CtoBox<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		PartialEq::eq(&**self, &**other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		PartialEq::ne(&**self, &**other)
	}
}

impl<T: CtoSafe + Eq> Eq for CtoBox<T>
{
}

impl<T: CtoSafe + PartialOrd> PartialOrd for CtoBox<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&**self, &**other)
	}
	
	#[inline(always)]
	fn lt(&self, other: &Self) -> bool
	{
		PartialOrd::lt(&**self, &**other)
	}
	
	#[inline(always)]
	fn le(&self, other: &Self) -> bool
	{
		PartialOrd::le(&**self, &**other)
	}
	
	#[inline(always)]
	fn ge(&self, other: &Self) -> bool
	{
		PartialOrd::ge(&**self, &**other)
	}
	
	#[inline(always)]
	fn gt(&self, other: &Self) -> bool
	{
		PartialOrd::gt(&**self, &**other)
	}
}

impl<T: CtoSafe + Ord> Ord for CtoBox<T>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

impl<T: CtoSafe + Hash> Hash for CtoBox<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state);
	}
}

impl<T: CtoSafe + Hasher> Hasher for CtoBox<T>
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

impl<T: CtoSafe + Display> Display for CtoBox<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe + Debug> Debug for CtoBox<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe> Pointer for CtoBox<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&self.deref(), f)
	}
}

impl<T: CtoSafe> Deref for CtoBox<T>
{
	type Target = T;
	
	fn deref(&self) -> &T
	{
		unsafe { &*(self.0 as *const T) }
	}
}

impl<T: CtoSafe> DerefMut for CtoBox<T>
{
	fn deref_mut(&mut self) -> &mut T
	{
		unsafe { &mut *self.0 }
	}
}

impl<T: CtoSafe> Borrow<T> for CtoBox<T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe> BorrowMut<T> for CtoBox<T>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<T: CtoSafe> AsRef<T> for CtoBox<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe> AsMut<T> for CtoBox<T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<T: CtoSafe> CtoBox<T>
{
	/// Converts to a raw value to pass to C without dropping.
	#[inline(always)]
	pub fn into_raw(b: Self) -> *mut T
	{
		let inner = b.0;
		forget(b);
		inner
	}
}
