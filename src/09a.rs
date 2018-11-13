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

fn group_score<I>(chars: &mut Peekable<I>, outer_score: u32) -> u32
    where I: Iterator<Item=char>
{
    let mut sum: u32 = 0;
    // Loop through a comma separated list of groups and garbage
    while chars.peek().is_some() {
        match *chars.peek().unwrap() {
            '{' => {
                // Starting inner group, add inner groups' scores
                chars.next(); // Consume '{'
                sum += group_score(chars, outer_score + 1);
            },
            '}' => {
                // Ending outer group, add outer group's score
                chars.next(); // Consume '}'
                sum += outer_score;
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
                        _ => (),
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
    println!("{}", group_score(&mut chars, 0));
}
