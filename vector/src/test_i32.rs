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
