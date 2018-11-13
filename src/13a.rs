use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines();

	let mut layers: Vec<u32> = Vec::new();

	for line in lines {
		let line = line.unwrap();
        if line.is_empty() {
            break;
        }
		let mut words = line.split(": ");

		// Retrieve depth
		let depth = words.next().unwrap().parse().expect("Expected depth");

		// Retrieve range
		let range = words.next().unwrap().parse().expect("Expected range");

		if layers.len() <= depth {
			layers.resize(depth + 1, 0);
		}
		layers[depth] = range;
	}

	let mut sum = 0;
	for depth in 0..layers.len() {
		let range = layers[depth];
        let depth = depth as u32;
		if range == 0 {
			continue;
		}

		// Calculate scanner position when packet moves
		let mut scanner = depth % (2 * (range - 1));
		if scanner > range - 1 {
			scanner = 2 * (range - 1) - scanner
		}

		// Count layer if scanner observes packet move
		if scanner == 0 {
			sum += depth * range;
		}
	}

	println!("{}", sum);
}
