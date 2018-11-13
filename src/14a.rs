use std::io;
use std::io::prelude::*;

const HASH_SIZE: usize = 128;
const MARKS: usize = 256;
fn knot_hash(input: &str) -> [u8; HASH_SIZE/8] {
    let suffix = [17, 31, 73, 47, 23];
    let lengths = input.bytes()
                       .map(|byte| byte as usize)
                       .chain(suffix.iter().cloned())
                       .collect::<Vec<_>>();

    let mut marks = [0; MARKS];
    for (i, mark) in marks.iter_mut().enumerate() {
        *mark = i as u8;
    }

    let mut skip = 0;
    let mut pos = 0;
    for _ in 0..64 {
        for len in &lengths {
            for i in 0..(len / 2) {
                marks.swap((pos + i) % MARKS, (pos + len - i - 1) % MARKS);
            }

            pos += len + skip;
            skip += 1;
        }
    }

    let mut hash = [0; HASH_SIZE/8];
    for i in 0..256 {
        hash[i / 16] ^= marks[i];
    }

    hash
}

fn main() {
	let stdin = io::stdin();
	let key = stdin.lock().lines().next().unwrap().unwrap();

	let mut count = 0;
	for row in 0..128 {
		let hash_input = format!("{}-{}", key, row);
		let hash_output = knot_hash(&hash_input);
		count += hash_output.iter().fold(0, |acc, elem| acc + elem.count_ones());
	}
	println!("{}", count);
}
