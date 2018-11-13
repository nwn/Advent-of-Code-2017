extern crate regex;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

// Represent squares as integers
type Sq2 = u8;
type Sq3 = u16;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut rules2 = HashMap::new();
    let mut rules3 = HashMap::new();

    let re2 = Regex::new(r"^(..)/(..) => (...)/(...)/(...)$").unwrap();
    let re3 = Regex::new(r"^(...)/(...)/(...) => (..)(..)/(..)(..)/(..)(..)/(..)(..)$").unwrap();
    for line in lines {
        let line = line.unwrap();

        if let Some(cap) = re2.captures(&line) {
            let from = make_sq2(cap[1].chars(), cap[2].chars());
            let to = make_sq3(cap[3].chars(), cap[4].chars(), cap[5].chars());
            eprintln!("RULE:");
            eprintln!("\tFROM:\n (original)"); print2(&from); eprintln!("");
            for sym in symmetries_2(from) {
                print2(&sym); eprintln!("");
                rules2.insert(sym, to);
            }
            eprintln!("\tTO:"); print3(&to);
            eprintln!("");
        } else if let Some(cap) = re3.captures(&line) {
            let from = make_sq3(cap[1].chars(), cap[2].chars(), cap[3].chars());
            let to = [
                make_sq2(cap[4].chars(), cap[6].chars()),
                make_sq2(cap[5].chars(), cap[7].chars()),
                make_sq2(cap[8].chars(), cap[10].chars()),
                make_sq2(cap[9].chars(), cap[11].chars()),
            ];
            eprintln!("RULE:");
            eprintln!("\tFROM:\n (original)"); print3(&from); eprintln!("");
            for sym in symmetries_3(from) {
                print3(&sym); eprintln!("");
                rules3.insert(sym, to);
            }
            eprintln!("\tTO:"); print2(&to[0]); eprintln!(""); print2(&to[1]); eprintln!(""); print2(&to[2]); eprintln!(""); print2(&to[3]);
            eprintln!("");
        } else {
            panic!();
        }
    }
            eprintln!("");
            eprintln!("");

    let mut grid_2 = vec![];
    let mut grid_3 = vec![ 0b010_001_111 ];
    print_grid3(&grid_3);
    eprintln!("");
    for i in 0..5 {
        eprintln!("Iter {}:", i);
        if i % 2 == 0 {
            for sq in grid_3.drain(..) {
                //print3(&sq);
                //eprintln!("Becomes");
                eprintln!("Replacing"); print3(&sq); eprintln!("with:"); print_grid2(rules3[&sq].iter().map(|c| c.clone()).collect::<Vec<_>>().as_slice());
                for sq in rules3[&sq].iter() {
                    grid_2.push(*sq);
                }
                eprintln!("");
            }
            print_grid2(&grid_2);
        } else {
            for sq in grid_2.drain(..) {
                //print2(&sq);
                //eprintln!("Becomes");
                //eprint!("Replacing {} with: ", sq);
                //eprintln!("{:?}", rules2[&sq]);
                grid_3.push(rules2[&sq]);
                //print3(&rules2[&sq]);
                //eprintln!("");
            }
            print_grid3(&grid_3);
        }
        eprintln!("");
    }

    let mut sum = 0;
    for sq in grid_2 {
        sum += sq.count_ones();
    }
    for sq in grid_3 {
        sum += sq.count_ones();
    }

    println!("Ones: {}", sum);
}

fn print2(sq: &Sq2) {
    let vec = [*sq];
    print_grid2(&vec);
}

fn print3(sq: &Sq3) {
    let vec = [*sq];
    print_grid3(&vec);
}

fn print_grid2(vec: &[Sq2]) {
    fn to_ch(b: Sq2) -> char { if b != 0 { '#' } else { '.' } }
    let mut size = 0;
    while size * size < vec.len() { size += 1; }
    for i in 0..size {
        for j in 0..size {
            eprint!("{}{}", to_ch(vec[i * size + j] & 0b1000), to_ch(vec[i * size + j] & 0b0100));
            if j + 1 != size { eprint!("|"); }
        }
        eprintln!("");
        for j in 0..size {
            eprint!("{}{}", to_ch(vec[i * size + j] & 0b0010), to_ch(vec[i * size + j] & 0b0001));
            if j + 1 != size { eprint!("|"); }
        }
        eprintln!("");
        if i + 1 != size {
            eprint!("{}", "--+".to_string().repeat(size - 1));
            eprintln!("--");
        }
    }
}

fn print_grid3(vec: &[Sq3]) {
    fn to_ch(b: Sq3) -> char { if b != 0 { '#' } else { '.' } }
    let mut size = 0;
    while size * size < vec.len() { size += 1; }
    for i in 0..size {
        for j in 0..size {
            eprint!("{}{}{}",
                    to_ch(vec[i * size + j] & 0b100_000_000),
                    to_ch(vec[i * size + j] & 0b010_000_000),
                    to_ch(vec[i * size + j] & 0b001_000_000));
            if j + 1 != size { eprint!("|"); }
        }
        eprintln!("");
        for j in 0..size {
            eprint!("{}{}{}",
                    to_ch(vec[i * size + j] & 0b000_100_000),
                    to_ch(vec[i * size + j] & 0b000_010_000),
                    to_ch(vec[i * size + j] & 0b000_001_000));
            if j + 1 != size { eprint!("|"); }
        }
        eprintln!("");
        for j in 0..size {
            eprint!("{}{}{}",
                    to_ch(vec[i * size + j] & 0b000_000_100),
                    to_ch(vec[i * size + j] & 0b000_000_010),
                    to_ch(vec[i * size + j] & 0b000_000_001));
            if j + 1 != size { eprint!("|"); }
        }
        eprintln!("");
        if i + 1 != size {
            eprint!("{}", "---+".to_string().repeat(size - 1));
            eprintln!("---");
        }
    }
}

fn make_sq2<It>(mut top: It, mut bot: It) -> Sq2
    where It: Iterator<Item = char> {
    u8::from(top.next().unwrap() == '#') << 3 |
    u8::from(top.next().unwrap() == '#') << 2 |
    u8::from(bot.next().unwrap() == '#') << 1 |
    u8::from(bot.next().unwrap() == '#')
}
fn make_sq3<It>(mut top: It, mut mid: It, mut bot: It) -> Sq3
    where It: Iterator<Item = char> {
    u16::from(top.next().unwrap() == '#') << 8 |
    u16::from(top.next().unwrap() == '#') << 7 |
    u16::from(top.next().unwrap() == '#') << 6 |
    u16::from(mid.next().unwrap() == '#') << 5 |
    u16::from(mid.next().unwrap() == '#') << 4 |
    u16::from(mid.next().unwrap() == '#') << 3 |
    u16::from(bot.next().unwrap() == '#') << 2 |
    u16::from(bot.next().unwrap() == '#') << 1 |
    u16::from(bot.next().unwrap() == '#')
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
        fn rotate(b: u8) -> u8 {
            ((b & 0b1000) >> 2) |
            ((b & 0b0100) << 1) |
            ((b & 0b0010) >> 1) |
            ((b & 0b0001) << 2)
        }
        fn flip(b: u8) -> u8 {
            ((b & 0b1100) >> 2) |
            ((b & 0b0011) << 2)
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
        fn rotate(b: u16) -> u16 {
            ((b & 0b100_000_000) >> 6) |
            ((b & 0b010_000_000) >> 2) |
            ((b & 0b001_000_000) << 2) |
            ((b & 0b000_100_000) >> 4) |
            ((b & 0b000_010_000)     ) |
            ((b & 0b000_001_000) << 4) |
            ((b & 0b000_000_100) >> 2) |
            ((b & 0b000_000_010) << 2) |
            ((b & 0b000_000_001) << 6)
        }
        fn flip(b: u16) -> u16 {
            ((b & 0b111_000_000) >> 6) |
            ((b & 0b000_111_000)     ) |
            ((b & 0b000_000_111) << 6)
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
