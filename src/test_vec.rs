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
	list.push_back(Box::new([0b00000000u8; 24]));
	let ptr = list.get(0).unwrap() as *const Box<[u8; 24]>;
	dbg!(unsafe { ptr::read(ptr) });
	list.push_back(Box::new([0b10101010u8; 24]));
	dbg!(unsafe { ptr::read(ptr) });
	list.push_back(Box::new([0b01010101u8; 24]));
	dbg!(unsafe { ptr::read(ptr) });
	assert_eq!(list.len, 3);
	assert_ne!(list.start, list.end);
	assert!(list.start.is_some());
	assert!(list.end.is_some());
	assert_eq!(list.get(0), Some(&Box::new([0b00000000u8; 24])));
	assert_eq!(list.get(1), Some(&Box::new([0b10101010u8; 24])));
	assert_eq!(list.get(2), Some(&Box::new([0b01010101u8; 24])));
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
