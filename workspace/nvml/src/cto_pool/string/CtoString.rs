// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// A UTF-8 encoded, growable string. See Rust stdlib String for more details.
#[derive(PartialOrd, Eq, Ord)]
pub struct CtoString
{
	vec: CtoVec<u8>,
}

impl CtoString
{
	/// Creates a new empty `CtoString`.
	#[inline(always)]
	pub fn new(cto_pool_alloc: CtoPoolAlloc) -> CtoString
	{
		CtoString { vec: CtoVec::new(cto_pool_alloc) }
	}
	
	/// Creates a new empty `CtoString` with a particular capacity.
	#[inline(always)]
	pub fn with_capacity(capacity: usize, cto_pool_alloc: CtoPoolAlloc) -> CtoString
	{
		CtoString { vec: CtoVec::with_capacity(capacity, cto_pool_alloc) }
	}
	
	/// Creates a new `String` from a length, capacity, and pointer.
	#[inline(always)]
	pub unsafe fn from_raw_parts(buf: *mut u8, length: usize, capacity: usize, cto_pool_alloc: CtoPoolAlloc) -> CtoString
	{
		CtoString { vec: CtoVec::from_raw_parts(buf, length, capacity, cto_pool_alloc) }
	}
	
	/// Converts a vector of bytes to a `String` without checking that the string contains valid UTF-8.
	#[inline(always)]
	pub unsafe fn from_utf8_unchecked(bytes: CtoVec<u8>) -> CtoString
	{
		CtoString { vec: bytes }
	}
	
	/// Converts a `String` into a byte vector.
	///
	/// This consumes the `String`, so we do not need to copy its contents.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let s = String::from("hello");
	/// let bytes = s.into_bytes();
	///
	/// assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
	/// ```
	#[inline(always)]
	pub fn into_bytes(self) -> CtoVec<u8>
	{
		self.vec
	}
	
	/// Extracts a string slice containing the entire string.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let s = String::from("foo");
	///
	/// assert_eq!("foo", s.as_str());
	/// ```
	#[inline(always)]
	pub fn as_str(&self) -> &str
	{
		self
	}
	
	/// Converts a `String` into a mutable string slice.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let mut s = String::from("foobar");
	/// let s_mut_str = s.as_mut_str();
	///
	/// s_mut_str.make_ascii_uppercase();
	///
	/// assert_eq!("FOOBAR", s_mut_str);
	/// ```
	#[inline(always)]
	pub fn as_mut_str(&mut self) -> &mut str
	{
		self
	}
	
	/// Appends a given string slice onto the end of this `String`.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let mut s = String::from("foo");
	///
	/// s.push_str("bar");
	///
	/// assert_eq!("foobar", s);
	/// ```
	#[inline(always)]
	pub fn push_str(&mut self, string: &str)
	{
		self.vec.extend_from_slice(string.as_bytes())
	}
	
	/// Returns this `String`'s capacity, in bytes.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let s = String::with_capacity(10);
	///
	/// assert!(s.capacity() >= 10);
	/// ```
	#[inline(always)]
	pub fn capacity(&self) -> usize
	{
		self.vec.capacity()
	}
	
	/// Ensures that this `String`'s capacity is at least `additional` bytes larger than its length.
	#[inline(always)]
	pub fn reserve(&mut self, additional: usize)
	{
		self.vec.reserve(additional)
	}
	
	/// Ensures that this `String`'s capacity is `additional` bytes larger than its length.
	#[inline(always)]
	pub fn reserve_exact(&mut self, additional: usize)
	{
		self.vec.reserve_exact(additional)
	}
	
	/// Shrinks the capacity of this `String` to match its length.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let mut s = String::from("foo");
	///
	/// s.reserve(100);
	/// assert!(s.capacity() >= 100);
	///
	/// s.shrink_to_fit();
	/// assert_eq!(3, s.capacity());
	/// ```
	#[inline(always)]
	pub fn shrink_to_fit(&mut self)
	{
		self.vec.shrink_to_fit()
	}
	
	/// Appends the given [`char`] to the end of this `String`.
	///
	/// [`char`]: ../../std/primitive.char.html
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let mut s = String::from("abc");
	///
	/// s.push('1');
	/// s.push('2');
	/// s.push('3');
	///
	/// assert_eq!("abc123", s);
	/// ```
	#[inline(always)]
	pub fn push(&mut self, ch: char)
	{
		match ch.len_utf8()
		{
			1 => self.vec.push(ch as u8),
			_ => self.vec.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
		}
	}
	
	/// Returns a byte slice of this `String`'s contents.
	///
	/// The inverse of this method is [`from_utf8`].
	///
	/// [`from_utf8`]: #method.from_utf8
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let s = String::from("hello");
	///
	/// assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
	/// ```
	#[inline(always)]
	pub fn as_bytes(&self) -> &[u8]
	{
		&self.vec
	}
	
	/// Shortens this `String` to the specified length.
	///
	/// If `new_len` is greater than the string's current length, this has no
	/// effect.
	///
	/// Note that this method has no effect on the allocated capacity
	/// of the string.
	#[inline(always)]
	pub fn truncate(&mut self, new_len: usize)
	{
		if new_len <= self.len()
		{
			assert!(self.is_char_boundary(new_len));
			self.vec.truncate(new_len)
		}
	}
	
	/// Removes the last character from the string buffer and returns it.
	#[inline(always)]
	pub fn pop(&mut self) -> Option<char>
	{
		let ch = match self.chars().rev().next()
		{
			Some(ch) => ch,
			None => return None,
		};
		
		let new_length = self.len() - ch.len_utf8();
		unsafe
		{
			self.vec.set_len(new_length);
		}
		Some(ch)
	}
	
	/// Removes a [`char`] from this `String` at a byte position and returns it.
	///
	/// This is an `O(n)` operation as it requires copying every element in the buffer.
	#[inline(always)]
	pub fn remove(&mut self, idx: usize) -> char
	{
		let ch = match self[idx..].chars().next()
		{
			Some(ch) => ch,
			None => panic!("cannot remove a char from the end of a string"),
		};
		
		let next = idx + ch.len_utf8();
		let len = self.len();
		unsafe
		{
			copy(self.vec.as_ptr().offset(next as isize), self.vec.as_mut_ptr().offset(idx as isize), len - next);
			self.vec.set_len(len - (next - idx));
		}
		ch
	}
	
	/// Retains only the characters specified by the predicate.
	///
	/// In other words, remove all characters `c` such that `f(c)` returns `false`.
	/// This method operates in place and preserves the order of the retained characters.
	#[inline(always)]
	pub fn retain<F>(&mut self, mut f: F)
		where F: FnMut(char) -> bool
	{
		let len = self.len();
		let mut del_bytes = 0;
		let mut idx = 0;
		
		while idx < len
		{
			let ch = unsafe
			{
				self.slice_unchecked(idx, len).chars().next().unwrap()
			};
			let ch_len = ch.len_utf8();
			
			if !f(ch)
			{
				del_bytes += ch_len;
			}
			else if del_bytes > 0
			{
				unsafe
				{
					copy(self.vec.as_ptr().offset(idx as isize), self.vec.as_mut_ptr().offset((idx - del_bytes) as isize), ch_len);
				}
			}
			
			// Point idx to the next char
			idx += ch_len;
		}
		
		if del_bytes > 0
		{
			unsafe { self.vec.set_len(len - del_bytes); }
		}
	}
	
	/// Inserts a character into this `String` at a byte position.
	///
	/// This is an `O(n)` operation as it requires copying every element in the buffer.
	#[inline(always)]
	pub fn insert(&mut self, idx: usize, ch: char)
	{
		assert!(self.is_char_boundary(idx));
		
		let mut bits = [0; 4];
		let bits = ch.encode_utf8(&mut bits).as_bytes();
		
		unsafe
		{
			self.insert_bytes(idx, bits);
		}
	}
	
	#[inline(always)]
	unsafe fn insert_bytes(&mut self, idx: usize, bytes: &[u8])
	{
		let len = self.len();
		let amt = bytes.len();
		self.vec.reserve(amt);
		
		copy(self.vec.as_ptr().offset(idx as isize), self.vec.as_mut_ptr().offset((idx + amt) as isize), len - idx);
		copy(bytes.as_ptr(), self.vec.as_mut_ptr().offset(idx as isize), amt);
		self.vec.set_len(len + amt);
	}
	
	/// Inserts a string slice into this `String` at a byte position.
	///
	/// This is an `O(n)` operation as it requires copying every element in the buffer.
	#[inline(always)]
	pub fn insert_str(&mut self, idx: usize, string: &str)
	{
		assert!(self.is_char_boundary(idx));
		
		unsafe
		{
			self.insert_bytes(idx, string.as_bytes());
		}
	}
	
	/// Returns a mutable reference to the contents of this `String`.
	///
	/// # Safety
	///
	/// This function is unsafe because it does not check that the bytes passed
	/// to it are valid UTF-8. If this constraint is violated, it may cause
	/// memory unsafety issues with future users of the `String`, as the rest of
	/// the standard library assumes that `String`s are valid UTF-8.
	#[inline(always)]
	pub unsafe fn as_mut_vec(&mut self) -> &mut CtoVec<u8>
	{
		&mut self.vec
	}
	
	/// Returns the length of this `String`, in bytes.
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.vec.len()
	}
	
	/// Returns `true` if this `String` has a length of zero.
	///
	/// Returns `false` otherwise.
	pub fn is_empty(&self) -> bool
	{
		self.len() == 0
	}
	
	/// Splits the string into two at the given index.
	///
	/// Returns a newly allocated `String`. `self` contains bytes `[0, at)`, and
	/// the returned `String` contains bytes `[at, len)`. `at` must be on the
	/// boundary of a UTF-8 code point.
	///
	/// Note that the capacity of `self` does not change.
	///
	/// # Panics
	///
	/// Panics if `at` is not on a `UTF-8` code point boundary, or if it is beyond the last
	/// code point of the string.
	///
	/// # Examples
	///
	/// ```
	/// # fn main()
	/// {
	/// let mut hello = String::from("Hello, World!");
	/// let world = hello.split_off(7);
	/// assert_eq!(hello, "Hello, ");
	/// assert_eq!(world, "World!");
	/// # }
	/// ```
	#[inline(always)]
	pub fn split_off(&mut self, at: usize) -> CtoString
	{
		assert!(self.is_char_boundary(at));
		let other = self.vec.split_off(at);
		unsafe { CtoString::from_utf8_unchecked(other) }
	}
	
	/// Truncates this `String`, removing all contents.
	///
	/// While this means the `String` will have a length of zero, it does not
	/// touch its capacity.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// let mut s = String::from("foo");
	///
	/// s.clear();
	///
	/// assert!(s.is_empty());
	/// assert_eq!(0, s.len());
	/// assert_eq!(3, s.capacity());
	/// ```
	#[inline(always)]
	pub fn clear(&mut self)
	{
		self.vec.clear()
	}
	
	/// Creates a draining iterator that removes the specified range in the string
	/// and yields the removed chars.
	///
	/// Note: The element range is removed even if the iterator is not
	/// consumed until the end.
	pub fn drain<R>(&mut self, range: R) -> CtoStringDrain
		where R: RangeArgument<usize>
	{
		// Memory safety
		//
		// The String version of Drain does not have the memory safety issues
		// of the vector version. The data is just plain bytes.
		// Because the range removal happens in Drop, if the Drain iterator is leaked,
		// the removal will not happen.
		let len = self.len();
		let start = match range.start()
		{
			Included(&n) => n,
			Excluded(&n) => n + 1,
			Unbounded => 0,
		};
		let end = match range.end()
		{
			Included(&n) => n + 1,
			Excluded(&n) => n,
			Unbounded => len,
		};
		
		// Take out two simultaneous borrows. The &mut String won't be accessed
		// until iteration is over, in Drop.
		let self_ptr = self as *mut _;
		
		// slicing does the appropriate bounds checks
		let chars_iter = self[start..end].chars();
		
		CtoStringDrain
		{
			start,
			end,
			iter: chars_iter,
			string: self_ptr,
		}
	}
	
	/// Creates a splicing iterator that removes the specified range in the string, and replaces it with the given string.
	/// The given string doesn't need to be the same length as the range.
	pub fn splice<R>(&mut self, range: R, replace_with: &str)
		where R: RangeArgument<usize>
	{
		// Memory safety
		//
		// The String version of Splice does not have the memory safety issues
		// of the vector version. The data is just plain bytes.
		
		match range.start()
		{
			Included(&n) => assert!(self.is_char_boundary(n)),
			Excluded(&n) => assert!(self.is_char_boundary(n + 1)),
			Unbounded => {},
		};
		match range.end()
		{
			Included(&n) => assert!(self.is_char_boundary(n + 1)),
			Excluded(&n) => assert!(self.is_char_boundary(n)),
			Unbounded => {},
		};
		
		unsafe
		{
			self.as_mut_vec()
		}.splice(range, replace_with.bytes());
	}
}

impl Extend<char> for CtoString
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I)
	{
		let iterator = iter.into_iter();
		let (lower_bound, _) = iterator.size_hint();
		self.reserve(lower_bound);
		for ch in iterator
		{
			self.push(ch)
		}
	}
}

impl<'a> Extend<&'a char> for CtoString
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item = &'a char>>(&mut self, iter: I)
	{
		self.extend(iter.into_iter().cloned());
	}
}

impl<'a> Extend<&'a str> for CtoString
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I)
	{
		for s in iter
		{
			self.push_str(s)
		}
	}
}

impl Extend<CtoString> for CtoString
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item =CtoString>>(&mut self, iter: I)
	{
		for s in iter
		{
			self.push_str(&s)
		}
	}
}

impl<'a> Extend<Cow<'a, str>> for CtoString
{
	#[inline(always)]
	fn extend<I: IntoIterator<Item = Cow<'a, str>>>(&mut self, iter: I)
	{
		for s in iter
		{
			self.push_str(&s)
		}
	}
}

/// A convenience impl that delegates to the impl for `&str`
impl<'a, 'b> Pattern<'a> for &'b CtoString
{
	type Searcher = <&'b str as Pattern<'a>>::Searcher;
	
	#[inline(always)]
	fn into_searcher(self, haystack: &'a str) -> <&'b str as Pattern<'a>>::Searcher
	{
		self[..].into_searcher(haystack)
	}
	
	#[inline(always)]
	fn is_contained_in(self, haystack: &'a str) -> bool
	{
		self[..].is_contained_in(haystack)
	}
	
	#[inline(always)]
	fn is_prefix_of(self, haystack: &'a str) -> bool
	{
		self[..].is_prefix_of(haystack)
	}
}

impl PartialEq for CtoString
{
	#[inline(always)]
	fn eq(&self, other: &CtoString) -> bool
	{
		PartialEq::eq(&self[..], &other[..])
	}
	
	#[inline(always)]
	fn ne(&self, other: &CtoString) -> bool
	{
		PartialEq::ne(&self[..], &other[..])
	}
}

macro_rules! impl_eq
{
    ($lhs:ty, $rhs: ty) =>
    {
        impl<'a, 'b> PartialEq<$rhs> for $lhs
        {
			#[inline(always)]
            fn eq(&self, other: &$rhs) -> bool { PartialEq::eq(&self[..], &other[..]) }
            
			#[inline(always)]
            fn ne(&self, other: &$rhs) -> bool { PartialEq::ne(&self[..], &other[..]) }
        }
    }
}

impl_eq! { CtoString, str }
impl_eq! { CtoString, &'a str }
impl_eq! { CtoString, Cow<'a, str> }

impl Display for CtoString
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		fmt::Display::fmt(&**self, f)
	}
}

impl Debug for CtoString
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		fmt::Debug::fmt(&**self, f)
	}
}

impl Hash for CtoString
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		(**self).hash(hasher)
	}
}

impl<'a> Add<&'a str> for CtoString
{
	type Output = CtoString;
	
	#[inline(always)]
	fn add(mut self, other: &str) -> CtoString
	{
		self.push_str(other);
		self
	}
}

impl<'a> AddAssign<&'a str> for CtoString
{
	#[inline(always)]
	fn add_assign(&mut self, other: &str)
	{
		self.push_str(other);
	}
}

impl Index<Range<usize>> for CtoString
{
	type Output = str;
	
	#[inline(always)]
	fn index(&self, index: Range<usize>) -> &str
	{
		&self[..][index]
	}
}

impl Index<RangeTo<usize>> for CtoString
{
	type Output = str;
	
	#[inline(always)]
	fn index(&self, index: RangeTo<usize>) -> &str
	{
		&self[..][index]
	}
}

impl Index<RangeFrom<usize>> for CtoString
{
	type Output = str;
	
	#[inline(always)]
	fn index(&self, index: RangeFrom<usize>) -> &str
	{
		&self[..][index]
	}
}

impl Index<RangeFull> for CtoString
{
	type Output = str;
	
	#[inline(always)]
	fn index(&self, _index: RangeFull) -> &str
	{
		unsafe { from_utf8_unchecked(&self.vec) }
	}
}

impl Index<RangeInclusive<usize>> for CtoString
{
	type Output = str;
	
	#[inline(always)]
	fn index(&self, index: RangeInclusive<usize>) -> &str
	{
		Index::index(&**self, index)
	}
}

impl Index<RangeToInclusive<usize>> for CtoString
{
	type Output = str;
	
	#[inline(always)]
	fn index(&self, index: RangeToInclusive<usize>) -> &str
	{
		Index::index(&**self, index)
	}
}

impl IndexMut<Range<usize>> for CtoString
{
	#[inline(always)]
	fn index_mut(&mut self, index: Range<usize>) -> &mut str
	{
		&mut self[..][index]
	}
}

impl IndexMut<RangeTo<usize>> for CtoString
{
	#[inline(always)]
	fn index_mut(&mut self, index: RangeTo<usize>) -> &mut str
	{
		&mut self[..][index]
	}
}

impl IndexMut<RangeFrom<usize>> for CtoString
{
	#[inline(always)]
	fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut str
	{
		&mut self[..][index]
	}
}

impl IndexMut<RangeFull> for CtoString
{
	#[inline(always)]
	fn index_mut(&mut self, _index: RangeFull) -> &mut str
	{
		unsafe { from_utf8_unchecked_mut(&mut *self.vec) }
	}
}

impl IndexMut<RangeInclusive<usize>> for CtoString
{
	#[inline(always)]
	fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut str
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl IndexMut<RangeToInclusive<usize>> for CtoString
{
	#[inline(always)]
	fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut str
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl Deref for CtoString
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &str
	{
		unsafe { from_utf8_unchecked(&self.vec) }
	}
}

impl DerefMut for CtoString
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut str
	{
		unsafe { from_utf8_unchecked_mut(&mut *self.vec) }
	}
}

impl AsRef<str> for CtoString
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self
	}
}

impl AsRef<[u8]> for CtoString
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		self.as_bytes()
	}
}

impl From<CtoString> for CtoVec<u8>
{
	#[inline(always)]
	fn from(string: CtoString) -> CtoVec<u8>
	{
		string.into_bytes()
	}
}

impl fmt::Write for CtoString
{
	#[inline(always)]
	fn write_str(&mut self, s: &str) -> fmt::Result
	{
		self.push_str(s);
		Ok(())
	}
	
	#[inline(always)]
	fn write_char(&mut self, c: char) -> fmt::Result
	{
		self.push(c);
		Ok(())
	}
}
