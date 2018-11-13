use std::io;
use std::io::prelude::*;
use std::collections::{HashMap,HashSet};

struct UnionFind {
    pset: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn with_size(n: usize) -> Self {
        Self {
            pset: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn find_set(&mut self, i: usize) -> usize {
        if self.pset[i] != i {
            let set = self.pset[i];
            self.pset[i] = self.find_set(set);
        }
        self.pset[i]
    }
    fn union_set(&mut self, i: usize, j: usize) {
        let set_i = self.find_set(i);
        let set_j = self.find_set(j);
        if set_i != set_j {
            self.pset[set_i] = set_j;
            self.size[set_i] += self.size[set_j];
            self.size[set_j] = self.size[set_i];
        }
    }
    fn set_size(&mut self, i: usize) -> usize {
        let set = self.find_set(i);
        self.size[set]
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut progs: HashMap<usize,HashSet<usize>> = HashMap::new();

    lines.for_each(|line| {
        let line = line.iter().next().unwrap();
        let mut words = line.split_whitespace();

        // Read prog id
        let id: usize = words.next().unwrap().parse().unwrap();
        progs.insert(id.clone(), HashSet::new());
        let neighbours = progs.get_mut(&id).unwrap();

        // Skip arrow
        words.next();

        // Read neighbours
        for word in words {
            let word = word.split_terminator(',').next().unwrap();
            let id: usize = word.parse().unwrap();
            neighbours.insert(id);
        }
    });

    let mut uf = UnionFind::with_size(progs.len());
    for prog in &progs {
        for neighbour in prog.1 {
            uf.union_set(*prog.0, *neighbour);
        }
    }
    println!("{}", uf.set_size(0));
}
