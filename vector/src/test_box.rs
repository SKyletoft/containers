use crate::*;

#[test]
fn add_one() {
	let mut vec = Vector::new();
	vec.push(Box::new(1));
	assert_eq!(vec.get(0), Some(&Box::new(1)));
}

#[test]
fn insert() {
	let mut vec = Vector::new();
	vec.push(Box::new(1));
	vec.push(Box::new(2));
	vec.push(Box::new(4));
	vec.push(Box::new(5));
	vec.insert(2, Box::new(3));
	dbg!(&vec);
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
	assert!(vec.get(4).is_none());
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
