use crate::*;

#[test]
fn add_one() {
	let mut vec = Vector::new();
	vec.push(1);
	assert_eq!(vec.get(0), Some(&1));
}

#[test]
fn insert() {
	let mut vec = Vector::new();
	vec.push(1);
	vec.push(2);
	vec.push(4);
	vec.push(5);
	vec.insert(2, 3);
	assert_eq!(vec.get(0), Some(&1));
	assert_eq!(vec.get(1), Some(&2));
	assert_eq!(vec.get(2), Some(&3));
	assert_eq!(vec.get(3), Some(&4));
	assert_eq!(vec.get(4), Some(&5));
}

#[test]
fn remove() {
	let mut vec = Vector::new();
	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.push(4);
	vec.push(5);
	vec.remove(3);
	assert_eq!(vec.get(0), Some(&1));
	assert_eq!(vec.get(1), Some(&2));
	assert_eq!(vec.get(2), Some(&3));
	assert_eq!(vec.get(3), Some(&5));
	assert_eq!(vec.get(4), None);
}

#[test]
fn as_slice() {
	let mut vec = Vector::new();
	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.push(4);
	vec.push(5);
	assert_eq!(vec.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.push(4);
	vec.push(5);
	let std_vec = vec![1, 2, 3, 4, 5];
	let mut std_iter = std_vec.into_iter();
	let mut iter = vec.into_iter();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn back_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.push(4);
	vec.push(5);
	let std_vec = vec![1, 2, 3, 4, 5];
	let mut std_iter = std_vec.into_iter().rev();
	let mut iter = vec.into_iter().rev();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn both_iterator() {
	let mut list = Vector::new();
	list.push(1);
	list.push(2);
	list.push(3);
	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next_back(), Some(3));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next_back(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn borrowed_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.push(4);
	vec.push(5);
	let std_vec = vec![1, 2, 3, 4, 5];
	let mut std_iter = std_vec.iter();
	let mut iter = vec.iter();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn borrowed_back_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(1);
	vec.push(2);
	vec.push(3);
	vec.push(4);
	vec.push(5);
	let std_vec = vec![1, 2, 3, 4, 5];
	let mut std_iter = std_vec.iter().rev();
	let mut iter = vec.iter().rev();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn borrowed_both_iterator() {
	let mut list = Vector::new();
	list.push(1);
	list.push(2);
	list.push(3);
	let mut iter = list.iter();
	assert_eq!(iter.next(), Some(&1));
	assert_eq!(iter.next_back(), Some(&3));
	assert_eq!(iter.next(), Some(&2));
	assert_eq!(iter.next_back(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn drop() {
	static mut SUM: i32 = 0;
	#[derive(Clone, Debug)]
	struct ToDrop {
		b: u8,
	}
	impl Drop for ToDrop {
		fn drop(&mut self) {
			unsafe {
				SUM += 1;
			}
		}
	}
	{
		let mut vec = Vector::with_capacity(10);
		for i in 0..10 {
			vec.push(ToDrop { b: i });
		}
		for _ in vec.iter() {}
		for _ in vec.into_iter() {}
	}
	assert_eq!(unsafe { SUM }, 10);
}
