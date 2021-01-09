use std::{
	alloc,
	alloc::Layout,
	fmt,
	fmt::Debug,
	iter::FromIterator,
	mem,
	ops::{Index, IndexMut},
	ptr,
	ptr::NonNull,
};

#[cfg(test)]
pub mod test_box;

#[cfg(test)]
pub mod test_i32;

#[cfg(test)]
pub mod test_zst;

pub mod iterator;
use iterator::{BorrowedVectorIterator, BorrowedVectorIteratorMut, VectorIterator};

const GROWTH_RATE: f64 = 1.25;

///A resizable contiguous array of `T`. Does not allocate upon creation.
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

impl<T> IndexMut<usize> for Vector<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.get_mut(index).expect("Index was out of bounds")
	}
}

impl<T> IntoIterator for Vector<T> {
	type Item = T;

	type IntoIter = VectorIterator<T>;

	fn into_iter(mut self) -> Self::IntoIter {
		let Vector {
			data,
			capacity,
			size,
		} = self;
		//Moves the pointer out of the vector so that the allocation
		// won't be freed at the end of this block.
		self.data = None;
		self.size = 0;
		VectorIterator {
			data,
			capacity,
			index: -1isize as usize,
			index_back: size,
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

impl<'a, T> IntoIterator for &'a mut Vector<T> {
	type Item = &'a mut T;

	type IntoIter = BorrowedVectorIteratorMut<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		let size = self.size;
		BorrowedVectorIteratorMut {
			vector: self,
			index: -1isize as usize,
			index_back: size,
		}
	}
}

impl<T> FromIterator<T> for Vector<T> {
	fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
		let iter = iter.into_iter();
		let (min, _) = iter.size_hint();
		let mut vec = Vector::with_capacity(min);
		for item in iter {
			vec.push(item);
		}
		vec
	}
}

impl<T> Drop for Vector<T> {
	fn drop(&mut self) {
		//Outside the loop to handle zero size types
		self.clear();
		if let Some(ptr) = self.data {
			let ptr = ptr.as_ptr();
			let layout = Layout::array::<T>(self.capacity)
				.expect("Cannot recreate layout. Has capacity been changed?");
			//Safety: Capacity is only changed on reallocation, pointer is trusted
			// and iterators return to vectors for deallocation.
			unsafe { alloc::dealloc(ptr as *mut u8, layout) }
		}
	}
}

impl<T> Vector<T> {
	///Creates a new vector. Does not allocate till it's needed.
	pub fn new() -> Self {
		let capacity = if mem::size_of::<T>() == 0 {
			usize::MAX
		} else {
			0
		};
		Vector {
			data: None,
			size: 0,
			capacity,
		}
	}

	///Creates a new vector with a preallocated buffer with space for `cap` elements.
	pub fn with_capacity(cap: usize) -> Self {
		let mut vec = Vector::new();
		if mem::size_of::<T>() != 0 {
			vec.reserve(cap);
		}
		vec
	}

	///Checks if the vector has no elements in it. Does not check if there is an allocated buffer or not.
	pub fn is_empty(&self) -> bool {
		self.size == 0
	}

	///Returns the amount of elements stored in the vector.
	pub fn len(&self) -> usize {
		self.size
	}

	///Allocates a new buffer for the vector of specified size.
	///
	/// Panics if `new_cap` is smaller than current size or overflows a `usize`. Has O(n) complexity.
	fn reserve(&mut self, new_cap: usize) {
		assert_ne!(
			mem::size_of::<T>(),
			0,
			"Vector currently doesn't support storing 0 sized types"
		);
		let layout = Layout::array::<T>(new_cap).expect("Overflow");
		//Safety: Layout is type and capacity checked.
		let new_ptr = unsafe { alloc::alloc(layout) as *mut T };
		assert!(
			new_cap >= self.size,
			"New capacity can't contain current vector"
		);
		assert!(!new_ptr.is_null());
		let new_data = NonNull::new(new_ptr);
		if let Some(old_ptr) = self.data {
			unsafe {
				//Safety: The new allocation is a seperate allocation, so the copy is guaranteed to not overlap.
				ptr::copy_nonoverlapping(old_ptr.as_ptr(), new_ptr, self.size);
				//Safety: The pointer is only changed here in allocation.
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

	///Allocates a new buffer for the vector that is larger by `additional` elements.
	///
	/// Panics if `additional` causes it to overflow a `usize`. Has O(n) complexity.
	pub fn reserve_additional(&mut self, additional: usize) {
		if mem::size_of::<T>() == 0 {
			return;
		}
		let new_cap = self
			.capacity
			.checked_add(additional)
			.expect("New size overflowed usize");
		new_cap
			.checked_mul(mem::size_of::<T>())
			.expect("New size overflowed usize");
		self.reserve(new_cap);
	}

	///Inserts an element at the back of the vector.
	///
	/// Panics if the length of the vector is equal to usize::MAX. Has complexity O(1).
	pub fn push(&mut self, elem: T) {
		if self.data.is_none() && mem::size_of::<T>() != 0 {
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
		assert!(self.data.is_some() || (mem::size_of::<T>() == 0));
		//Safety: Length is checked. If the allocation was already full it is reallocated above.
		unsafe {
			self.as_ptr_mut()
				.expect("Above assertion failed?")
				.add(self.size)
				.write(elem)
		};
		self.size += 1;
	}

	///Gets a reference to the element at index's position.
	///
	/// Returns `None` if index is greater than the length of the vector. Has complexity O(1).
	pub fn get(&self, idx: usize) -> Option<&T> {
		if idx >= self.size {
			return None;
		}
		//Safety: Index is already checked.
		unsafe { self.as_ptr()?.add(idx).as_ref() }
	}

	///Gets a mutable reference to the element at index's position.
	///
	/// Returns `None` if index is greater than the length of the vector. Has complexity O(1).
	pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
		if idx >= self.size {
			return None;
		}
		//Safety: Index is already checked.
		unsafe { self.as_ptr_mut()?.add(idx).as_mut() }
	}

	///Inserts element in vector at index, moving everything after it to the right.
	/// Will reallocate if length equals capacity.
	///
	/// Panics if the vector's length will overflow `usize::MAX`. Has O(n) complexity.
	pub fn insert(&mut self, idx: usize, elem: T) {
		if idx == self.size {
			return self.push(elem);
		}

		if self.size == self.capacity {
			if self.capacity == usize::MAX {
				panic!("Overflow");
			}
			self.reserve(
				(self.capacity as f64 * GROWTH_RATE)
					.ceil()
					.min(usize::MAX as f64) as usize,
			);
		} else if self.data.is_none() && mem::size_of::<T>() != 0 {
			self.reserve(2);
		}
		assert!(self.size < self.capacity);
		assert!(self.data.is_some() || mem::size_of::<T>() == 0);
		let data_ptr = self
			.as_ptr_mut()
			.expect("Vector's data pointer is null despite being just checked?");

		for i in (idx..self.size).rev() {
			//Safety: Copies element by element within the size of the vector's allocation.
			// `self.size` keeps this within `self.size`.
			unsafe { data_ptr.add(i + 1).write(data_ptr.add(i).read()) };
		}
		//Safety: The element that was here has been moved, this is guaranteed in bounds.
		unsafe { data_ptr.add(idx).write(elem) };

		self.size += 1;
	}

	///Removes the last element in the vector
	///
	/// Returns `None` if the vector is empty. Has O(1) complexity.
	pub fn pop(&mut self) -> Option<T> {
		if self.size == 0 {
			return None;
		}

		self.size -= 1;
		let data_ptr = self.as_ptr_mut()?;
		//Safety: Existing pointer is trusted.
		Some(unsafe { data_ptr.add(self.size).read() })
	}

	///Removes the item at index, moving everything after that by one step to the left.
	/// If you're removing several elements, consider using the `retain` function for O(n)
	/// complexity instead of O(n²)
	///
	/// Panics if index >= to the vector's length. Has O(n) complexity.
	pub fn remove(&mut self, idx: usize) -> T {
		if idx >= self.size {
			panic!("Index was out of bounds!");
		}

		if idx == self.size {
			return self.pop().expect("Vector is empty");
		}
		if self.size == 0 || (self.data.is_none() && mem::size_of::<T>() != 0) {
			panic!("Vector is empty");
		}

		let data_ptr = self.as_ptr_mut().expect("Check above was incorrect?");

		//Safety: Index is checked and pointer is trusted.
		let ret = unsafe { data_ptr.add(idx).read() };
		for i in idx..(self.size - 1) {
			//Safety: Copies element by element within the size of the vector's allocation.
			// `self.size - 1 + 1` keeps this within `self.size`.
			unsafe { data_ptr.add(i).write(data_ptr.add(i + 1).read()) };
		}

		self.size -= 1;
		ret
	}

	///Removes every element in the vector.
	///
	/// Has O(n) complexity.
	pub fn clear(&mut self) {
		while !self.is_empty() {
			self.pop();
		}
	}

	///Borrows the vector's allocation as an immutable slice.
	///
	/// Has complexity O(1).
	pub fn as_slice(&self) -> &[T] {
		if self.data.is_some() || mem::size_of::<T>() == 0 {
			//Safety: Or existing pointer and size are trusted as they can't (safely)
			// be set from outside.
			unsafe {
				ptr::slice_from_raw_parts(
					self.as_ptr().expect("Cannot get pointer to create slice"),
					self.size,
				)
				.as_ref()
				.expect("Vector's internal NonNull pointer was null?")
			}
		} else {
			assert!(self.size == 0);
			&[]
		}
	}

	///Borrows the vector's allocation as a mutable slice.
	///
	/// Has complexity O(1).
	pub fn as_slice_mut(&mut self) -> &mut [T] {
		if self.data.is_some() || mem::size_of::<T>() == 0 {
			//Safety: Or existing pointer and size are trusted as they can't (safely)
			// be set from outside.
			unsafe {
				ptr::slice_from_raw_parts_mut(
					self.as_ptr_mut()
						.expect("Cannot get pointer to create slice"),
					self.size,
				)
				.as_mut()
				.expect("Vector's internal NonNull pointer was null?")
			}
		} else {
			assert!(self.size == 0);
			&mut []
		}
	}

	///Sets the length of the vector, within the existing capacity.
	///
	/// Has complexity O(1).
	/// # Safety
	/// Panics if len is greater than the vector's capacity.
	/// Exposes potentially uninitialised memory if len is greater than the vector's length.
	pub unsafe fn set_len(&mut self, len: usize) {
		if len > self.capacity {
			panic!();
		}
		self.size = len;
	}

	///Returns an iterator over borrowed elements of the vector.
	///
	/// Has complexity O(1).
	pub fn iter(&self) -> BorrowedVectorIterator<'_, T> {
		(&self).into_iter()
	}

	///Returns an iterator over mutably borrowed elements of the vector.
	///
	/// Has complexity O(1).
	pub fn iter_mut(&mut self) -> BorrowedVectorIteratorMut<'_, T> {
		(self).into_iter()
	}

	///Returns the pointer to the allocation of the Vector or
	/// `None` if nothing has been allocated yet.
	///
	/// Has complexity O(1).
	pub fn as_ptr(&self) -> Option<*const T> {
		if mem::size_of::<T>() == 0 {
			Some(self as *const Vector<T> as *const T)
		} else {
			self.data.map(|p| p.as_ptr() as *const _)
		}
	}

	///Returns the pointer to the allocation of the Vector or
	/// `None` if nothing has been allocated yet.
	///
	/// Has complexity O(1).
	pub fn as_ptr_mut(&mut self) -> Option<*mut T> {
		if mem::size_of::<T>() == 0 {
			Some(self as *mut Vector<T> as *mut T)
		} else {
			self.data.map(|p| p.as_ptr())
		}
	}

	///Removes any element which does not fulfill the requirement passed.
	/// It is recommended to use this over `remove` in a loop due to time
	/// complexity and fewer moves.
	///
	/// Has complexity O(n)
	pub fn retain(&mut self, f: fn(&T) -> bool) {
		if mem::size_of::<T>() == 0 {
			for i in (0..self.size).rev() {
				//Even if there is no data and the function can't actually depend
				// on the value of the element, the function might not be pure,
				// hence looping instead of one check and do nothing/clear all.
				if f(&self[i]) {
					self.pop();
				}
			}
			return;
		}
		if self.data.is_none() {
			return;
		}
		let ptr = self.data.expect("Above check failed?").as_ptr();
		let mut back = 0;
		for front in 0..self.size {
			let ok = f(&self[front]);
			if ok {
				if back != front {
					//Safety: Element is moved within the allocated space (as front is
					// always greater than back and front is bound by size) without extra
					// copies or clones which would be required as you otherwise can't move
					// out of a vector. The element which was overwritten had already been
					// moved or dropped.
					unsafe { ptr.add(back).write(ptr.add(front).read()) };
					back += 1;
				}
			} else {
				//Make sure drop is run and the element is not just left to be overwritten.
				let _ = unsafe { ptr.add(front).read() };
			}
		}
		self.size = back;
	}
}
