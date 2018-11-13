use std::io;
use std::io::prelude::*;
use std::ops::AddAssign;
use std::collections::{HashSet, HashMap};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut particles = vec![];
    for (i, line) in lines.enumerate() {
        let particle = parse_line(&line.unwrap());
        particles.push((i, particle));
    }

    // Assume all collisions occur in the first 1000 ticks
    let mut eliminated = HashSet::new();
    for _ in 0..1000 {
        let mut buckets = HashMap::<Coord, Vec<usize>>::new();
        for (i, (p,v,a)) in &mut particles {
            if eliminated.contains(i) { continue; }

            *v += *a;
            *p += *v;
            buckets.entry(*p).or_default().push(*i);
        }

        for (_, bucket) in buckets {
            if bucket.len() > 1 {
                for i in &bucket {
                    eliminated.insert(*i);
                }
            }
        }
    }

    println!("{}", particles.len() - eliminated.len());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}
impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

fn parse_line(mut line: &str) -> (Coord, Coord, Coord) {
    let mut coords = Vec::with_capacity(3);

    for _ in 0..3 {
        let begin = line.bytes().position(|ch| ch == b'<').unwrap();
        let end = line.bytes().position(|ch| ch == b'>').unwrap();

        let range = &line[begin+1 .. end];
        let comma1 = range.bytes().position(|ch| ch == b',').unwrap();
        let comma2 = comma1+1 + range[comma1+1 ..].bytes().position(|ch| ch == b',').unwrap();

        let x = range[..comma1].parse().unwrap();
        let y = range[comma1+1 .. comma2].parse().unwrap();
        let z = range[comma2+1 ..].parse().unwrap();

        coords.push(Coord { x, y, z, });

        line = &line[end+1 ..];
    }

    (coords[0], coords[1], coords[2])
}
