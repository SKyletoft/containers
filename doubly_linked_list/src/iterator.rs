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
