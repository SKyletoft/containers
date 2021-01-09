use std::{alloc, alloc::Layout, ops::Deref, ops::DerefMut, ptr::NonNull};

#[cfg(test)]
pub mod test_i32;

pub struct OwnedPointer<T> {
	val: NonNull<T>,
}

impl<T> Drop for OwnedPointer<T> {
	fn drop(&mut self) {
		let layout = {
			//Safety: The contained pointer is trusted. The inner scope will also
			// cause the value to be dropped before the allocation is deallocated.
			let elem = unsafe { self.val.as_ptr().read() };
			Layout::for_value(&elem)
		};
		unsafe { alloc::dealloc(self.val.as_ptr() as *mut u8, layout) };
	}
}

impl<T> AsMut<T> for OwnedPointer<T> {
	fn as_mut(&mut self) -> &mut T {
		//Safety: The object in question is owned and the element in inaccessible
		// from outside except for dedicated methods.
		unsafe { self.val.as_mut() }
	}
}

impl<T> AsRef<T> for OwnedPointer<T> {
	fn as_ref(&self) -> &T {
		//Safety: The object in question is owned and the element in inaccessible
		// from outside except for dedicated methods.
		unsafe { self.val.as_ref() }
	}
}

impl<T> Deref for OwnedPointer<T> {
	type Target = T;

	//Is this really correct? It feels wrong.
	fn deref(&self) -> &Self::Target {
		self.as_ref()
	}
}

impl<T> DerefMut for OwnedPointer<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut()
	}
}

impl<T> OwnedPointer<T> {
	///Allocates a new node containing the passed element. Will panic on allocation
	/// failure so any value returned will be valid.
	pub fn new(elem: T) -> OwnedPointer<T> {
		let layout = Layout::for_value(&elem);
		let val = NonNull::new(unsafe {
			//Safety: Assertion catches failed allocations, therefore anything
			// that doesn't immediately panic is safe, trusting that alloc respects
			// the layout passed to it
			alloc::alloc(layout) as *mut T
		})
		.expect("Allocation failed");
		unsafe { val.as_ptr().write(elem) };
		OwnedPointer { val }
	}
}
