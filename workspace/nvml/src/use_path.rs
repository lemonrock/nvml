// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! use_path
{
	($self: ident, $function: path$(,$argument: expr)*) =>
	{
		{
			let os_path = $self.as_os_str();
			let bytes = os_path.as_bytes();
			let pointer = bytes.as_ptr() as *const c_char;

			unsafe { $function(pointer$(,$argument)*) }
		}
	}
}
