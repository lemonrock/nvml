// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents a transaction.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transaction;

impl Transaction
{
	/// This logic currently does not work properly because Rust does not understand the implications of `setjmp` / `longjmp`.
	/// Please note that` work` may not ever be called - in which case, the next logic called is `on_abort`.
	#[inline(always)]
	pub fn transaction<Committed: Sized, Aborted: Sized, W: Fn(Transaction) -> c_int, C: Fn() -> Committed, A: Fn() -> Aborted>(pop: *mut PMEMobjpool, work: W, on_commit: C, on_abort: A) -> Result<Committed, Aborted>
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
			on_commit: C,
			on_abort: A,
			panic_payload: &mut Option<Box<Any + Send + 'static>>,
			function_result: &mut Option<Result<Committed, Aborted>>
		)
		{
			let tx_set_jmp_environment = zeroed();
			{
				// setjmp returns a non-zero value if returning from longjmp()
				if setjmp(tx_set_jmp_environment) == 0
				{
					set_error_number_if_necessary(pmemobj_tx_begin(pop, tx_set_jmp_environment, pobj_tx_param_TX_PARAM_NONE, pobj_tx_param_TX_PARAM_NONE));
				}
				else
				{
					set_error_number_if_necessary(pmemobj_tx_errno());
				}
				
				let mut stage;
				while
				{
					stage = pmemobj_tx_stage();
					stage != pobj_tx_stage_TX_STAGE_NONE
				}
				{
					match stage
					{
						pobj_tx_stage_TX_STAGE_WORK =>
						{
							let panic_os_error_nummber: c_int = E::ENOTSUP;
							
							match catch_unwind(AssertUnwindSafe(|| work(Transaction)))
							{
								Ok(some_os_error_number_for_transaction_abort) =>
								{
									if likely(some_os_error_number_for_transaction_abort == 0)
									{
										pmemobj_tx_commit();
									}
									else
									{
										pmemobj_tx_abort(panic_os_error_nummber);
									}
								},
								Err(payload) =>
								{
									pmemobj_tx_abort(panic_os_error_nummber);
									*panic_payload = Some(payload);
								},
							};
							
							pmemobj_tx_process();
						},
						
						pobj_tx_stage_TX_STAGE_ONCOMMIT =>
						{
							match catch_unwind(AssertUnwindSafe(|| on_commit()))
							{
								Ok(result) =>
								{
									*function_result = Some(Ok(result))
								},
								
								Err(payload) =>
								{
									if panic_payload.is_none()
									{
										*panic_payload = Some(payload)
									}
								}
							};
							
							pmemobj_tx_process();
						},
						
						pobj_tx_stage_TX_STAGE_ONABORT =>
						{
							match catch_unwind(AssertUnwindSafe(|| on_abort()))
							{
								Ok(result) =>
								{
									*function_result = Some(Err(result))
								},
								
								Err(payload) =>
								{
									if panic_payload.is_none()
									{
										*panic_payload = Some(payload)
									}
								}
							};
							
							pmemobj_tx_process();
						},
						
						pobj_tx_stage_TX_STAGE_FINALLY =>
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
				set_error_number_if_necessary(pmemobj_tx_end());
			}
		}
		
		let mut panic_payload = None;
		let mut function_result = None;
		
		unsafe { internal(pop, work, on_commit, on_abort, &mut panic_payload, &mut function_result) };
		
		if let Some(payload) = panic_payload
		{
			resume_unwind(payload);
		}
		
		function_result.unwrap()
	}
}

#[inline(always)]
fn set_error_number_if_necessary(os_error_number: c_int)
{
	if unlikely(os_error_number != 0)
	{
		set_errno(Errno(os_error_number));
	}
}
