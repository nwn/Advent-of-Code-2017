use std::io;
use std::io::prelude::*;

fn solve(start: u16, conns: &mut [(u16,u16)]) -> (u16,u16) {
    let mut best = (0,0);
    for i in 0..conns.len() {
        let other: u16;
        if start == conns[i].0 {
            other = conns[i].1;
        } else if start == conns[i].1 {
            other = conns[i].0;
        } else {
            continue;
        }

        conns.swap(0, i);
        let mut score = solve(other, &mut conns[1..]);
        score.0 += 1; // length
        score.1 += conns[0].0 + conns[0].1; // strength
        conns.swap(0, i);

        if score.0 > best.0 {
            best = score;
        } else if score.0 == best.0 && score.1 > best.1 {
            best = score;
        }
    }
    best
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut conns = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split('/');

        let a: u16 = parts.next().unwrap().parse().unwrap();
        let b: u16 = parts.next().unwrap().parse().unwrap();

        conns.push((a,b))
    }

    println!("{}", solve(0, &mut conns).1);
}
