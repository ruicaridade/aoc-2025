use core::f64;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

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

    let mut circuits: Vec<Vec<usize>> = nodes.iter().enumerate().map(|(i, _)| vec![i]).collect();
    let mut node_to_circuit: HashMap<usize, usize> = HashMap::new();
    for (circuit_idx, circuit) in circuits.iter().enumerate() {
        for &node in circuit {
            node_to_circuit.insert(node, circuit_idx);
        }
    }

    let mut part_one = 0;
    let mut part_two = 0;

    for (idx, &(i, j, _)) in pairs.iter().enumerate() {
        if idx == 1000 {
            let mut circuits_buffer = circuits.clone();
            circuits_buffer.sort_by(|a, b| b.len().cmp(&a.len()));
            part_one = circuits_buffer
                .iter()
                .take(3)
                .fold(1, |acc, c| acc * c.len());
        }

        edges[i].insert(j);
        edges[j].insert(i);

        let circuit_i = *node_to_circuit.get(&i).unwrap();
        let circuit_j = *node_to_circuit.get(&j).unwrap();

        if circuit_i == circuit_j {
            continue;
        }

        let populated_circuit_count = circuits.iter().filter(|circuit| circuit.len() > 0).count();
        if populated_circuit_count == 2 {
            part_two = nodes[i].x * nodes[j].x;
            break;
        }

        let mut merged_circuit = circuits[circuit_i].clone();
        merged_circuit.extend(circuits[circuit_j].clone());

        for &node in &merged_circuit {
            node_to_circuit.insert(node, circuit_i);
        }

        circuits[circuit_i] = merged_circuit;
        circuits[circuit_j].clear();
    }

    let elapsed = start.elapsed();

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
    println!("Time: {:?}", elapsed);
}
