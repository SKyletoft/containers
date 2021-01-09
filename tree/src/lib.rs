#![forbid(unsafe_code)]
use owned_pointer::OwnedPointer;
use std::cmp::Ordering::*;

pub struct Tree<T: PartialOrd> {
	val: Option<T>,
	left: Option<OwnedPointer<Tree<T>>>,
	right: Option<OwnedPointer<Tree<T>>>,
}

impl<T: PartialOrd> Default for Tree<T> {
	fn default() -> Self {
		Tree::new()
	}
}

impl<T: PartialOrd> Tree<T> {
	pub fn new() -> Self {
		Tree {
			val: None,
			left: None,
			right: None,
		}
	}

	/// Adds element to tree. Returns true if the tree already contains the element
	pub fn add(&mut self, elem: T) -> bool
	where
		T: PartialEq,
	{
		match self.val.as_ref().partial_cmp(&Some(&elem)) {
			Some(Equal) => true,
			Some(Less) => {
				let r = self.left.get_or_insert(OwnedPointer::new(Tree::new()));
				r.add(elem)
			}
			Some(Greater) => {
				let r = self.right.get_or_insert(OwnedPointer::new(Tree::new()));
				r.add(elem)
			}
			None => {
				self.val = Some(elem);
				false
			}
		}
	}

	fn get_mut(&mut self, elem: &T) -> Option<&mut Tree<T>>
	where
		T: PartialEq,
	{
		match self.val.as_ref().partial_cmp(&Some(&elem)) {
			Some(Equal) => Some(self),
			Some(Less) => {
				let r = self.left.as_deref_mut()?;
				r.get_mut(elem)
			}
			Some(Greater) => {
				let r = self.right.as_deref_mut()?;
				r.get_mut(elem)
			}
			None => None,
		}
	}

	fn get(&self, elem: &T) -> Option<&Tree<T>>
	where
		T: PartialEq,
	{
		match self.val.as_ref().partial_cmp(&Some(&elem)) {
			Some(Equal) => Some(self),
			Some(Less) => {
				let r = self.left.as_deref()?;
				r.get(elem)
			}
			Some(Greater) => {
				let r = self.right.as_deref()?;
				r.get(elem)
			}
			None => None,
		}
	}

	pub fn remove(&mut self, elem: &T) -> Option<T>
	where
		T: PartialEq,
	{
		let r = self.get_mut(elem)?;
		r.val.take()
	}

	pub fn contains(&self, elem: &T) -> bool
	where
		T: PartialEq,
	{
		self.get(elem).is_some()
	}

	pub fn clean(&mut self) {
		if let Some(l) = &mut self.left {
			l.clean();
			if l.left.is_none() && l.right.is_none() && l.val.is_none() {
				self.left = None;
			}
		}
		if let Some(r) = &mut self.right {
			r.clean();
			if r.left.is_none() && r.right.is_none() && r.val.is_none() {
				self.right = None;
			}
		}
	}
}
