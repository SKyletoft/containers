use doubly_linked_list::*;
use rand::prelude::*;
use std::collections::LinkedList;

const ADDITIONS: i32 = 5000;
const REMOVALS: i32 = 5000;
const LOOPS: usize = 1000;

fn main() {
	let action = std::env::args()
		.nth(1)
		.map(|r| match r.as_str() {
			"std" => Action::Std,
			"custom" => Action::Custom,
			"insert" => Action::Insert,
			"front" => Action::Front,
			_ => panic!("Please provide a benchmark to run in the arguments"),
		})
		.expect("Please provide a benchmark to run in the arguments");
	match action {
		Std => {
			print!("\nstd");
			for _ in 0..LOOPS {
				standard();
			}
		}
		Custom => {
			print!("\ncustom");
			for _ in 0..LOOPS {
				mine();
			}
		}
		Insert => {
			print!("\ninsert");
			for _ in 0..LOOPS {
				insert();
			}
		}
		Front => {
			print!("\ninsert_front");
			for _ in 0..LOOPS {
				insert_front();
			}
		}
	}
}

enum Action {
	Std,
	Custom,
	Insert,
	Front,
}
use Action::*;

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

fn insert() -> i32 {
	let mut rng = rand::thread_rng();
	let mut list = List::new();
	list.push_front(0);
	for x in 1..ADDITIONS {
		let idx = rng.gen::<usize>() % list.len();
		list.insert(idx, x);
	}
	list.iter().sum()
}

fn insert_front() -> i32 {
	let mut rng = rand::thread_rng();
	let mut list = List::new();
	list.push_front(0);
	for x in 1..ADDITIONS {
		let idx = rng.gen::<usize>() % list.len();
		list.insert_front(idx, x);
	}
	list.iter().sum()
}
