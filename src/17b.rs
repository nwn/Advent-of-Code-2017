use std::io;
use std::io::prelude::*;

/*
 * Since the buffer would be too long for actual insertion, we
 * simply track the position of 0 and any values inserted directly
 * after it.
 */

fn main() {
	let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let step: usize = line.parse().unwrap();

    let mut pos_0 = 0;
    let mut after_0 = 0;
    let mut pos = 0;
    for i in 1..=50_000_000 {
        let len = i;

        pos += step;
        pos %= len;

        let insert_pos = (pos + 1) % (len + 1);
        if insert_pos <= pos_0 {
            pos_0 += 1;
        } else if insert_pos == pos_0 + 1 {
            after_0 = i;
        }

        pos += 1;
    }

    println!("{}", after_0);
}
