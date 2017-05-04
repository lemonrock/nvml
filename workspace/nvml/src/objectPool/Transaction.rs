// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transaction;

impl Transaction
{
	/// Please note that work() may not ever be called - in which case, the next logic called is onAbort()
	#[inline(always)]
	pub fn transaction<Committed: Sized, Aborted: Sized, W: Fn(Transaction) -> c_int, C: Fn() -> Committed, A: Fn() -> Aborted>(pop: *mut PMEMobjpool, work: W, onCommit: C, onAbort: A) -> Result<Committed, Aborted>
	{
		// Must be used as a function, to prevent the volatile restrictions of setjmp leaking out
		#[inline(never)]
		unsafe fn internal
		<
			Committed: Sized,
			Aborted: Sized,
			W: Fn(Transaction) -> c_int,
			C: Fn() -> Committed,
			A: Fn() -> Aborted
		>
		(
			pop: *mut PMEMobjpool,
			work: W,
			onCommit: C,
			onAbort: A,
			panicPayload: &mut Option<Box<Any + Send + 'static>>,
			functionResult: &mut Option<Result<Committed, Aborted>>
		)
		{
			let txSetJmpEnvironment = zeroed();
			{
				// setjmp returns a non-zero value if returning from longjmp()
				if setjmp(txSetJmpEnvironment) == 0
				{
					setErrorNumberIfNecessary(pmemobj_tx_begin(pop, txSetJmpEnvironment, TX_PARAM_NONE, TX_PARAM_NONE));
				}
				else
				{
					setErrorNumberIfNecessary(pmemobj_tx_errno());
				}
				
				let mut stage;
				while
				{
					stage = pmemobj_tx_stage();
					stage != pobj_tx_stage::TX_STAGE_NONE
				}
				{
					match stage
					{
						pobj_tx_stage::TX_STAGE_WORK =>
						{
							let PanicOsErrorNumber: c_int = E::ENOTSUP;
							
							match catch_unwind(AssertUnwindSafe(|| work(Transaction)))
							{
								Ok(someOsErrorNumberForAbort) =>
								{
									if likely(someOsErrorNumberForAbort == 0)
									{
										pmemobj_tx_commit();
									}
									else
									{
										pmemobj_tx_abort(PanicOsErrorNumber);
									}
								},
								Err(payload) =>
								{
									pmemobj_tx_abort(PanicOsErrorNumber);
									*panicPayload = Some(payload);
								},
							};
							
							pmemobj_tx_process();
						},
						
						pobj_tx_stage::TX_STAGE_ONCOMMIT =>
						{
							match catch_unwind(AssertUnwindSafe(|| onCommit()))
							{
								Ok(result) =>
								{
									*functionResult = Some(Ok(result))
								},
								
								Err(payload) =>
								{
									if panicPayload.is_none()
									{
										*panicPayload = Some(payload)
									}
								}
							};
							
							pmemobj_tx_process();
						},
						
						pobj_tx_stage::TX_STAGE_ONABORT =>
						{
							match catch_unwind(AssertUnwindSafe(|| onAbort()))
							{
								Ok(result) =>
								{
									*functionResult = Some(Err(result))
								},
								
								Err(payload) =>
								{
									if panicPayload.is_none()
									{
										*panicPayload = Some(payload)
									}
								}
							};
							
							pmemobj_tx_process();
						},
						
						pobj_tx_stage::TX_STAGE_FINALLY =>
						{
							pmemobj_tx_process();
						},
						
						_ =>
						{
							pmemobj_tx_process();
						},
					}
				}
				
				pmemobj_tx_end();
				setErrorNumberIfNecessary(pmemobj_tx_end());
			}
		}
		
		let mut panicPayload = None;
		let mut functionResult = None;
		
		unsafe { internal(pop, work, onCommit, onAbort, &mut panicPayload, &mut functionResult) };
		
		if let Some(payload) = panicPayload
		{
			resume_unwind(payload);
		}
		
		functionResult.unwrap()
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	pub fn allocateUninitializedInTransaction<T: Persistable>(self, size: size_t, typeNumber: TypeNumber) -> Result<PersistentObject<T>, c_int>
	{
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		let result = unsafe { pmemobj_tx_alloc(size, typeNumber) };
		if unlikely(result.is_null())
		{
			Err(errno().0)
		}
		else
		{
			Ok(PersistentObject::new(result))
		}
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	pub fn allocateUninitializedInTransactionWithoutFlush<T: Persistable>(self, size: size_t, typeNumber: TypeNumber) -> Result<PersistentObject<T>, c_int>
	{
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		let result = unsafe { pmemobj_tx_xalloc(size, typeNumber, POBJ_XALLOC_NO_FLUSH) };
		if unlikely(result.is_null())
		{
			Err(errno().0)
		}
		else
		{
			Ok(PersistentObject::new(result))
		}
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	pub fn allocateZeroedInTransaction<T: Persistable>(self, size: size_t, typeNumber: TypeNumber) -> Result<PersistentObject<T>, c_int>
	{
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		let result = unsafe { pmemobj_tx_zalloc(size, typeNumber) };
		if unlikely(result.is_null())
		{
			Err(errno().0)
		}
		else
		{
			Ok(PersistentObject::new(result))
		}
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	pub fn allocateZeroedInTransactionWithoutFlush<T: Persistable>(self, size: size_t, typeNumber: TypeNumber) -> Result<PersistentObject<T>, c_int>
	{
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		const Flags: u64 = POBJ_XALLOC_ZERO | POBJ_XALLOC_NO_FLUSH;
		
		let result = unsafe { pmemobj_tx_xalloc(size, typeNumber, Flags) };
		if unlikely(result.is_null())
		{
			Err(errno().0)
		}
		else
		{
			Ok(PersistentObject::new(result))
		}
	}
	
	// If returns a non-zero error code, then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	pub fn free(self, object: PMEMoid) -> c_int
	{
		unsafe { pmemobj_tx_free(object) }
	}
	
	/// size can be zero
	/// If returns !=0 then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn addRangeSnapshotInTransaction(self, oid: PMEMoid, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(!oid.is_null(), "oid is null");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		if unlikely(size == 0)
		{
			return 0;
		}
		
		unsafe { pmemobj_tx_add_range(oid, offset, size) }
	}
	
	/// size can be zero
	/// If returns !=0 then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn addRangeSnapshotInTransactionWithoutFlush(self, oid: PMEMoid, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(!oid.is_null(), "oid is null");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		if unlikely(size == 0)
		{
			return 0;
		}
		
		unsafe { pmemobj_tx_xadd_range(oid, offset, size, POBJ_XADD_NO_FLUSH) }
	}
}

#[inline(always)]
fn setErrorNumberIfNecessary(osErrorNumber: c_int)
{
	if unlikely(osErrorNumber != 0)
	{
		set_errno(Errno(osErrorNumber));
	}
}
