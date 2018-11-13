use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut grid: Vec<Vec<u8>> = vec![];
    for line in lines {
        grid.push(line.unwrap().bytes().collect());
    }

    let starting_col = grid[0].iter().position(|ch| *ch == b'|').unwrap();

    let mut count = 0;
    let mut pos = (0, starting_col);
    let mut dir = (1, 0);
    loop {
        let ch = grid[pos.0][pos.1];
        match ch {
            b'+' => {
                match dir {
                    (_, 0) => {
                        // Check left/right
                        if grid[pos.0][pos.1 - 1] != b' ' {
                            dir = (0, -1)
                        } else {
                            dir = (0, 1);
                        }
                    },
                    (0, _) => {
                        // Check up/down
                        if grid[pos.0 - 1][pos.1] != b' ' {
                            dir = (-1, 0)
                        } else {
                            dir = (1, 0);
                        }
                    },
                    _ => panic!(),
                }
            },
            b' ' => {
                break;
            },
            _ => (),
        }
        pos.0 = (pos.0 as i32 + dir.0) as usize;
        pos.1 = (pos.1 as i32 + dir.1) as usize;

        count += 1;
    }

    println!("{}", count);
}
