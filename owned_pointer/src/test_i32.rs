use crate::OwnedPointer;

#[test]
fn deref() {
	let x = OwnedPointer::new(5);
	assert_eq!(5, *x);
}

#[test]
fn borrow_coersion() {
	//Not really a test, but should cause a compilation failure whenever the behaviour fails
	let x = OwnedPointer::new(5);
	let y: &i32 = &x;
	dbg!(y);
}

#[test]
fn borrow_coersion_mut() {
	//Not really a test, but should cause a compilation failure whenever the behaviour fails
	let mut x = OwnedPointer::new(5);
	let _: &mut i32 = &mut x;
}
