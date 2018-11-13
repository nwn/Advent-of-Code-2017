use std::io;
use std::io::prelude::*;

/// Use axial coordinates and hex distance:
/// https://www.redblobgames.com/grids/hexagons/#distances

fn hex_distance(x: i32, y: i32) -> i32 {
    (x.abs() + (x + y).abs() + y.abs()) / 2
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().expect("Failed to read input");
    let steps = input.split(',');

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_dist = 0;
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
        max_dist = max_dist.max(hex_distance(x, y));
    }
    println!("{}", max_dist);
}
