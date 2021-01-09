use crate::*;

#[test]
fn add_one() {
	let mut vec = Vector::new();
	vec.push(Box::new(1));
	assert_eq!(*vec[0], 1);
}

#[test]
fn insert() {
	let mut vec = Vector::new();
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	vec.insert(2, Box::new(3));
	assert_eq!(*vec[0], 1);
	assert_eq!(*vec[1], 2);
	assert_eq!(*vec[2], 3);
	assert_eq!(*vec[3], 4);
	assert_eq!(*vec[4], 5);
}

#[test]
fn remove() {
	let mut vec = Vector::new();
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	vec.remove(3);
	assert_eq!(*vec[0], 1);
	assert_eq!(*vec[1], 2);
	assert_eq!(*vec[2], 3);
	assert_eq!(*vec[3], 5);
	assert_eq!(vec.get(4), None);
}

#[test]
fn as_slice() {
	let mut vec = Vector::new();
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	assert_eq!(
		vec.as_slice(),
		&[
			Box::new(1),
			Box::new(2),
			Box::new(3),
			Box::new(4),
			Box::new(5)
		]
	);
}

#[test]
fn iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	let std_vec = vec![
		Box::new(1),
		Box::new(2),
		Box::new(3),
		Box::new(4),
		Box::new(5),
	];
	let mut std_iter = std_vec.into_iter();
	let mut iter = vec.into_iter();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn back_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	let std_vec = vec![
		Box::new(1),
		Box::new(2),
		Box::new(3),
		Box::new(4),
		Box::new(5),
	];
	let mut std_iter = std_vec.into_iter().rev();
	let mut iter = vec.into_iter().rev();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn both_iterator() {
	let mut vec = Vector::with_capacity(3);
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	let mut iter = vec.into_iter();
	assert_eq!(iter.next(), Some(Box::new(1)));
	assert_eq!(iter.next_back(), Some(Box::new(3)));
	assert_eq!(iter.next(), Some(Box::new(2)));
	assert_eq!(iter.next_back(), None);
	assert_eq!(iter.next(), None);
}

#[test]
fn borrowed_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	let std_vec = vec![
		Box::new(1),
		Box::new(2),
		Box::new(3),
		Box::new(4),
		Box::new(5),
	];
	let mut std_iter = std_vec.iter();
	let mut iter = vec.iter();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn borrowed_back_iterator() {
	let mut vec = Vector::with_capacity(5);
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	let std_vec = vec![
		Box::new(1),
		Box::new(2),
		Box::new(3),
		Box::new(4),
		Box::new(5),
	];
	let mut std_iter = std_vec.iter().rev();
	let mut iter = vec.iter().rev();
	for _ in 0..6 {
		assert_eq!(std_iter.next(), iter.next());
	}
}

#[test]
fn borrowed_both_iterator() {
	let mut vec = Vector::with_capacity(3);
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(3));
	let mut iter = vec.iter();
	assert_eq!(iter.next(), Some(&Box::new(1)));
	assert_eq!(iter.next_back(), Some(&Box::new(3)));
	assert_eq!(iter.next(), Some(&Box::new(2)));
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
			vec.push(Box::new(ToDrop { b: i }));
		}
		for _ in vec.iter() {}
		for _ in vec.into_iter() {}
	}
	assert_eq!(unsafe { SUM }, 10);
}
