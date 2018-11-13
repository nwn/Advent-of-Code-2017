#[macro_use] extern crate text_io;

use std::collections::HashMap;

fn to_coords(cell: i32) -> (i32,i32) {
    // Distance from centre to cell's ring
    let radius = (((cell as f32).sqrt() + 1_f32) / 2_f32).ceil() as i32 -1;

    // Exit early to avoid division by zero special case
    if radius == 0 {
        return (0,0);
    }

    //Side length of cell's ring
    let side_len = 2 * radius + 1;

    // Which side (right, top, left, bottom) of ring is cell on?
    // How far along this side is cell?
    let side = cell - (side_len - 2) * (side_len - 2) - 1;
    let side_pos = side % (side_len - 1);
    let side = side / (side_len - 1);

    return match side {
        0 => (radius, -radius + side_pos + 1),
        1 => (radius - side_pos - 1, radius),
        2 => (-radius, radius - side_pos - 1),
        3 => (-radius + side_pos + 1, -radius),
        _ => panic!(),
    }
}

fn main() {
    let input: i32 = read!();

    // Mapping from coord to cell value
    let mut values: HashMap<(i32,i32),i32> = HashMap::new();

    // Insert initial value
    let mut cell = 1;
    let mut cell_value = 1;
    values.insert(to_coords(cell), cell_value);

    while cell_value <= input {
        cell += 1;
        let coords = to_coords(cell);

        // Calculate and store next cell value
        cell_value = 0;
        for x in -1..2 {
            for y in -1..2 {
                cell_value += values.get(&(coords.0 + x, coords.1 + y)).unwrap_or(&0);
            }
        }
        values.insert(coords, cell_value);
    }

    println!("{}", cell_value);
}
