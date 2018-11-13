use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let step: usize = line.parse().unwrap();

    let mut pos = 0;
    let mut buffer = vec![0];
    for i in 1..=2017 {
        let len = buffer.len();

        pos += step;
        pos %= len;
        buffer.insert((pos + 1) % (len + 1), i);
        pos += 1;
    }

    println!("{}", buffer[(pos + 1) % buffer.len()]);
}
