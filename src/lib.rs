use std::{alloc, alloc::Layout, error::Error, fmt, ptr, ptr::NonNull};

#[derive(Clone, PartialEq)]
pub struct List<T> {
	start: Option<NonNull<ListNode<T>>>,
	end: Option<NonNull<ListNode<T>>>,
	len: usize,
}

impl<T: fmt::Debug> fmt::Debug for List<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(
			f,
			"\nstart: {:?}\nend: {:?}\nlen: {}",
			self.start, self.end, self.len
		)?;
		for i in 0..=self.len {
			writeln!(f, "{:?}", self.get(i))?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
struct ListNode<T> {
	val: T,
	next: Option<NonNull<ListNode<T>>>,
	prev: Option<NonNull<ListNode<T>>>,
}

impl<T> ListNode<T> {
	fn new_alloc(elem: ListNode<T>) -> Option<NonNull<ListNode<T>>> {
		let layout = Layout::for_value(&elem);
		let ptr = unsafe {
			let ptr = alloc::alloc(layout) as *mut ListNode<T>;
			assert!(!ptr.is_null(), "Allocation failed");
			ptr.write(elem);
			ptr
		};
		NonNull::new(ptr)
	}
}

impl<T> List<T> {
	pub fn new() -> Self {
		Self {
			start: None,
			end: None,
			len: 0,
		}
	}

	pub fn push_back(&mut self, elem: T) {
		if let Some(mut last) = self.end {
			assert!(unsafe { last.as_ptr().read() }.next.is_none());
			let ptr = ListNode::new_alloc(ListNode {
				next: None,
				prev: self.end,
				val: elem,
			});
			self.end = ptr;
			unsafe { last.as_mut() }.next = ptr;
		} else {
			assert!(self.start.is_none());
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
			assert!(unsafe { first.as_ptr().read() }.prev.is_none());
			let ptr = ListNode::new_alloc(ListNode {
				next: self.start,
				prev: None,
				val: elem,
			});
			self.start = ptr;
			unsafe { first.as_mut() }.prev = ptr;
			self.len += 1;
		} else {
			self.push_back(elem);
		}
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

	pub fn insert(&mut self, index: usize, elem: T) {
		if self.len < index {
			panic!("Index beyond last element of list!");
		}
		if self.len == index {
			return self.push_back(elem);
		}
		if index == 0 {
			return self.push_front(elem);
		}
		let curr = self.get_internal_mut(index).expect("Error in an insertion function, index is less than claimed length yet no element exists at index");

		if let Some(mut ptr_to_this) = curr.prev {
			let ptr = ListNode::new_alloc(ListNode {
				next: Some(
					NonNull::new(curr as *mut ListNode<T>).expect("Pointer already checked?"),
				),
				prev: curr.prev,
				val: elem,
			});

			unsafe { ptr_to_this.as_mut() }.next = ptr;
		} else {
			panic!("Pointer to previous missing!?");
		}
	}

	pub fn remove(&mut self, index: usize) -> T {
		if index == 0 {}
		if index == self.len - 1 {}
		let element = self.get_internal_mut(index).expect("Out of bounds?");
		let mut prev = element.prev.expect("Previous node missing!");
		let mut next = element.next.expect("Next node missing!");
		let prev_r = unsafe { prev.as_mut() };
		let next_r = unsafe { next.as_mut() };
		prev_r.next = element.next;
		next_r.prev = element.prev;

		//I don't think this is sound. Is it copying .val to the stack without copying potentially owned data or what?
		//It is what std::vec::Vec seems to do though, for what that's worth
		let ret = unsafe { ptr::read(&element.val as *const T) };

		let ptr = element as *mut ListNode<T>;
		let layout = Layout::for_value(element);
		unsafe { alloc::dealloc(ptr as *mut u8, layout) };
		ret
	}
}

impl<T> Default for List<T> {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListError {}

impl fmt::Display for ListError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "An error occured, probably out of bounds")
	}
}

impl Error for ListError {}

#[cfg(test)]
mod tests_i32 {
	use crate::*;

	#[test]
	fn add_one_back() {
		let mut list = List::new();
		assert_eq!(list.len, 0);
		list.push_back(5);
		assert_eq!(list.len, 1);
		assert_eq!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(ptr::eq(
			list.start.unwrap().as_ptr(),
			list.end.unwrap().as_ptr()
		));
	}

	#[test]
	fn add_one_front() {
		let mut list = List::new();
		assert_eq!(list.len, 0);
		list.push_front(5);
		assert_eq!(list.len, 1);
		assert_eq!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(ptr::eq(
			list.start.unwrap().as_ptr(),
			list.end.unwrap().as_ptr()
		));
	}

	#[test]
	fn add_three_back() {
		let mut list = List::new();
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		assert_eq!(list.len, 3);
		assert_ne!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(list.end.is_some());
		assert_eq!(list.get(0), Some(&1));
		assert_eq!(list.get(1), Some(&2));
		assert_eq!(list.get(2), Some(&3));
		assert_eq!(list.get(3), None);
	}

	#[test]
	fn add_three_front() {
		let mut list = List::new();
		list.push_front(1);
		list.push_front(2);
		list.push_front(3);
		assert_eq!(list.len, 3);
		assert_ne!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(list.end.is_some());
		assert_eq!(list.get(0), Some(&3));
		assert_eq!(list.get(1), Some(&2));
		assert_eq!(list.get(2), Some(&1));
		assert_eq!(list.get(3), None);
	}

	#[test]
	fn insert() {
		let mut list = List::new();
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);
		list.push_back(5);
		list.insert(2, 10);
		assert_eq!(list.get(0), Some(&1));
		assert_eq!(list.get(1), Some(&2));
		assert_eq!(list.get(2), Some(&10));
		assert_eq!(list.get(3), Some(&3));
		assert_eq!(list.get(4), Some(&4));
		assert_eq!(list.get(5), Some(&5));
		assert_eq!(list.get(6), None);
	}

	#[test]
	fn remove() {
		let mut list = List::new();
		list.push_back(1);
		list.push_back(2);
		list.push_back(3);
		list.push_back(4);
		list.push_back(5);
		assert_eq!(list.get(0), Some(&1));
		assert_eq!(list.get(1), Some(&2));
		assert_eq!(list.get(2), Some(&3));
		assert_eq!(list.get(3), Some(&4));
		assert_eq!(list.get(4), Some(&5));
		assert_eq!(list.get(5), None);
		let removed = list.remove(2);
		assert_eq!(list.get(0), Some(&1));
		assert_eq!(list.get(1), Some(&2));
		assert_eq!(list.get(2), Some(&4));
		assert_eq!(list.get(3), Some(&5));
		assert_eq!(list.get(4), None);
		assert_eq!(removed, 3);
	}
}

#[cfg(test)]
mod tests_vec_i32 {
	use crate::*;

	#[test]
	fn add_one_back() {
		let mut list = List::new();
		assert_eq!(list.len, 0);
		list.push_back(vec![5]);
		assert_eq!(list.len, 1);
		assert_eq!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(ptr::eq(
			list.start.unwrap().as_ptr(),
			list.end.unwrap().as_ptr()
		));
		assert_eq!(list.get(0), Some(&vec![5]));
	}

	#[test]
	fn add_one_front() {
		let mut list = List::new();
		assert_eq!(list.len, 0);
		list.push_front(vec![5]);
		assert_eq!(list.len, 1);
		assert_eq!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(ptr::eq(
			list.start.unwrap().as_ptr(),
			list.end.unwrap().as_ptr()
		));
		assert_eq!(list.get(0), Some(&vec![5]));
	}

	#[test]
	fn add_three_back() {
		let mut list = List::new();
		list.push_back(vec![1]);
		dbg!(&list);
		list.push_back(vec![7]);
		dbg!(&list);
		list.push_back(vec![5]);
		dbg!(&list);
		assert_eq!(list.len, 3);
		assert_ne!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(list.end.is_some());
		assert_eq!(list.get(0), Some(&vec![1]));
		assert_eq!(list.get(1), Some(&vec![7]));
		assert_eq!(list.get(2), Some(&vec![5]));
		assert_eq!(list.get(3), None);
	}

	#[test]
	fn add_three_front() {
		let mut list = List::new();
		list.push_front(vec![1]);
		list.push_front(vec![2]);
		list.push_front(vec![3]);
		assert_eq!(list.len, 3);
		assert_ne!(list.start, list.end);
		assert!(list.start.is_some());
		assert!(list.end.is_some());
		assert_eq!(list.get(0), Some(&vec![3]));
		assert_eq!(list.get(1), Some(&vec![2]));
		assert_eq!(list.get(2), Some(&vec![1]));
		assert_eq!(list.get(3), None);
	}

	#[test]
	fn insert() {
		let mut list = List::new();
		list.push_back(vec![1]);
		list.push_back(vec![2]);
		list.push_back(vec![3]);
		list.push_back(vec![4]);
		list.push_back(vec![5]);
		list.insert(2, vec![10]);
		assert_eq!(list.get(0), Some(&vec![1]));
		assert_eq!(list.get(1), Some(&vec![2]));
		assert_eq!(list.get(2), Some(&vec![10]));
		assert_eq!(list.get(3), Some(&vec![3]));
		assert_eq!(list.get(4), Some(&vec![4]));
		assert_eq!(list.get(5), Some(&vec![5]));
		assert_eq!(list.get(6), None);
	}

	#[test]
	fn remove() {
		let mut list = List::new();
		list.push_back(vec![1]);
		list.push_back(vec![2]);
		list.push_back(vec![3]);
		list.push_back(vec![4]);
		list.push_back(vec![5]);
		assert_eq!(list.get(0), Some(&vec![1]));
		assert_eq!(list.get(1), Some(&vec![2]));
		assert_eq!(list.get(2), Some(&vec![3]));
		assert_eq!(list.get(3), Some(&vec![4]));
		assert_eq!(list.get(4), Some(&vec![5]));
		assert_eq!(list.get(5), None);
		let removed = list.remove(2);
		assert_eq!(list.get(0), Some(&vec![1]));
		assert_eq!(list.get(1), Some(&vec![2]));
		assert_eq!(list.get(2), Some(&vec![4]));
		assert_eq!(list.get(3), Some(&vec![5]));
		assert_eq!(list.get(4), None);
		assert_eq!(removed, vec![3]);
	}
}
