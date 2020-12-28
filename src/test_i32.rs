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
	assert_eq!(list.len, 5);
	let removed = list.remove(2);
	assert_eq!(list.len, 4);
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(1), Some(&2));
	assert_eq!(list.get(2), Some(&4));
	assert_eq!(list.get(3), Some(&5));
	assert_eq!(list.get(4), None);
	assert_eq!(removed, 3);
	assert_eq!(list.get_internal(0).unwrap().prev, None);
	assert_eq!(list.get_internal(3).unwrap().next, None);
}

#[test]
fn remove_first() {
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
	assert_eq!(list.len, 5);
	let removed = list.remove(0);
	assert_eq!(list.len, 4);
	assert_eq!(list.get(0), Some(&2));
	assert_eq!(list.get(1), Some(&3));
	assert_eq!(list.get(2), Some(&4));
	assert_eq!(list.get(3), Some(&5));
	assert_eq!(list.get(4), None);
	assert_eq!(removed, 1);
	assert_eq!(list.get_internal(0).unwrap().prev, None);
	assert_eq!(list.get_internal(3).unwrap().next, None);
}

#[test]
fn remove_last() {
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
	assert_eq!(list.len, 5);
	let removed = list.remove(4);
	assert_eq!(list.len, 4);
	assert_eq!(list.get(0), Some(&1));
	assert_eq!(list.get(1), Some(&2));
	assert_eq!(list.get(2), Some(&3));
	assert_eq!(list.get(3), Some(&4));
	assert_eq!(list.get(4), None);
	assert_eq!(removed, 5);
	assert_eq!(list.get_internal(0).unwrap().prev, None);
	assert_eq!(list.get_internal(3).unwrap().next, None);
}

#[test]
fn into_iterator() {
	let mut list = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	let vec = vec![1, 2, 3, 4, 5];
	let mut list_iter = list.into_iter();
	let mut vec_iter = vec.into_iter();
	for _ in 0..6 {
		assert_eq!(list_iter.next(), vec_iter.next());
	}
	assert_eq!(list_iter.next(), None);
	assert_eq!(vec_iter.next(), None);
}

#[test]
fn borrowed_iterator() {
	let mut list = List::new();
	list.push_back(1);
	list.push_back(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	let vec = vec![1, 2, 3, 4, 5];
	let mut list_iter = list.iter();
	let mut vec_iter = vec.iter();
	for _ in 0..6 {
		assert_eq!(list_iter.next(), vec_iter.next());
	}
	assert_eq!(list_iter.next(), None);
	assert_eq!(vec_iter.next(), None);
}

#[test]
fn from_iter() {
	let list: List<i32> = (1..=5).collect();
	let mut list_iter = list.into_iter();
	assert_eq!(list_iter.next(), Some(1));
	assert_eq!(list_iter.next(), Some(2));
	assert_eq!(list_iter.next(), Some(3));
	assert_eq!(list_iter.next(), Some(4));
	assert_eq!(list_iter.next(), Some(5));
	assert_eq!(list_iter.next(), None);
}
