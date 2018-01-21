// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Identical in concept to a regular Rust Box but exists in a persistent object pool.
pub struct CtoBox<Value: CtoSafe>
{
	persistent_memory_pointer: NonNull<CtoBoxInner<Value>>,
}

impl<Value: CtoSafe> PersistentMemoryWrapper for CtoBox<Value>
{
	type PersistentMemory = CtoBoxInner<Value>;
	
	type Value = Value;
	
	#[inline(always)]
	unsafe fn initialize_persistent_memory<InitializationError, Initializer: FnOnce(*mut Self::Value, &CtoPoolArc) -> Result<(), InitializationError>>(persistent_memory_pointer: *mut Self::PersistentMemory, cto_pool_arc: &CtoPoolArc, initializer: Initializer) -> Result<Self, InitializationError>
	{
		let mut persistent_memory_pointer = NonNull::new_unchecked(persistent_memory_pointer);
		
		{
			persistent_memory_pointer.as_mut().allocated(cto_pool_arc, initializer)?;
		}
		
		Ok
		(
			Self
			{
				persistent_memory_pointer,
			}
		)
	}
}

impl<Value: CtoSafe> CtoSafe for CtoBox<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		self.persistent_memory_mut().cto_pool_opened(cto_pool_arc)
	}
}

impl<Value: CtoSafe> Drop for CtoBox<Value>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pool_pointer = self.persistent_memory().cto_pool_arc.pool_pointer();
		
		let persistent_memory_pointer = self.persistent_memory_pointer.as_ptr();
		
		unsafe { drop_in_place(persistent_memory_pointer) }
		
		pool_pointer.free(persistent_memory_pointer);
	}
}

impl<Value: CtoSafe + PartialEq> PartialEq for CtoBox<Value>
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

impl<Value: CtoSafe + Eq> Eq for CtoBox<Value>
{
}

impl<Value: CtoSafe + PartialOrd> PartialOrd for CtoBox<Value>
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

impl<Value: CtoSafe + Ord> Ord for CtoBox<Value>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

impl<Value: CtoSafe + Hash> Hash for CtoBox<Value>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state);
	}
}

impl<Value: CtoSafe + Hasher> Hasher for CtoBox<Value>
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

impl<Value: CtoSafe + Display> Display for CtoBox<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(self.deref(), f)
	}
}

impl<Value: CtoSafe + Debug> Debug for CtoBox<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.deref(), f)
	}
}

impl<Value: CtoSafe> Pointer for CtoBox<Value>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&self.deref(), f)
	}
}

impl<Value: CtoSafe> Deref for CtoBox<Value>
{
	type Target = Value;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.persistent_memory().deref()
	}
}

impl<Value: CtoSafe> DerefMut for CtoBox<Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.persistent_memory_mut().deref_mut()
	}
}

impl<Value: CtoSafe> Borrow<Value> for CtoBox<Value>
{
	#[inline(always)]
	fn borrow(&self) -> &Value
	{
		self.deref()
	}
}

impl<Value: CtoSafe> BorrowMut<Value> for CtoBox<Value>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut Value
	{
		self.deref_mut()
	}
}

impl<Value: CtoSafe> AsRef<Value> for CtoBox<Value>
{
	#[inline(always)]
	fn as_ref(&self) -> &Value
	{
		self.deref()
	}
}

impl<Value: CtoSafe> AsMut<Value> for CtoBox<Value>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Value
	{
		self.deref_mut()
	}
}

impl<Value: CtoSafe> CtoBox<Value>
{
	/// A pointer to use with C. Use wisely; dropping this object may cause the pointer to go out of scope.
	#[inline(always)]
	pub fn as_ptr(this: &Self) -> *const Value
	{
		this.deref() as *const Value
	}
	
	/// A pointer to use with C. Use wisely; dropping this object may cause the pointer to go out of scope.
	#[inline(always)]
	pub fn as_mut_ptr(this: &mut Self) -> *mut Value
	{
		this.deref_mut() as *mut Value
	}
	
	/// Gets a raw pointer to Value, suitable for use with FFI.
	/// Must be eventually passed to `from_raw()`, or a very serious (possibly irrecoverable even with reboots) memory leak will occur.
	#[inline(always)]
	pub fn into_raw(mut this: Self) -> *mut Value
	{
		this.persistent_memory_mut().into_raw_value_pointer()
	}
	
	/// Gets a CtoBox from a raw pointer to Value, typically passed back from FFI.
	/// Must be a pointer originally created using `into_raw()`.
	#[inline(always)]
	pub unsafe fn from_raw(raw_value_pointer: *mut Value) -> Self
	{
		Self
		{
			persistent_memory_pointer: NonNull::new_unchecked(CtoBoxInner::from_raw_value_pointer(raw_value_pointer)),
		}
	}
	
	#[inline(always)]
	fn persistent_memory(&self) -> &CtoBoxInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_ref() }
	}
	
	#[inline(always)]
	fn persistent_memory_mut(&mut self) -> &mut CtoBoxInner<Value>
	{
		unsafe { self.persistent_memory_pointer.as_mut() }
	}
}
