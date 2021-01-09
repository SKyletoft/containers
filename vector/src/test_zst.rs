use crate::*;

#[derive(Copy, Clone, Debug, PartialEq)]
struct ZST;

#[test]
fn add_one() {
	let mut vec = Vector::new();
	vec.push(1);
	assert_eq!(vec.get(0), Some(&1));
}

#[test]
fn insert() {
	let mut vec = Vector::new();
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.insert(2, ZST);
	assert_eq!(vec.get(0), Some(&ZST));
	assert_eq!(vec.get(1), Some(&ZST));
	assert_eq!(vec.get(2), Some(&ZST));
	assert_eq!(vec.get(3), Some(&ZST));
	assert_eq!(vec.get(4), Some(&ZST));
}

#[test]
fn remove() {
	let mut vec = Vector::new();
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.remove(3);
	assert_eq!(vec.get(0), Some(&ZST));
	assert_eq!(vec.get(1), Some(&ZST));
	assert_eq!(vec.get(2), Some(&ZST));
	assert_eq!(vec.get(3), Some(&ZST));
	assert_eq!(vec.get(4), None);
}

#[test]
fn as_slice() {
	let mut vec = Vector::new();
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	assert_eq!(vec.as_slice(), &[ZST, ZST, ZST, ZST, ZST]);
}

#[test]
fn iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	let std_vec = vec![ZST, ZST, ZST, ZST, ZST];
	let mut std_iter = std_vec.into_iter();
	let mut iter = vec.into_iter();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn back_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	let std_vec = vec![ZST, ZST, ZST, ZST, ZST];
	let mut std_iter = std_vec.into_iter().rev();
	let mut iter = vec.into_iter().rev();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn both_iterator() {
	let mut list = Vector::new();
	list.push(ZST);
	list.push(ZST);
	list.push(ZST);
	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(ZST));
	assert_eq!(iter.next_back(), Some(ZST));
	assert_eq!(iter.next(), Some(ZST));
	assert_eq!(iter.next_back(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn borrowed_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	let std_vec = vec![ZST, ZST, ZST, ZST, ZST];
	let mut std_iter = std_vec.iter();
	let mut iter = vec.iter();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn borrowed_back_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	vec.push(ZST);
	let std_vec = vec![ZST, ZST, ZST, ZST, ZST];
	let mut std_iter = std_vec.iter().rev();
	let mut iter = vec.iter().rev();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn borrowed_both_iterator() {
	let mut list = Vector::new();
	list.push(ZST);
	list.push(ZST);
	list.push(ZST);
	let mut iter = list.iter();
	assert_eq!(iter.next(), Some(&ZST));
	assert_eq!(iter.next_back(), Some(&ZST));
	assert_eq!(iter.next(), Some(&ZST));
	assert_eq!(iter.next_back(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn drop() {
	static mut SUM: i32 = 0;
	#[derive(Clone, Debug)]
	struct ToDrop;
	impl Drop for ToDrop {
		fn drop(&mut self) {
			unsafe {
				SUM += 1;
			}
		}
	}
	{
		let mut vec = Vector::with_capacity(10);
		for _ in 0..10 {
			vec.push(ToDrop);
		}
		for _ in vec.iter() {}
		for _ in vec.into_iter() {}
	}
	assert_eq!(unsafe { SUM }, 10);
}
