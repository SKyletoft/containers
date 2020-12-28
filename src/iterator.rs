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
	pub(crate) list: &'a List<T>,
	pub(crate) node: Option<&'a ListNode<T>>,
}

impl<'a, T> Iterator for BorrowedListIterator<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(node) = self.node {
			let ptr = node.next?;
			let next = unsafe { &*ptr.as_ptr() };
			self.node = Some(next);
			Some(&next.val)
		} else {
			let first = unsafe { &*self.list.start?.as_ptr() };
			self.node = Some(first);
			Some(&first.val)
		}
	}
}

impl<'a, T> DoubleEndedIterator for BorrowedListIterator<'a, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if let Some(node) = self.node {
			let ptr = node.prev?;
			let prev = unsafe { &*ptr.as_ptr() };
			self.node = Some(prev);
			Some(&prev.val)
		} else {
			let last = unsafe { &*self.list.end?.as_ptr() };
			self.node = Some(last);
			Some(&last.val)
		}
	}
}
