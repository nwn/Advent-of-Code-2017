use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.as_bytes();
    let input = &input[..input.len()-1];

    // Create two iterators: one normal, the other rotated forward
    let iter0 = input.iter();
    let iter1 = (&input[input.len()/2..]).iter().chain((&input[..input.len()/2]).iter());

    // Zip and sum if equal
    let sum: u32 = iter0.zip(iter1)
                   .filter_map(|(cur,next)|
        if cur == next {
            Some((cur - '0' as u8) as u32)
        } else {
            None
        }
    ).sum();
    println!("{}", sum);
}
