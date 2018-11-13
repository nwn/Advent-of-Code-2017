extern crate regex;

use std::io;
use std::io::prelude::*;
use std::collections::{HashSet, HashMap};
use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    weight: i32,
    total_weight: i32,
    children: Vec<String>,
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut tree: HashMap<String, Node> = HashMap::new();

    let line_regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();
    let child_regex = Regex::new(r"\w+").unwrap();
    for line in lines {
        let line = line.iter().next().unwrap();
        let caps = line_regex.captures(line).unwrap();

        let parent = caps.get(1).unwrap().as_str().to_string();
        let weight = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let children = if let Some(mat) = caps.get(3) {
            child_regex.find_iter(mat.as_str()).map(|mat| mat.as_str().to_string()).collect()
        } else {
            Vec::new()
        };

        tree.insert(parent, Node { weight, total_weight: weight, children });
    }

    let root = {
        let mut left_refs = HashSet::new();
        let mut right_refs = HashSet::new();

        for (name, node) in &tree {
            left_refs.insert(name.clone());
            for child in &node.children {
                right_refs.insert(child.clone());
            }
        }

        left_refs.difference(&right_refs).next().unwrap().clone()
    };

    calculate_weights(&mut tree, &root);

    println!("{}", solve(&tree, &root, 0).unwrap());
}

fn calculate_weights(tree: &mut HashMap<String, Node>, root: &String) {
    let mut node = tree[root].clone();

    node.total_weight = node.weight;
    for child in &node.children {
        calculate_weights(tree, child);
        node.total_weight += tree[child].total_weight;
    }

    tree.insert(root.clone(), node);
}

fn solve(tree: &HashMap<String, Node>, root: &String, diff: i32) -> Option<i32> {
    // Sort children by total weight to find outlier
    let mut weight_buckets = HashMap::<i32, Vec<String>>::new();
    for child in &tree[root].children {
        weight_buckets.entry(tree[child].total_weight).or_default().push(child.clone());
    }

    if weight_buckets.len() >= 2 {
        let mut buckets = weight_buckets.iter();
        let mut odd = buckets.next().unwrap();
        let mut rest = buckets.next().unwrap();
        if odd.1.len() > 1 {
            std::mem::swap(&mut odd, &mut rest);
        }

        if rest.1.len() > 1 {
            // We know the odd one.
            // If recursing gives a solution, use that.
            // Otherwise, the direct child must make up the difference.
            let diff = rest.0 - odd.0;
            let solution = solve(tree, &odd.1[0], diff);
            match solution {
                Some(_) => solution,
                None => {
                    let odd = &tree[&odd.1[0]];
                    Some(odd.weight + diff)
                },
            }
        } else {
            // We don't know which one is odd. Use the diff to decide.
            if odd.0 + diff != *rest.0 {
                std::mem::swap(&mut odd, &mut rest);
            }
            let solution = solve(tree, &odd.1[0], diff);
            match solution {
                Some(_) => solution,
                None => {
                    let odd = &tree[&odd.1[0]];
                    Some(odd.weight + diff)
                },
            }
        }
    } else {
        // No children to be unbalanced, so let caller decide.
        None
    }
}
