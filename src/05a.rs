use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut instrs: Vec<i32> = lines.map(|line| {
        let line = line.unwrap();
        line.parse().expect("Failed to read integer")
    }).collect();

    let mut pc: i32 = 0;
    let mut steps = 0;
    while pc >= 0 && pc < instrs.len() as i32 {
        let next_pc = pc + instrs[pc as usize];
        instrs[pc as usize] += 1;
        pc = next_pc;
        steps += 1;
    }
    println!("{}", steps);
}
