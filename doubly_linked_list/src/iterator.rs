use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ListIterator<T> {
	pub(crate) list: List<T>,
}

impl<T> Iterator for ListIterator<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.list.start?;
		Some(self.list.pop_front())
	}
}

impl<T> DoubleEndedIterator for ListIterator<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.list.end?;
		Some(self.list.pop_back())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct BorrowedListIterator<'a, T> {
	pub(crate) front: Option<&'a ListNode<T>>,
	pub(crate) back: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for BorrowedListIterator<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let front = self.front?;
		let back = self.back.expect("Back shouldn't be none if front wasn't");

		let value = &front.val;
		//Compares that actual allocations rather than values
		if ptr::eq(front, back) {
			//If equal, the iterator has been exhausted
			self.front = None;
			self.back = None;
		} else {
			let ptr = front.next.expect("Next should not be null");
			//Safety: pointers are trusted. As these pointers are only
			// read and not offset any incorrect behaviour is not caused here
			let next = unsafe { &*ptr.as_ptr() };
			self.front = Some(next);
		}
		Some(&value)
	}
}

impl<'a, T> DoubleEndedIterator for BorrowedListIterator<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let front = self.front?;
		let back = self.back.expect("Back shouldn't be none if front wasn't");

		let value = &back.val;
		//Compares that actual allocations rather than values
		if ptr::eq(front, back) {
			//If equal, the iterator has been exhausted
			self.front = None;
			self.back = None;
		} else {
			let ptr = back.prev.expect("Next should not be null");
			//Safety: pointers are trusted. As these pointers are only
			// read and not offset any incorrect behaviour is not caused here
			let prev = unsafe { &*ptr.as_ptr() };
			self.back = Some(prev);
		}
		Some(&value)
	}
}

///The iterator struct for mutably iterating over a list. It is
/// recommended that you use the `.iter_mut()` method on a list
/// rather than constructing this struct yourself.
///# Safety:
///  This struct is only safe to construct if you
///  can ensure you have a mutable borrow on the underlying
///  list, even if the references used are immutable.
#[derive(Debug, PartialEq)]
pub struct BorrowedListIteratorMut<'a, T> {
	pub(crate) front: Option<&'a mut ListNode<T>>,
	pub(crate) back: Option<&'a mut ListNode<T>>,
}

impl<'a, T> Iterator for BorrowedListIteratorMut<'a, T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		let front = self.front.as_mut()?;
		let back = self
			.back
			.as_mut()
			.expect("Back shouldn't be none if front wasn't");

		//Safety: Lifetime hack due to limitations of the borrow checker. Or a major mistake.
		// Tests work though. Check both the copy type (test_i32) and non-copy type (test_vec)
		// tests when making changes.
		// It should work because the lifetime of the val field of the node is the same as the
		// entire node, which is 'a
		let value = unsafe {
			(&mut front.val as *mut T)
				.as_mut()
				.expect("Broke the reference?")
		};
		//Compares that actual allocations rather than values
		if ptr::eq(*front, *back) {
			//If equal, the iterator has been exhausted
			self.front = None;
			self.back = None;
		} else {
			let ptr = front.next.expect("Next should not be null");
			//Safety: pointers are trusted. As these pointers are only
			// read and not offset any incorrect behaviour is not caused here
			let next = unsafe { &mut *ptr.as_ptr() };
			self.front = Some(next);
		}
		Some(value)
	}
}

impl<'a, T> DoubleEndedIterator for BorrowedListIteratorMut<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let front = self.front.as_mut()?;
		let back = self
			.back
			.as_mut()
			.expect("Back shouldn't be none if front wasn't");

		//Safety: Lifetime hack due to limitations of the borrow checker. Or a major mistake.
		// Tests work though. Check both the copy type (test_i32) and non-copy type (test_vec)
		// tests when making changes.
		// It should work because the lifetime of the val field of the node is the same as the
		// entire node, which is 'a
		let value = unsafe {
			(&mut back.val as *mut T)
				.as_mut()
				.expect("Broke the reference?")
		};
		//Compares that actual allocations rather than values
		if ptr::eq(*front, *back) {
			//If equal, the iterator has been exhausted
			self.front = None;
			self.back = None;
		} else {
			let ptr = back.prev.expect("Previous should not be null");
			//Safety: pointers are trusted. As these pointers are only
			// read and not offset any incorrect behaviour is not caused here
			let next = unsafe { &mut *ptr.as_ptr() };
			self.back = Some(next);
		}
		Some(value)
	}
}
