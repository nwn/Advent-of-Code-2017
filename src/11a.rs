use std::io;
use std::io::prelude::*;

/// Use axial coordinates and hex distance:
/// https://www.redblobgames.com/grids/hexagons/#distances

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().expect("Failed to read input");
    let steps = input.split(',');

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for step in steps {
        match step {
            "n" => {
                y += 1;
            },
            "s" => {
                y -= 1;
            },
            "nw" => {
                x -= 1;
                y += 1;
            },
            "ne" => {
                x += 1;
            },
            "sw" => {
                x -= 1;
            },
            "se" => {
                x += 1;
                y -= 1;
            },
            _ => panic!("Invalid instruction: {}", step),
        }
    }
    let dist = (x.abs() + (x + y).abs() + y.abs()) / 2;
    println!("{}", dist);
}
