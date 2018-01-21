// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.
use super::*;
use ::std::sync::atomic::AtomicPtr;
use ::std::sync::atomic::Ordering;
use ::std::sync::atomic::Ordering::Relaxed;
use ::std::sync::atomic::Ordering::Acquire;


// https://github.com/nahratzah/ll / <https://nahratzah.wordpress.com/2012/10/11/lock-free-programming-and-formal-proofs/>

// Option 2: Reimplement the Tsigas paper ourselves
	// Paper: http://www.cse.chalmers.se/~tsigas/papers/Lock-Free%20Doubly%20Linked%20lists%20and%20Deques%20-OPODIS04.pdf
	// Some slightly helpful advice here: https://stackoverflow.com/questions/19609417/atomic-operations-for-lock-free-doubly-linked-list
	// Seems that Tsigas relies on prev potentially being stale

// Option 3: Use the Harris solution for a singly-linked list: eg read about it here: https://en.wikipedia.org/wiki/Non-blocking_linked_list

// Option 4: See if anything is in https://link.springer.com/chapter/10.1007%2F978-3-642-41527-2_17

// Option 5: A d-lang implementation: https://code.dlang.org/packages/lock-free (Tsigas, Sundell)




// More publications:-

// https://dl.acm.org/citation.cfm?id=2144829 (sundell, tsigas, 2004, earlier paper)
// https://dl.acm.org/citation.cfm?id=1989550 sundell et al, concurrent bags lock-free



// ANother patent, based on a ring: http://www.freshpatents.com/Lock-free-double-ended-queue-based-on-a-dynamic-ring-dt20070705ptan20070157214.php
//* <https://www.codeproject.com/Articles/723555/A-Lock-Free-Doubly-Linked-List>
	// Faster for traversal
//* <https://www.codeproject.com/articles/825703/a-lock-free-doubly-linked-list-with-safe-memory-re>
//* Sundell et al, C++: <https://github.com/Qarterd/Honeycomb/blob/master/src/common/Honey/Thread/LockFree/List.h>
//* Sundell & Tsigas, C# implemtation: <https://github.com/c7hm4r/LockFreeDoublyLinkedList>
//* Paper:
//
//Descriptions:-
//* Sun / Java patent, with pretty much complete code: https://www.google.com/patents/US7533138
//Notes:-
//* http://www.rossbencina.com/code/lockfree?q=~rossb/code/lockfree/
//* Sundell et al Paper: http://citeseer.ist.psu.edu/cache/papers/cs/32904/http:zSzzSzwww.cs.chalmers.sezSz~phszSzTechnicalReportszSzSunT04_Deque.pdf/sundell04lockfree.pdf
//* is Sundell et al the same as:-
//	"Efficient and Reliable Lock-Free Memory Reclamation Based on Reference Counting" Anders Gidenstam,Member, IEEE,Marina Papatriantafilou, H˚ akan Sundell and Philippas Tsigas
//	"Lock-free deques and doubly linked lists" Håkan Sundell, Philippas Tsigas
//* http://15418.courses.cs.cmu.edu/spring2013/article/46
//
//Other
// Modern C++ implementation of a hashmap: https://shlomisteinberg.com/2015/09/28/designing-a-lock-free-wait-free-hash-map/
// C Implementation of cliff click's hashmap and a skiplist: https://code.google.com/archive/p/nbds/ or  https://github.com/argv0/nbds
// COmcurrency kit (but not multi-writer)
// Vjukok's hash map, 1024 cores, behaviours not known re MPMC
// http://concurrencykit.org/
// Cambridge stuff: http://www.cl.cam.ac.uk/research/srg/netos/lock-free/
//* Java implementations of various concurrent sets, binary search trees (sorted sets), etc: https://github.com/bapi/ConcurrentSet
//* Java implementation of concurrent k-d tree (https://en.wikipedia.org/wiki/K-d_tree) (https://github.com/bapi/ConcurrentKDTree_o/blob/master/src/se/chalmers/dcs/bapic/concurrentKDTree/KDTrees/SimpleSingleLockBasedKDTree.java)
// * Dmitry Vjukov'q MPSC: https://groups.google.com/forum/#!topic/lock-free/Vd9xuHrLggE
// A modern, DWCAS aware lock-free stack: http://nullprogram.com/blog/2014/09/02/
// Branchless UTD-8 decoding: http://nullprogram.com/blog/2017/10/06/ and, potentially similar: http://bjoern.hoehrmann.de/utf-8/decoder/dfa/
