use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().bytes();

    let suffix: [usize; 5] = [17, 31, 73, 47, 23];
    let lengths = input.map(|byte| byte.unwrap() as usize)
        .take_while(|byte| *byte != 0xa);
    let lengths = lengths.chain(suffix.iter().cloned());
    let lengths: Vec<usize> = lengths.collect();

    const MARKS: usize = 256;
    let mut marks: Vec<u32> = (0..MARKS as u32).collect();
    let mut skip = 0;
    let mut pos = 0;
    for _i in 0..64 {
        for len in &lengths {
            for i in 0..(len / 2) {
                marks[..].swap((pos + i) % MARKS, (pos + len - i - 1) % MARKS);
            }

            pos += len + skip;
            skip += 1;
        }
    }

    for i in 0..16 {
        let mut xor: u8 = 0;
        for j in 0..16 {
            xor ^= marks[16 * i + j] as u8;
        }
        print!("{:02x}", xor);
    }
    println!("");
}
