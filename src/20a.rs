use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut particles = vec![];
    for (i, line) in lines.enumerate() {
        let particle = parse_line(&line.unwrap());
        particles.push((i, particle));
    }

    let mut distances = vec![];
    let t = 1000.0; // Approximate asymptotic behaviour
    for (i, (p,v,a)) in &particles {
        let dist_x = 0.5 * a.x * t * t + (v.x + 0.5 * a.x) * t + p.x;
        let dist_y = 0.5 * a.y * t * t + (v.y + 0.5 * a.y) * t + p.y;
        let dist_z = 0.5 * a.z * t * t + (v.z + 0.5 * a.z) * t + p.z;
        let dist = dist_x.abs() + dist_y.abs() + dist_z.abs();
        distances.push((dist, *i));
    }

    distances[..].sort_by(|a,b| a.partial_cmp(b).unwrap());
    println!("{}", distances[0].1);
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
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
