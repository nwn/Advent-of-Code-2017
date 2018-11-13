#[macro_use] extern crate text_io;

fn main() {
    let cell: u32 = read!();

    // Distance from centre to cell's ring
    let radius = (((cell as f32).sqrt() + 1_f32) / 2_f32).ceil() as u32 - 1;
    
    // Side length of cell's ring
    let side = 2 * radius + 1;

    // Distance from corners of cell's ring
    let dist_from_corner = (side * side - cell) % (side - 1);
    let dist_from_corner = dist_from_corner.min(side - 1 - dist_from_corner);

    // Manhatten distance from centre to cell
    let dist = 2 * radius - dist_from_corner;

    println!("{}", dist);
}
