use crate::*;

pub struct VectorIterator<T> {
	pub(crate) data: Option<NonNull<T>>,
	pub(crate) capacity: usize,
	pub(crate) index: usize,
	pub(crate) index_back: usize,
}

impl<T> Iterator for VectorIterator<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.index = self.index.wrapping_add(1);
		if self.index >= self.capacity || self.index == self.index_back {
			self.index = self.index.wrapping_sub(1);
			return None;
		}
		//Safety: The data is only read from and the pointer is set to None when deallocated.
		// The origin of the pointer is in the Vector and any safety issues occur there.
		Some(unsafe { self.data?.as_ptr().add(self.index).read() })
	}
}

impl<T> DoubleEndedIterator for VectorIterator<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.index_back = self.index_back.wrapping_sub(1);
		if self.index_back == usize::MAX || self.index_back == self.index {
			self.index_back = self.index_back.wrapping_add(1);
			return None;
		}
		//Safety: The data is only read from and the pointer is set to None when deallocated.
		// The origin of the pointer is in the Vector and any safety issues occur there.
		Some(unsafe { self.data?.as_ptr().add(self.index_back).read() })
	}
}

//This is needed due to the pointers being moved into the iterator struct. This means that the vector
// is never droppped itself.
impl<T> Drop for VectorIterator<T> {
	fn drop(&mut self) {
		let cap = self.capacity;
		let data = self.data;
		//Do proper drops for remaining items in the iterator
		for _ in self {}
		if let Some(ptr) = data {
			let layout = Layout::array::<T>(cap).expect("Cannot recreate layout for deallocation from vector iterator, has capacity been edited?");
			unsafe { alloc::dealloc(ptr.as_ptr() as *mut u8, layout) }
		}
	}
}

pub struct BorrowedVectorIterator<'a, T> {
	pub(crate) vector: &'a Vector<T>,
	pub(crate) index: usize,
	pub(crate) index_back: usize,
}

impl<'a, T> Iterator for BorrowedVectorIterator<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		let next = self.index.wrapping_add(1);
		if next == self.index_back {
			return None;
		}
		self.index = next;
		self.vector.get(self.index)
	}
}

impl<'a, T> DoubleEndedIterator for BorrowedVectorIterator<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let next = self.index_back.wrapping_sub(1);
		if next == self.index {
			return None;
		}
		self.index_back = next;
		self.vector.get(self.index_back)
	}
}
