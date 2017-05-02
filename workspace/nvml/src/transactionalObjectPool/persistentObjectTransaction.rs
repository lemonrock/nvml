// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Please note that work() may not ever be called - in which case, the next logic called is onAbort()
pub fn persistentObjectTransaction<Committed: Sized, Aborted: Sized, W: Fn() -> c_int, C: Fn() -> Committed, A: Fn() -> Aborted>(pop: *mut PMEMobjpool, work: W, onCommit: C, onAbort: A) -> Result<Committed, Aborted>
{
	// Must be used as a function, to prevent the volatile restrictions of setjmp leaking out
	#[inline(never)]
	fn internal
	<
		Committed: Sized,
		Aborted: Sized,
		W: Fn() -> c_int,
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
		let mut txSetJmpEnvironment: jmp_buf = unsafe { zeroed() };
		{
			let txSetJmpEnvironmentPointer = txSetJmpEnvironment.as_mut_ptr();
			// setjmp returns a non-zero value if returning from longjmp()
			if setjmp(txSetJmpEnvironmentPointer) == 0
			{
				setErrorNumberIfNecessary(pmemobj_tx_begin(pop, txSetJmpEnvironmentPointer, TX_PARAM_NONE, TX_PARAM_NONE));
			}
			else
			{
				setErrorNumberIfNecessary(pmemobj_tx_errno());
			}
	
			let mut stage: pobj_tx_stage;
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
						
						match catch_unwind(AssertUnwindSafe(|| work()))
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
	
	internal(pop, work, onCommit, onAbort, &mut panicPayload, &mut functionResult);
	
	if let Some(payload) = panicPayload
	{
		resume_unwind(payload);
	}
	
	functionResult.unwrap()
}

#[inline(always)]
fn setErrorNumberIfNecessary(osErrorNumber: c_int)
{
	if unlikely(osErrorNumber != 0)
	{
		set_errno(Errno(osErrorNumber));
	}
}
