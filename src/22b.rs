use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut infected = HashMap::new();

    let mut cur_pos = (0, 0);
    let mut cur_dir = Up;
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        for (x, ch) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            if ch == '#' {
                infected.insert((x,y), Infected);
            }
            cur_pos = (x/2, y/2);
        }
    }

    let mut count = 0;
    for _ in 0..10_000_000 {
        match infected.get(&cur_pos) {
            Some(Weakened) => {
                infected.insert(cur_pos, Infected);
                count += 1;
            },
            Some(Infected) => {
                cur_dir.turn_right();
                infected.insert(cur_pos, Flagged);
            },
            Some(Flagged) => {
                cur_dir.turn_around();
                infected.remove(&cur_pos);
            },
            None => {
                cur_dir.turn_left();
                infected.insert(cur_pos, Weakened);
            },
        };
        cur_pos = cur_dir.advance(cur_pos);
    }

    println!("{}", count);
}

#[derive(Copy, Clone, Debug)]
enum State {
    Weakened,
    Infected,
    Flagged,
}
use State::*;

#[derive(Copy, Clone, Debug)]
enum Dir {
    Up, Left, Down, Right,
}
use Dir::*;
impl Dir {
    fn turn_left(&mut self) {
        *self = match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
    fn turn_right(&mut self) {
        *self = match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }
    fn turn_around(&mut self) {
        *self = match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }
    fn advance(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Up => (pos.0, pos.1 - 1),
            Left => (pos.0 - 1, pos.1),
            Down => (pos.0, pos.1 + 1),
            Right => (pos.0 + 1, pos.1),
        }
    }
}
