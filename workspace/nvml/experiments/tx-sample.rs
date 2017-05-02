extern crate errno;
extern crate libc;
extern crate nvml_sys;
extern crate rust_extra;


use ::errno::Errno;
use ::errno::set_errno;
use ::libc::c_int;
use ::nvml_sys:*;
use ::rust_extra::unlikely;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::panic::resume_unwind;


// #[inline(always)]
// fn setErrorNumber(osErrorNumber: c_int)
// {
// 	set_errno(Errno(osErrorNumber))
// }

/// Please note that work() may not ever be called - in which case, the next logic called is onAbort()
pub fn persistentObjectTransaction<Committed: Sized, Aborted: Sized, W: Fn(), C: Fn() -> Committed, A: Fn() -> Aborted>(pop: *mut PMEMobjpool, work: W, onCommit: C, onAbort: A) -> Result<Committed, Aborted>
{
	// Must be used as a function, to prevent the volatile restrictions of setjmp leaking out
	#[inline(never)]
	fn internal
	<
		Committed: Sized,
		Aborted: Sized,
		W: Fn(),
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
	) -> Result<Committed, Aborted>
	{
		const panicOsErrorNumber: c_int = 2;
		let mut txSetJmpEnvironment: jmp_buf;
		
		// != 0 if returning from longjmp()
		if setjmp(txSetJmpEnvironment) != 0
		{
			//setErrorNumber(pmemobj_tx_errno());
		}
		else
		{
			pmemobj_tx_begin(pop, txSetJmpEnvironment, TX_PARAM_NONE, TX_PARAM_NONE);
			// let osErrorNumber = pmemobj_tx_begin(pop, txSetJmpEnvironment, TX_PARAM_NONE, TX_PARAM_NONE);
			// if unlikely(osErrorNumber != 0)
			// {
			// 	setErrorNumber(osErrorNumber);
			// }
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
					match catch_unwind(AssertUnwindSafe(|| work())
					{
						Ok(someOsErrorNumberForAbort) =>
						{
							if likely(someOsErrorNumberForAbort == 0)
							{
								pmemobj_tx_commit();
							}
							else
							{
								pmemobj_tx_abort(someOsErrorNumberForAbort);
							}
						},
						Err(payload) =>
						{
							pmemobj_tx_abort(panicOsErrorNumber);
							*panicPayload = Some(payload);
						},
					};
				
					pmemobj_tx_process();
				},
			
				pobj_tx_stage::TX_STAGE_ONCOMMIT =>
				{
					match catch_unwind(AssertUnwindSafe(|| onCommit())
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
					match catch_unwind(AssertUnwindSafe(|| onAbort())
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
		// let ifAbortedTheTransactionErrorNumber = pmemobj_tx_end();
		// if unlikely(ifAbortedTheTransactionErrorNumber != 0)
		// {
		// 	setErrorNumber(ifAbortedTheTransactionErrorNumber);
		// }
	}

	let mut panicPayload = None;
	let mut functionResult = None;
	
	internal(work, onCommit, onAbort, &mut panicPayload, &mut functionResult);
	
	if let Some(payload) = panicPayload
	{
		resume_unwind(payload);
	}
	
	functionResult.unwrap()
}
