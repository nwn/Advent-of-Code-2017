extern crate regex;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

// Represent squares as integers
type Sq2 = [[bool; 2]; 2];
type Sq3 = [[bool; 3]; 3];

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut rules2 = HashMap::<Sq2,Sq3>::new();
    let mut rules3 = HashMap::new();

    let re2 = Regex::new(r"^(..)/(..) => (...)/(...)/(...)$").unwrap();
    let re3 = Regex::new(r"^(...)/(...)/(...) => (..)(..)/(..)(..)/(..)(..)/(..)(..)$").unwrap();
    for line in lines {
        let line = line.unwrap();

        if let Some(cap) = re2.captures(&line) {
            let from = make_sq2(cap[1].chars(), cap[2].chars());
            let to = make_sq3(cap[3].chars(), cap[4].chars(), cap[5].chars());

            for sym in symmetries_2(from) {
                rules2.insert(sym, to);
            }
        } else if let Some(cap) = re3.captures(&line) {
            let from = make_sq3(cap[1].chars(), cap[2].chars(), cap[3].chars());
            let to = [
                [
                    make_sq2(cap[4].chars(), cap[6].chars()),
                    make_sq2(cap[5].chars(), cap[7].chars()),
                ],
                [
                    make_sq2(cap[8].chars(), cap[10].chars()),
                    make_sq2(cap[9].chars(), cap[11].chars()),
                ],
            ];

            for sym in symmetries_3(from) {
                rules3.insert(sym, to);
            }
        } else {
            panic!();
        }
    }

    let mut from_grid = vec![
        vec![false, true,  false],
        vec![false, false, true],
        vec![true,  true,  true],
    ];
    let mut to_grid;

    for _ in 0..5 {
        if from_grid.len() % 2 == 0 {
            // Divide into 2x2 squares

            let size = from_grid.len() / 2;
            to_grid = vec![vec![false; 3 * size]; 3 * size];

            for row in 0..size {
                for col in 0..size {
                    let key = [[from_grid[2*row][2*col],   from_grid[2*row][2*col+1]],
                               [from_grid[2*row+1][2*col], from_grid[2*row+1][2*col+1]]];
                    let val = rules2[&key];

                    to_grid[3*row][3*col..(3*(col+1))].copy_from_slice(&val[0]);
                    to_grid[3*row+1][3*col..(3*(col+1))].copy_from_slice(&val[1]);
                    to_grid[3*row+2][3*col..(3*(col+1))].copy_from_slice(&val[2]);
                }
            }
        } else {
            // Divide into 3x3 squares

            let size = from_grid.len() / 3;
            to_grid = vec![vec![false; 4 * size]; 4 * size];

            for row in 0..size {
                for col in 0..size {
                    let key = [[from_grid[3*row][3*col],   from_grid[3*row][3*col+1],   from_grid[3*row][3*col+2]],
                               [from_grid[3*row+1][3*col], from_grid[3*row+1][3*col+1], from_grid[3*row+1][3*col+2]],
                               [from_grid[3*row+2][3*col], from_grid[3*row+2][3*col+1], from_grid[3*row+2][3*col+2]]];
                    let val = rules3[&key];

                    to_grid[4*row][4*col..(4*col+2)].copy_from_slice(&val[0][0][0]);
                    to_grid[4*row+1][4*col..(4*col+2)].copy_from_slice(&val[0][0][1]);
                    to_grid[4*row][(4*col+2)..(4*col+4)].copy_from_slice(&val[0][1][0]);
                    to_grid[4*row+1][(4*col+2)..(4*col+4)].copy_from_slice(&val[0][1][1]);
                    to_grid[4*row+2][4*col..(4*col+2)].copy_from_slice(&val[1][0][0]);
                    to_grid[4*row+3][4*col..(4*col+2)].copy_from_slice(&val[1][0][1]);
                    to_grid[4*row+2][(4*col+2)..(4*col+4)].copy_from_slice(&val[1][1][0]);
                    to_grid[4*row+3][(4*col+2)..(4*col+4)].copy_from_slice(&val[1][1][1]);
                }
            }
        }
        std::mem::swap(&mut from_grid, &mut to_grid);
    }

    let sum = from_grid.into_iter().flatten().filter(|b| *b).count();

    println!("{}", sum);
}

fn make_sq2<It>(mut top: It, mut bot: It) -> Sq2
    where It: Iterator<Item = char> {
    [
        [top.next().unwrap() == '#', top.next().unwrap() == '#'],
        [bot.next().unwrap() == '#', bot.next().unwrap() == '#'],
    ]
}
fn make_sq3<It>(mut top: It, mut mid: It, mut bot: It) -> Sq3
    where It: Iterator<Item = char> {
    [
        [top.next().unwrap() == '#', top.next().unwrap() == '#', top.next().unwrap() == '#'],
        [mid.next().unwrap() == '#', mid.next().unwrap() == '#', mid.next().unwrap() == '#'],
        [bot.next().unwrap() == '#', bot.next().unwrap() == '#', bot.next().unwrap() == '#'],
    ]
}

fn symmetries_2(base: Sq2) -> Symmetry2 {
    Symmetry2 {
        base,
        iter: 0,
    }
}
fn symmetries_3(base: Sq3) -> Symmetry3 {
    Symmetry3 {
        base,
        iter: 0,
    }
}

struct Symmetry2 {
    base: Sq2,
    iter: u8,
}
impl Iterator for Symmetry2 {
    type Item = Sq2;
    fn next(&mut self) -> Option<Self::Item> {
        fn rotate(b: Sq2) -> Sq2 {
            [[b[0][1], b[1][1]], [b[0][0], b[1][0]]]
        }
        fn flip(b: Sq2) -> Sq2 {
            [[b[1][0], b[1][1]], [b[0][0], b[0][1]]]
        }

        if self.iter >= 8 {
            return None;
        }

        let mut res = self.base;
        for _ in 0..(self.iter % 4) {
            res = rotate(res);
        }
        if self.iter / 4 != 0 {
            res = flip(res);
        }

        self.iter += 1;
        Some(res)
    }
}

struct Symmetry3 {
    base: Sq3,
    iter: u8,
}
impl Iterator for Symmetry3 {
    type Item = Sq3;
    fn next(&mut self) -> Option<Self::Item> {
        fn rotate(b: Sq3) -> Sq3 {
            [
                [b[0][2], b[1][2], b[2][2]],
                [b[0][1], b[1][1], b[2][1]],
                [b[0][0], b[1][0], b[2][0]],
            ]
        }
        fn flip(b: Sq3) -> Sq3 {
            [b[2], b[1], b[0]]
        }

        if self.iter >= 8 {
            return None;
        }

        let mut res = self.base;
        for _ in 0..(self.iter % 4) {
            res = rotate(res);
        }
        if self.iter / 4 != 0 {
            res = flip(res);
        }

        self.iter += 1;
        Some(res)
    }
}
