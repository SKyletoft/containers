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
