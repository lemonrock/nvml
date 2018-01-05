// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// A Persistable that can be an entry in a doubly-linked list.
pub trait ListEntryPersistable: Persistable
{
	/// The offset in bytes of the field in the Persistable-implementing struct that holds the next() pointer
	const PersistentCircularDoublyLinkedListEntryFieldOffset: size_t = 0;
	
	/// Tell calling code which field in this persistable linked list is used to record next / previous entry.
	#[inline(always)]
	fn list_entry_field(&self) -> &PersistentCircularDoublyLinkedListEntry<Self>;
}
