use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ListNode<T> {
	pub(crate) val: T,
	pub(crate) next: Option<NonNull<ListNode<T>>>,
	pub(crate) prev: Option<NonNull<ListNode<T>>>,
}

impl<T> ListNode<T> {
	pub fn new_alloc(elem: ListNode<T>) -> Option<NonNull<ListNode<T>>> {
		let layout = Layout::for_value(&elem);
		let ptr = unsafe {
			//Safety: Assertion catches failed allocations, therefore anything
			// that doesn't immediately panic is safe, trusting that alloc respects
			// the layout passed to it
			let ptr = alloc::alloc(layout) as *mut ListNode<T>;
			assert!(!ptr.is_null(), "Allocation failed");
			ptr.write(elem);
			ptr
		};
		NonNull::new(ptr)
	}
}
