// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// TODO: Possibly a better implementation: https://github.com/Qarterd/Honeycomb/blob/master/src/common/Honey/Thread/LockFree/Backoff.h
/// In fully concurrent systems the helping strategy, as well as heavy contention on atomic primitives, can downgrade the performance significantly.
/// Therefore the algorithm, after a number of consecutive failed CAS operations (ie failed attempts to help concurrent operations) puts the current operation into back-off mode.
/// When in back-off mode, the thread does nothing for a while, and in this way avoids disturbing the concurrent operations that might otherwise progress slower.
/// The duration of the back-off is initialized to some value (eg proportional to the number of threads) at the start of an operation, and for each consecutive entering of the back-off mode during one operation invocation, the duration of the back-off is changed using some scheme, eg increased exponentially.
#[allow(non_snake_case)]
#[inline(always)]
fn Back_Off()
{
	// Issues a pause, yielding the thread to allow a spinlock to be fair on hyper-threaded architectures, so that we don't hog the cpu if we are spinning.
	hint_core_should_pause();
}
