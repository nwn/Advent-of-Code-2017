use std::io;
use std::io::prelude::*;
use std::iter::{Iterator,Peekable};

/// Matching a group:
/// match {
///     match (group|garbage)(,(group|garbage))*
/// match }
///
/// Matching garbage:
/// match <
///     match ((!.)|[~>])*
/// match >

fn garbage_count<I>(chars: &mut Peekable<I>) -> u32
    where I: Iterator<Item=char>
{
    let mut sum: u32 = 0;
    // Loop through a comma separated list of groups and garbage
    while chars.peek().is_some() {
        match *chars.peek().unwrap() {
            '{' => {
                // Starting inner group
                chars.next(); // Consume '{'
                sum += garbage_count(chars);
            },
            '}' => {
                // Ending outer group
                chars.next(); // Consume '}'
                break;
            },
            '<' => {
                // Starting garbage
                chars.next(); // Consume '<'
                // Loop through garbage characters until terminator is found
                loop {
                    match chars.next().unwrap() {
                        '!' => {
                            chars.next(); // Consume any other char
                        },
                        '>' => break,
                        _ => sum += 1,
                    }
                }
            },
            ',' => {
                // Between groups or garbage patches
                chars.next(); // Consume ','
            },
            _ => {
                chars.next();
            },
        }
    }
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut chars = stdin.lock().bytes()
        .map(|res| res.expect("Failed to read input") as char)
        .peekable();
    println!("{}", garbage_count(&mut chars));
}
