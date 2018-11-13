use std::io;
use std::io::prelude::*;

const HASH_SIZE: usize = 128;
const MARKS: usize = 256;
type Row = [u8; HASH_SIZE/8];
type Grid = [Row; 128];
fn knot_hash(input: &str) -> Row {
    let suffix = [17, 31, 73, 47, 23];
    let lengths = input.bytes()
                       .map(|byte| byte as usize)
                       .chain(suffix.iter().cloned())
                       .collect::<Vec<_>>();

    let mut marks = [0; MARKS];
    for (i, mark) in marks.iter_mut().enumerate() {
        *mark = i as u8;
    }

    let mut skip = 0;
    let mut pos = 0;
    for _ in 0..64 {
        for len in &lengths {
            for i in 0..(len / 2) {
                marks.swap((pos + i) % MARKS, (pos + len - i - 1) % MARKS);
            }

            pos += len + skip;
            skip += 1;
        }
    }

    let mut hash = [0; HASH_SIZE/8];
    for i in 0..256 {
        hash[i / 16] ^= marks[i];
    }

    hash
}

fn count_regions(mut grid: Grid) -> u32 {
    fn test_bit(grid: &Grid, row: usize, col: usize) -> bool {
        (grid[row][col / 8] & (1 << (7 - col % 8))) != 0
    }
    fn clear_bit(grid: &mut Grid, row: usize, col: usize) {
        grid[row][col / 8] &= !(1 << (7 - col % 8));
    }
    fn clear_region(grid: &mut Grid, row: usize, col: usize) {
        let max = HASH_SIZE - 1;
        if test_bit(grid, row, col) {
            clear_bit(grid, row, col);
            if row > 0   { clear_region(grid, row - 1, col); }
            if row < max { clear_region(grid, row + 1, col); }
            if col > 0   { clear_region(grid, row, col - 1); }
            if col < max { clear_region(grid, row, col + 1); }
        }
    }

    let mut count = 0;
    for row in 0..HASH_SIZE {
        for col in 0..HASH_SIZE {
            if test_bit(&grid, row, col) {
                count += 1;
                clear_region(&mut grid, row, col);
            }
        }
    }
    count
}

fn main() {
	let stdin = io::stdin();
	let key = stdin.lock().lines().next().unwrap().unwrap();

    let mut grid = [[0; HASH_SIZE/8]; 128];
	for row in 0..128 {
		let hash_input = format!("{}-{}", key, row);
		grid[row] = knot_hash(&hash_input);
	}

    let count = count_regions(grid);
	println!("{}", count);
}
