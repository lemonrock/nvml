// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Wrapper type. Refer to `parking_lot::Mutex`.
/// Access the mutex by using `deref()` or `deref_mut()`.
pub struct CtoParkingLotMutexLock<Value: CtoSafe>(Mutex<Value>, CtoPoolArc);

impl<Value: CtoSafe> Deref for CtoParkingLotMutexLock<Value>
{
	type Target = Mutex<Value>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<Value: CtoSafe> DerefMut for CtoParkingLotMutexLock<Value>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl<Value: CtoSafe> CtoSafe for CtoParkingLotMutexLock<Value>
{
	#[inline(always)]
	fn cto_pool_opened(&mut self, cto_pool_arc: &CtoPoolArc)
	{
		{
			let mutate_mutex_private_fields = self.hack_to_mutate_mutex_private_fields();
			
			Self::initialize_raw_mutex(mutate_mutex_private_fields);
			
			unsafe { &mut *mutate_mutex_private_fields.data.get() }.cto_pool_opened(cto_pool_arc);
		}
		
		cto_pool_arc.write(&mut self.1);
	}
}

impl<Value: CtoSafe> CtoParkingLotMutexLock<Value>
{
	/// Create a new instance on the Stack (or inside a persistent memory object).
	#[inline(always)]
	pub fn new<InitializationError, Initializer: FnOnce(*mut Value, &CtoPoolArc) -> Result<(), InitializationError>>(initializer: Initializer, cto_pool_arc: &CtoPoolArc) -> Result<Self, InitializationError>
	{
		let mut this: Self = unsafe { uninitialized() };
		
		{
			cto_pool_arc.write(&mut this.1);
		}
		
		{
			Self::initialize_raw_mutex(this.hack_to_mutate_mutex_private_fields());
		}
		
		let result = initializer(this.hack_to_mutate_mutex_private_fields().data.get(), cto_pool_arc);
		
		// Note: Since an UnsafeCell is just a NewType wrapper (ie, has one field, called, `value`, of type `Value`), the pointer is always valid and UnsafeCell is validly initialized.
		// However, if a panic occurs and `drop()` is invoked, all bets are off.
		match result
		{
			Ok(_) => Ok(this),
			Err(error) =>
			{
				unsafe { drop_in_place(&mut this.1) };
				
				// No need to drop anything in Mutex
				
				forget(this);
				
				Err(error)
			}
		}
	}
	
	#[inline(always)]
	fn hack_to_mutate_mutex_private_fields(&mut self) -> &mut Mutex_HorribleHackToAccessPrivateFields<Value>
	{
		unsafe { &mut * (&mut self.0 as *mut Mutex<Value> as *mut Mutex_HorribleHackToAccessPrivateFields<Value>) }
	}
	
	#[inline(always)]
	fn initialize_raw_mutex(mutate_mutex_private_fields: &mut Mutex_HorribleHackToAccessPrivateFields<Value>)
	{
		unsafe { write(&mut mutate_mutex_private_fields.raw, RawMutex_HorribleHackToAccessPrivateFields::new()) };
	}
}

#[allow(non_camel_case_types)]
struct Mutex_HorribleHackToAccessPrivateFields<T: ?Sized>
{
	raw: RawMutex_HorribleHackToAccessPrivateFields,
	data: UnsafeCell<T>,
}

#[allow(non_camel_case_types)]
struct RawMutex_HorribleHackToAccessPrivateFields
{
	#[allow(dead_code)]
	state: AtomicU8,
}

impl RawMutex_HorribleHackToAccessPrivateFields
{
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			state: AtomicU8::new(0),
		}
	}
}
