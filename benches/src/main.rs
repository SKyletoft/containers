use doubly_linked_list::*;
use rand::prelude::*;
use std::collections::LinkedList;

fn main() {
	for _ in 0..10000 {
		mine();
	}
}

fn mine() -> i32 {
	let mut rng = rand::thread_rng();
	let mut list = (1..100).collect::<List<i32>>();
	for x in 0..10_000 {
		list.insert(rng.gen::<usize>() % list.len(), x);
	}
	for _ in 0..1000 {
		list.remove(rng.gen::<usize>() % list.len());
	}
	list.iter().sum()
}

fn standard() -> i32 {
	let mut rng = rand::thread_rng();
	let mut list = (1..100).collect::<LinkedList<i32>>();
	for x in 0..10_000 {
		let mut snd = list.split_off(rng.gen::<usize>() % list.len());
		list.push_back(x);
		list.append(&mut snd);
	}
	for _ in 0..1000 {
		let mut snd = list.split_off(rng.gen::<usize>() % list.len());
		snd.pop_front();
		list.append(&mut snd);
	}
	list.iter().sum()
}
