use std::{
	alloc,
	alloc::Layout,
	error::Error,
	fmt,
	iter::FromIterator,
	mem,
	ops::{Index, IndexMut},
	ptr,
	ptr::NonNull,
};
#[cfg(test)]
pub mod test_i32;
#[cfg(test)]
pub mod test_vec;

pub mod list_node;
use list_node::ListNode;

pub mod iterator;
use iterator::{BorrowedListIterator, ListIterator};

pub mod error;

pub struct List<T> {
	pub(crate) start: Option<NonNull<ListNode<T>>>,
	pub(crate) end: Option<NonNull<ListNode<T>>>,
	pub(crate) len: usize,
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_empty() {
			return write!(f, "[]");
		}
		write!(f, "[")?;
		for elem in self.iter().take(self.len()) {
			write!(f, "{:?}, ", elem)?;
		}
		write!(
			f,
			"{:?}]",
			self.get_back(0).expect("length already checked?")
		)
	}
}

impl<T> Default for List<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Index<usize> for List<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		if index <= self.len / 2 {
			self.get(index).expect("Out of bounds")
		} else {
			self.get_back(self.len - index).expect("Out of bounds")
		}
	}
}

impl<T> IndexMut<usize> for List<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		if index <= self.len / 2 {
			self.get_mut(index).expect("Out of bounds")
		} else {
			self.get_back_mut(self.len - index).expect("Out of bounds")
		}
	}
}

impl<T> Index<isize> for List<T> {
	type Output = T;

	fn index(&self, index: isize) -> &Self::Output {
		if index.is_positive() {
			self.get(index as usize).expect("Out of bounds")
		} else {
			self.get_back(index.abs() as usize).expect("Out of bounds")
		}
	}
}

impl<T> IndexMut<isize> for List<T> {
	fn index_mut(&mut self, index: isize) -> &mut Self::Output {
		if index.is_positive() {
			self.get_mut(index as usize).expect("Out of bounds")
		} else {
			self.get_back_mut(index.abs() as usize)
				.expect("Out of bounds")
		}
	}
}

impl<T> IntoIterator for List<T> {
	type Item = T;

	type IntoIter = ListIterator<T>;

	fn into_iter(self) -> Self::IntoIter {
		ListIterator { list: self }
	}
}

impl<'a, T> IntoIterator for &'a List<T> {
	type Item = &'a T;

	type IntoIter = BorrowedListIterator<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		BorrowedListIterator {
			list: self,
			node: None,
		}
	}
}

impl<T> FromIterator<T> for List<T> {
	fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
		let mut list = List::new();
		for item in iter {
			list.push_back(item);
		}
		list
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		while self.start.is_some() && self.end.is_some() && self.len > 0 {
			self.pop_front();
		}
		assert!(self.start.is_none());
		assert!(self.end.is_none());
		assert_eq!(self.len, 0);
	}
}

impl<T: Clone> Clone for List<T> {
	fn clone(&self) -> Self {
		self.iter().cloned().collect()
	}
}

impl<T: PartialEq> PartialEq for List<T> {
	fn eq(&self, other: &Self) -> bool {
		if self.len != other.len {
			return false;
		}
		self.iter().zip(other.iter()).all(|(a, b)| a == b)
	}
}

impl<'a, T> List<T> {
	pub fn new() -> Self {
		Self {
			start: None,
			end: None,
			len: 0,
		}
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	pub fn iter(&'a self) -> BorrowedListIterator<'a, T> {
		self.into_iter()
	}

	pub fn push_back(&mut self, elem: T) {
		if let Some(mut last) = self.end {
			assert!(self.start.is_some());
			assert!(unsafe { last.as_ref() }.next.is_none());
			assert!(self.len > 0);
			let ptr = ListNode::new_alloc(ListNode {
				next: None,
				prev: self.end,
				val: elem,
			});
			self.end = ptr;
			unsafe { last.as_mut() }.next = ptr;
		} else {
			assert!(self.start.is_none());
			assert_eq!(self.len, 0);
			let ptr = ListNode::new_alloc(ListNode {
				next: None,
				prev: None,
				val: elem,
			});
			self.start = ptr;
			self.end = ptr;
		}
		self.len += 1;
	}

	pub fn push_front(&mut self, elem: T) {
		if let Some(mut first) = self.start {
			assert!(self.end.is_some());
			assert!(unsafe { first.as_ref() }.prev.is_none());
			assert!(self.len > 0);
			let ptr = ListNode::new_alloc(ListNode {
				next: self.start,
				prev: None,
				val: elem,
			});
			self.start = ptr;
			unsafe { first.as_mut() }.prev = ptr;
			self.len += 1;
		} else {
			assert!(self.end.is_none());
			self.push_back(elem);
		}
	}

	pub fn insert(&mut self, index: usize, elem: T) {
		if index <= self.len() / 2 {
			self.insert_front(index, elem)
		} else {
			self.insert_back(self.len() - index - 1, elem)
		}
	}

	pub fn insert_front(&mut self, index: usize, elem: T) {
		if self.len() < index {
			panic!("Index beyond last element of list!");
		}
		if self.len() == index {
			return self.push_back(elem);
		}
		if index == 0 {
			return self.push_front(elem);
		}
		let curr = self.get_internal_mut(index).expect("Error in an insertion function, index is less than claimed length yet no element exists at index");

		let mut last_ptr = curr.prev.expect("Pointer to previous missing!?");
		let ptr = ListNode::new_alloc(ListNode {
			next: Some(NonNull::new(curr as *mut ListNode<T>).expect("Pointer already checked?")),
			prev: curr.prev,
			val: elem,
		});

		unsafe { last_ptr.as_mut() }.next = ptr;
		curr.prev = ptr;
		self.len += 1;
	}

	pub fn insert_back(&mut self, index: usize, elem: T) {
		if self.len() < index {
			panic!("Index beyond first element of list!");
		}
		if self.len() == index {
			return self.push_front(elem);
		}
		if index == 0 {
			return self.push_back(elem);
		}
		let curr = self.get_internal_back_mut(index).expect("Error in an insertion function, index is less than claimed length yet no element exists at index");

		let mut ptr_from_last = curr.prev.expect("Pointer to previous missing!?");
		let ptr = ListNode::new_alloc(ListNode {
			next: Some(NonNull::new(curr as *mut ListNode<T>).expect("Pointer already checked?")),
			prev: curr.prev,
			val: elem,
		});

		unsafe { ptr_from_last.as_mut() }.next = ptr;
		curr.prev = ptr;
		self.len += 1;
	}

	fn get_internal(&self, index: usize) -> Option<&ListNode<T>> {
		let mut curr = self.start.as_ref()?;
		for _ in 0..index {
			curr = unsafe { curr.as_ref() }.next.as_ref()?;
		}
		Some(unsafe { curr.as_ref() })
	}

	fn get_internal_back(&self, index: usize) -> Option<&ListNode<T>> {
		let mut curr = self.end.as_ref()?;
		for _ in 0..index {
			curr = unsafe { curr.as_ref() }.prev.as_ref()?;
		}
		Some(unsafe { curr.as_ref() })
	}

	fn get_internal_mut(&mut self, index: usize) -> Option<&mut ListNode<T>> {
		let mut curr = self.start.as_mut()?;
		for _ in 0..index {
			curr = unsafe { curr.as_mut() }.next.as_mut()?;
		}
		Some(unsafe { curr.as_mut() })
	}

	fn get_internal_back_mut(&mut self, index: usize) -> Option<&mut ListNode<T>> {
		let mut curr = self.end.as_mut()?;
		for _ in 0..index {
			curr = unsafe { curr.as_mut() }.prev.as_mut()?;
		}
		Some(unsafe { curr.as_mut() })
	}

	pub fn get(&self, index: usize) -> Option<&T> {
		self.get_internal(index).map(|v| &v.val)
	}

	pub fn get_back(&self, index: usize) -> Option<&T> {
		self.get_internal_back(index).map(|v| &v.val)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.get_internal_mut(index).map(|v| &mut v.val)
	}

	pub fn get_back_mut(&mut self, index: usize) -> Option<&mut T> {
		self.get_internal_back_mut(index).map(|v| &mut v.val)
	}

	pub fn pop_front(&mut self) -> T {
		assert!(!self.is_empty());
		self.len -= 1;
		let mut first = self
			.start
			.expect("Bounds already checked? len is incorrectly set");
		let element = unsafe { first.as_mut() };
		self.start = element.next;
		let ret = unsafe { ptr::read(&element.val as *const T) };
		if let Some(mut ptr) = self.start {
			let new_start = unsafe { ptr.as_mut() };
			new_start.prev = None;
		}
		let ptr = element as *mut ListNode<T>;
		let layout = Layout::for_value(element);
		unsafe { alloc::dealloc(ptr as *mut u8, layout) };
		if self.is_empty() {
			assert!(self.start.is_none());
			self.end = None;
		}
		ret
	}

	pub fn pop_back(&mut self) -> T {
		assert!(!self.is_empty());
		self.len -= 1;
		let mut last = self
			.end
			.expect("Bounds already checked? len is incorrectly set");
		let element = unsafe { last.as_mut() };
		self.end = element.prev;
		let ret = unsafe { ptr::read(&element.val as *const T) };
		if let Some(mut ptr) = self.end {
			let new_end = unsafe { ptr.as_mut() };
			new_end.next = None;
		}
		let ptr = element as *mut ListNode<T>;
		let layout = Layout::for_value(element);
		unsafe { alloc::dealloc(ptr as *mut u8, layout) };
		if self.is_empty() {
			assert!(self.end.is_none());
			self.start = None;
		}
		ret
	}

	pub fn remove(&mut self, index: usize) -> T {
		if index <= self.len() / 2 {
			self.remove_front(index)
		} else {
			self.remove_back(self.len() - index - 1)
		}
	}

	pub fn remove_front(&mut self, index: usize) -> T {
		if index == 0 {
			return self.pop_front();
		}
		if index == self.len - 1 {
			return self.pop_back();
		}
		self.len -= 1;
		let element = self.get_internal_mut(index).expect("Out of bounds?");
		let mut prev = element.prev.expect("Previous node missing!");
		let mut next = element.next.expect("Next node missing!");
		let prev_r = unsafe { prev.as_mut() };
		let next_r = unsafe { next.as_mut() };
		prev_r.next = element.next;
		next_r.prev = element.prev;

		let ret = unsafe { ptr::read(&element.val as *const T) };
		let ptr = element as *mut ListNode<T>;
		let layout = Layout::for_value(element);
		unsafe { alloc::dealloc(ptr as *mut u8, layout) };
		ret
	}

	pub fn remove_back(&mut self, index: usize) -> T {
		if index == 0 {
			return self.pop_back();
		}
		if index == self.len - 1 {
			return self.pop_front();
		}
		self.len -= 1;
		let element = self.get_internal_back_mut(index).expect("Out of bounds?");
		let mut prev = element.prev.expect("Previous node missing!");
		let mut next = element.next.expect("Next node missing!");
		let prev_r = unsafe { prev.as_mut() };
		let next_r = unsafe { next.as_mut() };
		prev_r.next = element.next;
		next_r.prev = element.prev;

		let ret = unsafe { ptr::read(&element.val as *const T) };
		let ptr = element as *mut ListNode<T>;
		let layout = Layout::for_value(element);
		unsafe { alloc::dealloc(ptr as *mut u8, layout) };
		ret
	}

	pub fn split_off(&mut self, index: usize) -> Self {
		if index >= self.len {
			panic!()
		}
		if index == 0 {
			let mut list = List::new();
			mem::swap(self, &mut list);
			return list;
		}

		let mut list = List::new();

		let last = if index <= self.len() / 2 {
			self.get_internal_mut(index).expect("len is misset")
		} else {
			self.get_internal_back_mut(self.len() - index - 1)
				.expect("len is misset")
		};
		let prev = last.prev;

		last.prev = None;
		list.start = NonNull::new(last);
		list.end = self.end;
		list.len = self.len - index;

		self.len = index;
		self.end = prev;
		if let Some(r) = self.get_internal_back_mut(0) {
			r.next = None;
		}
		list
	}

	pub fn append(&mut self, other: &mut Self) {
		if self.is_empty() {
			mem::swap(self, other);
			return;
		}
		if other.is_empty() {
			return;
		}
		self.get_internal_back_mut(0).unwrap().next = other.start;
		other.get_internal_mut(0).unwrap().prev = self.end;
		self.end = other.end;
		self.len += other.len;
		other.len = 0;
		other.start = None;
		other.end = None;
	}

	pub fn prepend(&mut self, other: &mut Self) {
		if self.is_empty() {
			mem::swap(self, other);
			return;
		}
		if other.is_empty() {
			return;
		}
		self.get_internal_mut(0).unwrap().prev = other.end;
		other.get_internal_back_mut(0).unwrap().next = self.start;
		self.start = other.start;
		self.len += other.len;
		other.len = 0;
		other.start = None;
		other.end = None;
	}
}
