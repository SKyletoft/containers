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
	list.insert(4, 11);
	list.insert(4, 12);
	list.push_back(123);
	dbg!(&list.len());
	dbg!(&list);
	let expected = vec![1, 2, 10, 3, 12, 11, 4, 5, 123];
	list.iter()
		.zip(expected.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list.iter()
		.rev()
		.zip(expected.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn remove() {
	let mut list: List<i32> = (1..10).collect();
	let expected_before = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
	let expected_mid = vec![1, 2, 4, 5, 6, 7, 8, 9];
	let expected_after = vec![1, 2, 4, 5, 6, 8, 9];
	assert_eq!(list.len(), 9);
	list.iter()
		.zip(expected_before.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list.iter()
		.rev()
		.zip(expected_before.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
	let removed_1 = list.remove(2);
	assert_eq!(list.len(), 8);
	list.iter()
		.zip(expected_mid.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list.iter()
		.rev()
		.zip(expected_mid.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
	let removed_2 = list.remove(5);
	assert_eq!(list.len(), 7);
	list.iter()
		.zip(expected_after.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list.iter()
		.rev()
		.zip(expected_after.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
	assert_eq!(removed_1, 3);
	assert_eq!(removed_2, 7);
	assert_eq!(list.get_internal(0).unwrap().prev, None);
	assert_eq!(list.get_internal_back(0).unwrap().next, None);
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

#[test]
fn split() {
	let mut list_1: List<i32> = (0..10).collect();
	let list_2 = list_1.split_off(5);
	assert_eq!(list_1.len, 5);
	assert_eq!(list_2.len, 5);
	let expected_1 = vec![0, 1, 2, 3, 4];
	let expected_2 = vec![5, 6, 7, 8, 9];

	list_1
		.iter()
		.zip(expected_1.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_1
		.iter()
		.rev()
		.zip(expected_1.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_2
		.iter()
		.zip(expected_2.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_2
		.iter()
		.rev()
		.zip(expected_2.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));

	let mut list_1: List<i32> = (0..10).collect();
	let list_2 = list_1.split_off(2);
	assert_eq!(list_1.len, 2);
	assert_eq!(list_2.len, 8);
	let expected_1 = vec![0, 1];
	let expected_2 = vec![2, 3, 4, 5, 6, 7, 8, 9];

	list_1
		.iter()
		.zip(expected_1.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_1
		.iter()
		.rev()
		.zip(expected_1.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_2
		.iter()
		.zip(expected_2.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_2
		.iter()
		.rev()
		.zip(expected_2.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));

	let mut list_1: List<i32> = (0..10).collect();
	let list_2 = list_1.split_off(8);
	assert_eq!(list_1.len, 8);
	assert_eq!(list_2.len, 2);
	let expected_1 = vec![0, 1, 2, 3, 4, 5, 6, 7];
	let expected_2 = vec![8, 9];

	list_1
		.iter()
		.zip(expected_1.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_1
		.iter()
		.rev()
		.zip(expected_1.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_2
		.iter()
		.zip(expected_2.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_2
		.iter()
		.rev()
		.zip(expected_2.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn append() {
	let mut list_1: List<i32> = (0..5).collect();
	let mut list_2: List<i32> = (5..10).collect();
	list_1.append(&mut list_2);
	assert!(list_2.is_empty());
	let expected = (0..10).collect::<Vec<_>>();
	list_1
		.iter()
		.zip(expected.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_1
		.iter()
		.rev()
		.zip(expected.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn prepend() {
	let mut list_1: List<i32> = (5..10).collect();
	let mut list_2: List<i32> = (0..5).collect();
	list_1.prepend(&mut list_2);
	assert!(list_2.is_empty());
	let expected = (0..10).collect::<Vec<_>>();
	list_1
		.iter()
		.zip(expected.iter())
		.for_each(|(a, b)| assert_eq!(a, b));
	list_1
		.iter()
		.rev()
		.zip(expected.iter().rev())
		.for_each(|(a, b)| assert_eq!(a, b));
}

#[test]
fn print() {
	let arr = [1, 2, 3, 4, 5];
	let list = arr.iter().copied().collect::<List<_>>();
	let list_str = format!("{:?}", list);
	let arr_str = format!("{:?}", arr);
	assert_eq!(list_str, arr_str);
}
