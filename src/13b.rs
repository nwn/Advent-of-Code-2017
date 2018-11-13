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
		let depth = words.next().unwrap().parse().unwrap();

		// Retrieve range
		let range = words.next().unwrap().parse().unwrap();

		if layers.len() <= depth {
			layers.resize(depth + 1, 0);
		}
		layers[depth] = range;
	}

    'delay: for delay in 0.. {
        for depth in 0..layers.len() {
            let range = layers[depth];
            let depth = depth as u32;
            if range == 0 {
                continue;
            }

            // Calculate scanner position when packet moves
            let mut scanner = (delay + depth) % (2 * (range - 1));
            if scanner > range - 1 {
                scanner = 2 * (range - 1) - scanner
            }

            // Give up if scanner observes packet move
            if scanner == 0 {
                continue 'delay;
            }
        }

        println!("{}", delay);
        break;
    }
}
