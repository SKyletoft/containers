#[cfg(test)]
pub mod test_i32;
#[cfg(test)]
pub mod test_vec;

pub mod list_node;
use list_node::ListNode;

pub mod error;

use std::{alloc, alloc::Layout, error::Error, fmt, ptr, ptr::NonNull};

#[derive(Clone, PartialEq)]
pub struct List<T> {
	pub(crate) start: Option<NonNull<ListNode<T>>>,
	pub(crate) end: Option<NonNull<ListNode<T>>>,
	pub(crate) len: usize,
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

impl<T> Default for List<T> {
	fn default() -> Self {
		Self::new()
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
