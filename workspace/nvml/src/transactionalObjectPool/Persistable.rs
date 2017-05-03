// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait Persistable: Sized
{
	#[inline(always)]
	fn typeNumber() -> TypeNumber;
	
	#[inline(always)]
	fn size() -> size_t
	{
		size_of::<Self>() as size_t
	}
	
	#[inline(always)]
	fn oid(&self) -> PMEMoid
	{
		let pointer = self as *const _ as *const c_void;
		let oid = pointer.oid();
		debug_assert!(!oid.is_null(), "This is not a Persistable");
		oid
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateUninitializedInTransaction(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateUninitializedInTransaction::<Self>(Self::size(), Self::typeNumber())
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateUninitializedInTransactionWithoutFlush(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateUninitializedInTransactionWithoutFlush::<Self>(Self::size(), Self::typeNumber())
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateZeroedInTransaction(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateZeroedInTransaction::<Self>(Self::size(), Self::typeNumber())
	}
	
	/// Zero-sized allocations are not supported
	/// If returns Err(error) then the transaction will have been aborted; return immediately from work() function
	#[inline(always)]
	fn allocateZeroedInTransactionWithoutFlush(transaction: Transaction) -> Result<OidWrapper<Self>, c_int>
	{
		transaction.allocateZeroedInTransactionWithoutFlush::<Self>(Self::size(), Self::typeNumber())
	}
	
	#[inline(always)]
	fn free(self, transaction: Transaction) -> c_int
	{
		transaction.free(self.oid())
	}
	
	/// size can be zero
	#[inline(always)]
	fn addRangeSnapshotInTransaction(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= Self::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, Self::size());
		
		transaction.addRangeSnapshotInTransaction(self.oid(), offset, size)
	}

	/// Can only be called from a work() function
	/// If returns !=0 then the transaction will have been aborted; return immediately from work() function
	/// No checks are made for offset or size
	/// size can be zero
	#[inline(always)]
	fn addRangeSnapshotInTransactionWithoutFlush(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= Self::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, Self::size());
		
		transaction.addRangeSnapshotInTransactionWithoutFlush(self.oid(), offset, size)
	}
}

#[macro_export]
macro_rules! persistent_struct
{
	($typeNumber: expr, $name: ident, $($element: ident: $type: ty),+) =>
	{
		interpolate_idents!
		{
			//pub type [$name _toid_type_num] = [u8; $typeNumber + 1];
			
			#[derive(Copy, Clone)]
			#[repr(C)]
			pub struct $name
			{
				$(
					pub $element: $type,
				)*
			}
			
			impl Persistable for $name
			{
				#[inline(always)]
				fn typeNumber() -> TypeNumber
				{
					$typeNumber
				}
			}
		}
	}
}

persistent_struct!(0, root, node: u64);
persistent_struct!(1, node, next: OidWrapper<node>, foo: OidWrapper<foo>);
persistent_struct!(2, foo, _address: u8);


// Maximum layout name incl \0
// #define PMEMOBJ_MAX_LAYOUT ((size_t)1024)




/*

BEGIN
typedef uint8_t _pobj_layout_mylayout_ref[0 + 1];
pub type _pobj_layout_mylayout_ref = [u8; 1usize];



ROOT
typedef uint8_t root_toid_type_num[(0) + 1];
pub type root_toid_type_num = [u8; 1usize];

union root_toid
{
	PMEMoid oid;
	struct root * _type;
	root_toid_type_num * _type_num;
}
#[repr(C)]
#[derive(Copy)]
pub union root_toid
{
    pub oid: PMEMoid,
    pub _type: *mut root,
    pub _type_num: *mut root_toid_type_num,
}


TOID
typedef uint8_t node_toid_type_num[((1 + 1 - (sizeof(_pobj_layout_mylayout_ref)))) + 1];
pub type node_toid_type_num = [u8; 2usize];

union node_toid
{
	PMEMoid oid;
	struct node *_type;
	node_toid_type_num *_type_num;
};
#[repr(C)]
#[derive(Copy)]
pub union node_toid
{
    pub oid: PMEMoid,
    pub _type: *mut node,
    pub _type_num: *mut node_toid_type_num,
}


TOID
typedef uint8_t foo_toid_type_num[((2 + 1 - (sizeof(_pobj_layout_mylayout_ref)))) + 1];
pub type foo_toid_type_num = [u8; 3usize];

union foo_toid
{
	PMEMoid oid;
	struct foo *_type;
	foo_toid_type_num *_type_num;
};

#[repr(C)]
#[derive(Copy)]
pub union foo_toid
{
    pub oid: PMEMoid,
    pub _type: *mut foo,
    pub _type_num: *mut foo_toid_type_num,
}

END
typedef char _pobj_layout_mylayout_cnt[3 + 1 - (sizeof(_pobj_layout_mylayout_ref))];
pub type _pobj_layout_mylayout_cnt = [::std::os::raw::c_char; 3usize];




struct root
{
	union node_toid node;
}
#[repr(C)]
#[derive(Copy)]
pub struct root
{
    pub node: node_toid,
}

struct node
{
	union node_toid next;
	union foo_toid foo;
}
#[repr(C)]
#[derive(Copy)]
pub struct node
{
    pub next: node_toid,
    pub foo: foo_toid,
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo {
    pub _address: u8,
}

const char *layout_name = "mylayout";
pub const layout_name: &'static [u8; 9usize] = b"mylayout\x00";

int num_of_types = (sizeof(_pobj_layout_mylayout_cnt) - 1);
pub const num_of_types: ::std::os::raw::c_int = 2;















impl Clone for root {
    fn clone(&self) -> Self { *self }
}



impl Clone for node {
    fn clone(&self) -> Self { *self }
}




impl Clone for foo {
    fn clone(&self) -> Self { *self }
}

*/
