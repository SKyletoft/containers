use doubly_linked_list::*;
use rand::prelude::*;
use std::collections::LinkedList;

const ADDITIONS: i32 = 5000;
const REMOVALS: i32 = 5000;
const LOOPS: usize = 1000;

fn main() {
	if std::env::args().nth(1).map(|s| &s == "std") == Some(true) {
		print!("\nstd");
		for _ in 0..LOOPS {
			standard();
		}
	} else {
		print!("\ncustom");
		for _ in 0..LOOPS {
			mine();
		}
	}
}

fn mine() -> i32 {
	let mut rng = rand::thread_rng();
	let mut list = (1..100).collect::<List<i32>>();
	for x in 0..ADDITIONS {
		list.insert(rng.gen::<usize>() % list.len(), x);
	}
	for _ in 0..REMOVALS {
		list.remove(rng.gen::<usize>() % list.len());
	}
	list.iter().sum()
}

fn standard() -> i32 {
	let mut rng = rand::thread_rng();
	let mut list = (1..100).collect::<LinkedList<i32>>();
	for x in 0..ADDITIONS {
		let mut snd = list.split_off(rng.gen::<usize>() % list.len());
		list.push_back(x);
		list.append(&mut snd);
	}
	for _ in 0..REMOVALS {
		let mut snd = list.split_off(rng.gen::<usize>() % list.len());
		snd.pop_front();
		list.append(&mut snd);
	}
	list.iter().sum()
}
