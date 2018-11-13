use std::io;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct State {
    instr: [Instr; 2],
}

#[derive(Debug)]
struct Instr {
    write: bool,
    step: i32,
    next: char,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut state = lines.next().unwrap().split_whitespace().nth(3).unwrap().chars().next().unwrap();
    let diag: u32 = lines.next().unwrap().split_whitespace().nth(5).unwrap().parse().unwrap();

    let mut states = HashMap::new();
    while let Some((name, state)) = parse_state(&mut lines) {
        states.insert(name, state);
    }

    let mut tape = HashSet::new();
    let mut cursor = 0;
    for _ in 0..diag {
        let read = if tape.contains(&cursor) { 1 } else { 0 };
        let instr = &states[&state].instr[read];

        // Write to tape
        if instr.write {
            tape.insert(cursor);
        } else {
            tape.remove(&cursor);
        }

        // Move cursor
        cursor += instr.step;

        // Change state
        state = instr.next;
    }

    println!("{}", tape.len());
}

fn parse_state(lines: &mut impl Iterator<Item = String>) -> Option<(char, State)> {
    lines.next()?; // Skip blank line
    let name = lines.next().unwrap().split_whitespace().nth(2).unwrap().chars().next().unwrap();

    lines.next(); // Skip "current value" line
    let write = lines.next().unwrap().split_whitespace().nth(4).unwrap().chars().next().unwrap() == '1';
    let step = if lines.next().unwrap().split_whitespace().nth(6).unwrap() == "right." { 1 } else { -1 };
    let next = lines.next().unwrap().split_whitespace().nth(4).unwrap().chars().next().unwrap();
    let instr1 = Instr { write, step, next };

    lines.next(); // Skip "current value" line
    let write = lines.next().unwrap().split_whitespace().nth(4).unwrap().chars().next().unwrap() == '1';
    let step = if lines.next().unwrap().split_whitespace().nth(6).unwrap() == "right." { 1 } else { -1 };
    let next = lines.next().unwrap().split_whitespace().nth(4).unwrap().chars().next().unwrap();
    let instr2 = Instr { write, step, next };

    Some((name, State { instr: [instr1, instr2] }))
}
