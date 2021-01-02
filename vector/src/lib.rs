pub mod iterator;
use iterator::{BorrowedVectorIterator, VectorIterator};
pub mod test_box;
pub mod test_i32;

use std::{alloc, alloc::Layout, fmt, fmt::Debug, ops::Index, ptr, ptr::NonNull};

const GROWTH_RATE: f64 = 1.25;

pub struct Vector<T> {
	pub(crate) data: Option<NonNull<T>>,
	pub(crate) size: usize,
	pub(crate) capacity: usize,
}

impl<T> Default for Vector<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: Debug> Debug for Vector<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_empty() {
			return write!(f, "[]");
		}
		write!(f, "[")?;
		for i in 0..(self.size - 1) {
			write!(f, "{:?}, ", self[i])?;
		}
		write!(
			f,
			"{:?}]",
			self.get(self.size - 1).expect("length already checked?")
		)
	}
}

impl<T> Index<usize> for Vector<T> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		self.get(index).expect("Index was out of bounds")
	}
}

impl<T> IntoIterator for Vector<T> {
	type Item = T;

	type IntoIter = VectorIterator<T>;

	fn into_iter(self) -> Self::IntoIter {
		VectorIterator {
			data: self.data,
			capacity: self.capacity,
			index: -1isize as usize,
			index_back: self.size,
		}
	}
}

impl<'a, T> IntoIterator for &'a Vector<T> {
	type Item = &'a T;

	type IntoIter = BorrowedVectorIterator<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		BorrowedVectorIterator {
			vector: &self,
			index: -1isize as usize,
			index_back: self.size,
		}
	}
}

impl<T> Drop for Vector<T> {
	fn drop(&mut self) {}
}

impl<T> Vector<T> {
	pub fn new() -> Self {
		Vector {
			data: None,
			size: 0,
			capacity: 0,
		}
	}

	pub fn with_capacity(cap: usize) -> Self {
		let mut vec = Vector::new();
		vec.reserve(cap);
		vec
	}

	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	pub fn len(&self) -> usize {
		self.size
	}

	fn reserve(&mut self, new_cap: usize) {
		let layout = Layout::array::<T>(new_cap).expect("Overflow");
		let new_ptr = unsafe { alloc::alloc(layout) as *mut T };
		assert!(!new_ptr.is_null());
		let new_data = NonNull::new(new_ptr);
		if let Some(old_ptr) = self.data {
			unsafe {
				ptr::copy_nonoverlapping(old_ptr.as_ptr(), new_ptr, self.size);
				alloc::dealloc(
					old_ptr.as_ptr() as *mut u8,
					Layout::array::<T>(self.capacity)
						.expect("Cannot recreate layout? Has capacity been edited?"),
				);
			}
		}
		self.data = new_data;
		self.capacity = new_cap;
	}

	pub fn push(&mut self, elem: T) {
		if self.data.is_none() {
			self.reserve(2);
		} else if self.size == self.capacity {
			if self.capacity == usize::MAX {
				panic!("Overflow");
			}
			self.reserve(
				(self.capacity as f64 * GROWTH_RATE)
					.ceil()
					.min(usize::MAX as f64) as usize,
			);
		}
		assert!(self.size < self.capacity);
		assert!(self.data.is_some());
		let data_ptr = unsafe { self.data.unwrap().as_ptr().add(self.size).as_mut().unwrap() };
		*data_ptr = elem;
		self.size += 1;
	}

	pub fn get(&self, idx: usize) -> Option<&T> {
		if idx >= self.size {
			return None;
		}
		if let Some(ptr) = self.data {
			let ptr = unsafe { &*ptr.as_ptr().add(idx) };
			Some(ptr)
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
		if idx >= self.size {
			return None;
		}
		if let Some(ptr) = self.data {
			let ptr = unsafe { &mut *ptr.as_ptr().add(idx) };
			Some(ptr)
		} else {
			None
		}
	}

	pub fn insert(&mut self, idx: usize, elem: T) {
		if idx == self.size {
			return self.push(elem);
		}

		if self.data.is_none() {
			self.reserve(2);
		} else if self.size == self.capacity {
			if self.capacity == usize::MAX {
				panic!("Overflow");
			}
			self.reserve(
				(self.capacity as f64 * GROWTH_RATE)
					.ceil()
					.min(usize::MAX as f64) as usize,
			);
		}
		assert!(self.size < self.capacity);
		assert!(self.data.is_some());
		let data_ptr = self
			.data
			.expect("Vector's data pointer is null despite being just checked?")
			.as_ptr();

		for i in (idx..=self.size).rev() {
			unsafe { data_ptr.add(i + 1).write(data_ptr.add(i).read()) };
		}
		unsafe { data_ptr.add(idx).write(elem) };

		self.size += 1;
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.size == 0 || self.data.is_none() {
			return None;
		}
		let data_ptr = self.data.expect("Check above was incorrect?").as_ptr();
		self.size -= 1;
		Some(unsafe { data_ptr.add(self.size).read() })
	}

	pub fn remove(&mut self, idx: usize) -> T {
		if idx == self.size {
			return self.pop().expect("Vector is empty");
		}
		if self.size == 0 || self.data.is_none() {
			panic!("Vector is empty");
		}

		let data_ptr = self.data.expect("Check above was incorrect?").as_ptr();

		let ret = unsafe { data_ptr.add(idx).read() };
		for i in idx..self.size {
			unsafe { data_ptr.add(i).write(data_ptr.add(i + 1).read()) };
		}

		self.size -= 1;
		ret
	}

	///Borrows the vector's allocation as an immutable slice.
	pub fn as_slice(&self) -> &[T] {
		if let Some(ptr) = self.data {
			unsafe {
				ptr::slice_from_raw_parts(ptr.as_ptr(), self.size)
					.as_ref()
					.expect("Vector's internal NonNull pointer was null?")
			}
		} else {
			assert!(self.size == 0);
			&[]
		}
	}

	///Sets the length of the vector, within the existing capacity.
	/// # Safety
	/// Panics if len is greater than the vector's capacity.
	/// Exposes potentially uninitialised memory if len is greater than the vector's length.
	pub unsafe fn set_len(&mut self, len: usize) {
		if len > self.capacity {
			panic!();
		}
		self.size = len;
	}

	pub fn iter(&self) -> BorrowedVectorIterator<'_, T> {
		(&self).into_iter()
	}

	///Returns the pointer to the allocation of the Vector or
	/// `None` if nothing has been allocated yet
	pub fn as_ptr(&self) -> Option<*const T> {
		self.data.map(|p| p.as_ptr() as *const _)
	}
}
