// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


// Option: https://github.com/nahratzah/ll
	// <https://nahratzah.wordpress.com/2012/10/11/lock-free-programming-and-formal-proofs/>
	// Code is close to impenetrable but could be converted... just.

// Option : Hardware Transactional Memory
	// Definitely do-able
	// A software implementation is maybe possible via `picotm`, but this uses `setjmp` / `longjmp` and a thread-lock
		// Indeed, most hybrid transactional memory solutions do use a thread-lock
		// We don't need to if we know which memory values we're touching
	// libitm for a composite?

// Investigate: http://drops.dagstuhl.de/opus/volltexte/2016/6678/pdf/LIPIcs-OPODIS-2015-35.pdf
	// Non-Blocking Doubly-Linked Lists with Good Amortized Complexity
	// ? https://github.com/mynameisfiber/jackslinks ?
	// ? https://github.com/Kronuz/Xapiand/commit/e26274777cab99971250fd0ffd736996bff51f6c ?

// Not-So-Keen Option: Adapt Java's variation of MSQueue (ConcurrentLinkedQueue) so can delete OR adapt Java's ConcurrentDeque
	// Relies on Java GC, so might be hard to tease out.

// Lesser Option: Michael Scott (https://www.cs.rochester.edu/~scott/#personal) has a C++ deque. Need to find how to get the source.

// PicoTM Blog: http://transactionblog.org/2017/12/15/porting-picotm-to-mac-os-windows-and-freebsd/
	- useful ideas





// NOTE: A working deque using a lock: https://github.com/cksystemsgroup/scalloc/blob/master/src/deque.h

// NOTE: A skiplist (priority queue) from Facebook in C++: https://github.com/facebook/folly/blob/master/folly/ConcurrentSkipList.h

// Non-Option: ccqueue - uses a dummy node
// Non-Option : Dmitry Vjukov'q MPSC: https://groups.google.com/forum/#!topic/lock-free/Vd9xuHrLggE
// Non-Option: Moody Camel C++ queue: https://github.com/cameron314/concurrentqueue
	// http://moodycamel.com/blog/2014/a-fast-general-purpose-lock-free-queue-for-c++
	// http://moodycamel.com/blog/2014/detailed-design-of-a-lock-free-queue
// Probably Non-Option `wfqueue` (bounded array) (https://github.com/chaoran/fast-wait-free-queue)
// Probably Non-Option `lcrq` (bounded array) (https://github.com/chaoran/fast-wait-free-queue)
// Non-Option MSQueue in https://www.liblfds.org/ (uses a dummy node so won't work for us)
// Non-Option: Anything ring based


//Other

// WOW: Lock free persistent queues: http://concurrencyfreaks.blogspot.fr/2018/01/optimization-to-michael-scott.html

// WOW: https://github.com/ezrosent/wf_hash_writeup - A wait-free hashtable for Rust

// WOW: Intel TSX hashtable: 4 (5 incl an inferior original) different hopscotch hash implementations: https://www.cs.tau.ac.il/~multi/2015a/chhm.htm
	// IN C++ and Java
	// There is a hardware transactional variant with fallback
	// Code is written by students...

// WOW: https://dl.acm.org/citation.cfm?id=3079519 - a wait-free hashtable in C++. Now where is the code for it?..

// TSX: Poor quality implementation, single linked list C++, HTM / libitm single linked list: https://github.com/nmldiegues/proteustm/blob/master/benchmarks/datastructures-gcc/linkedlist/linkedlist.cpp (walks the list)

// TSX: Poor quality implementation, skip list, but Intel TSX based skiplist / leaplist: https://www.cs.tau.ac.il/~multi/2015a/Leaplist.htm

// TSX: Poor quality implementation, hash map, https://github.com/nmldiegues/proteustm/blob/master/benchmarks/datastructures-gcc/hashmap/hashmap.cpp

// Contains pointers to algos that could work with Intel TSX but not adapated as such: https://software.intel.com/en-us/blogs/2013/06/23/tsx-fallback-paths

// ProteusTM (HTM / STM composite): https://github.com/nmldiegues/proteustm/

// Synquake TSM:   https://github.com/mikedw/synquake_tsx ('HythTM')


// Modern C++ implementation of a hashmap: https://shlomisteinberg.com/2015/09/28/designing-a-lock-free-wait-free-hash-map/
// C Implementation of cliff click's hashmap and a skiplist: https://code.google.com/archive/p/nbds/ or  https://github.com/argv0/nbds
// Concurrency kit (but not multi-writer) http://concurrencykit.org/
// Vjukok's hash map, 1024 cores, behaviours not known re MPMC
// Cambridge stuff (impractical): http://www.cl.cam.ac.uk/research/srg/netos/lock-free/

// Stacks
	// A modern, DWCAS aware lock-free stack: http://nullprogram.com/blog/2014/09/02/
	// https://dl.acm.org/citation.cfm?id=2676963
		//
// Branchless UTF-8 decoding: http://nullprogram.com/blog/2017/10/06/ and, potentially similar: http://bjoern.hoehrmann.de/utf-8/decoder/dfa/
// An efficient mutex that doesn't use pthreads and is simpler (in some ways) to parking_lot: https://docs.rs/mcs/0.1.1/mcs/struct.Mutex.html
	// Needs a `Slot`, which is a thread-local handle.
	// Uses a spinlock with a queue of waiting threads

Using HTM

Notes from Wang, libTM:-
"to  ensure  progress,  we  implement  a
 spinlock-based  software  fallback  path  for  every  RTM  trans-
 action started. TSX will speculatively attempt to elide the spin
 locks but should any transactions abort, critical sections will be
 protected by acquiring the spin locks in software. Our imple-
 mentation  supports  both  multiple  fine-grained  spin  locks  and
 one global spin lock as the software fallback path."

LibTM seems to be here: https://github.com/mikedw/synquake_tsx/tree/master/src/lib_tm  and untouched since 2014... despite the HythTM paper.
	Code of interest seems to be here: https://github.com/mikedw/synquake_tsx/blob/d9217fb85408216c8f0c39ec85ae462109fc9940/src/lib_tm/tm_scope.cc



