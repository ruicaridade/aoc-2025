use core::f64;
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}

impl Node {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Node) -> f64 {
        let dx = (other.x - self.x).abs() as f64;
        let dy = (other.y - self.y).abs() as f64;
        let dz = (other.z - self.z).abs() as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let nodes: Vec<Node> = lines
        .iter()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|part| part.parse::<i64>().unwrap())
                .collect();

            Node::new(coords[0], coords[1], coords[2])
        })
        .collect();

    let distances: Vec<Vec<f64>> = nodes
        .iter()
        .map(|n1| nodes.iter().map(|n2| n1.distance_to(n2)).collect())
        .collect();

    let mut pairs: Vec<(usize, usize, f64)> = distances
        .iter()
        .enumerate()
        .flat_map(|(i, distances)| {
            distances
                .iter()
                .enumerate()
                .map(move |(j, distance)| (i, j, *distance))
        })
        .filter(|(i, j, _)| i != j)
        .step_by(2) // Skip over duplicates
        .collect();

    pairs.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut edges: Vec<HashSet<usize>> = vec![HashSet::new(); nodes.len()];

    for &(i, j, _) in pairs.iter().take(1000) {
        edges[i].insert(j);
        edges[j].insert(i);
    }

    let mut queue: Vec<usize> = Vec::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut visited: HashSet<usize> = HashSet::new();

    for i in 0..nodes.len() {
        if visited.contains(&i) {
            continue;
        }

        let mut circuit = Vec::new();
        queue.push(i);

        while !queue.is_empty() {
            let current = queue.remove(0);
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);
            circuit.push(current);

            for edge in edges[current].iter() {
                let j = *edge;
                if !visited.contains(&j) {
                    queue.push(j);
                }
            }
        }

        circuits.push(circuit);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let circuit_size = circuits.iter().take(3).fold(1, |acc, c| acc * c.len());

    let elapsed = start.elapsed();

    println!("Part 1: {}", circuit_size);
    println!("Part 2: {}", 0);
    println!("Time: {:?}", elapsed);
}
