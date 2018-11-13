use std::io;
use std::io::prelude::*;

struct Generator {
    val: u32,
    factor: u32,
}
impl Generator {
    fn new(val: u32, factor: u32) -> Self {
        Self { val, factor }
    }
}
impl Iterator for Generator {
    type Item = u32;
    fn next(self: &mut Self) -> Option<Self::Item> {
        let mut val = self.val as u64;
        val *= self.factor as u64;
        val %= 2_147_483_647;

        self.val = val as u32;
        Some(self.val)
    }
}

fn main() {
	let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

	let a_val: u32 = {
        let line = lines.next().unwrap().unwrap();
        let words = line.split_whitespace();
        words.last().unwrap().parse().unwrap()
    };
	let b_val: u32 = {
        let line = lines.next().unwrap().unwrap();
        let words = line.split_whitespace();
        words.last().unwrap().parse().unwrap()
    };

    let mut gen_a = Generator::new(a_val, 16807).filter(|val| val % 4 == 0);
    let mut gen_b = Generator::new(b_val, 48271).filter(|val| val % 8 == 0);

    let modulo = 1 << 16;
    let mut count = 0;
    for _ in 0..5_000_000 {
        let a_val = gen_a.next().unwrap();
        let b_val = gen_b.next().unwrap();

        if a_val % modulo == b_val % modulo {
            count += 1;
        }
    }

    println!("{}", count);
}
