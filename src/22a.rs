use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut infected = HashSet::new();

    let mut cur_pos = (0, 0);
    let mut cur_dir = Up;
    for (y, line) in lines.enumerate() {
        let line = line.unwrap();
        for (x, ch) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            if ch == '#' {
                infected.insert((x,y));
            }
            cur_pos = (x/2, y/2);
        }
    }

    let mut count = 0;
    for _ in 0..10_000 {
        if infected.contains(&cur_pos) {
            cur_dir.turn_right();
            infected.remove(&cur_pos);
        } else {
            cur_dir.turn_left();
            infected.insert(cur_pos);
            count += 1;
        }
        cur_pos = cur_dir.advance(cur_pos);
    }

    println!("{}", count);
}

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
    fn advance(&self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Up => (pos.0, pos.1 - 1),
            Left => (pos.0 - 1, pos.1),
            Down => (pos.0, pos.1 + 1),
            Right => (pos.0 + 1, pos.1),
        }
    }
}
