use std::{
    collections::{HashMap, HashSet},
    time::Instant,
    vec,
};

fn area_between(a: &(i32, i32), b: &(i32, i32)) -> i64 {
    let dx = (b.0 - a.0).abs() + 1;
    let dy = (b.1 - a.1).abs() + 1;
    dx as i64 * dy as i64
}

fn solve_part_one(lines: &Vec<String>) {
    let start = Instant::now();

    let points: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (parts[0], parts[1])
        })
        .collect();

    let areas: Vec<Vec<i64>> = points
        .iter()
        .map(|p1| points.iter().map(|p2| area_between(p1, p2)).collect())
        .collect();

    let largest_area = areas
        .iter()
        .map(|distances| distances.iter().max().unwrap())
        .max_by(|a, b| a.cmp(b))
        .unwrap();

    let elapsed = start.elapsed();
    println!("\nPart 1: {}", largest_area);
    println!("Time: {:?}", elapsed);
}

fn solve_part_two(lines: &Vec<String>) {
    let start = Instant::now();

    let mut points: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (parts[0], parts[1])
        })
        .collect();

    points.push(points[0]);

    println!("Finding polygon edges...");

    let mut edges: HashSet<(i32, i32)> = HashSet::new();
    let mut ranges_by_row: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;

    for i in 1..points.len() {
        let p1 = &points[i - 1];
        let p2 = &points[i];

        min_x = min_x.min(p1.0.min(p2.0));
        max_x = max_x.max(p1.0.max(p2.0));
        min_y = min_y.min(p1.1.min(p2.1));
        max_y = max_y.max(p1.1.max(p2.1));

        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                edges.insert((x, y));
            }
        }

        if p1.1 == p2.1 {
            ranges_by_row
                .entry(p1.1)
                .or_insert_with(Vec::new)
                .push((p1.0.min(p2.0), p2.0.max(p1.0)));
        }
    }

    println!("Scanning for horizontal ranges...");
    println!("X: {} to {} ({})", min_x, max_x, max_x - min_x);
    println!("Y: {} to {} ({})", min_y, max_y, max_y - min_y);

    // Track all points inside the polygon by keeping track of ranges rather than individual coordinates.
    for y in min_y..=max_y {
        let mut start_x = 0;

        let mut is_inside = false;

        for x in (min_x - 1)..max_x {
            let curr_point = (x, y);
            let next_point = (x + 1, y);

            let curr_collision = !!edges.contains(&curr_point);
            let next_collision = !!edges.contains(&next_point);

            // We're about to collide with an edge.
            // Depending on `is_inside`, this is either the start or the end of a range.
            if !curr_collision && next_collision {
                if is_inside {
                    ranges_by_row
                        .entry(y)
                        .or_insert_with(Vec::new)
                        .push((start_x, next_point.0));
                } else {
                    start_x = next_point.0;
                }
            }

            // We're about to leave an edge, so we need to flip the `is_inside` state.
            if curr_collision && !next_collision {
                is_inside = !is_inside;
            }
        }
    }

    println!("Finding the largest valid area...");

    let is_inside = |p: &(i32, i32)| {
        if let Some(ranges) = ranges_by_row.get(&p.1) {
            for (x1, x2) in ranges {
                if p.0 >= *x1 && p.0 <= *x2 {
                    return true;
                }
            }
        }

        return false;
    };

    let mut largest_area = 0;

    for i in 0..points.len() {
        'scan: for j in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            let x1 = p1.0.min(p2.0);
            let x2 = p1.0.max(p2.0);
            let y1 = p1.1.min(p2.1);
            let y2 = p1.1.max(p2.1);

            for x in x1..=x2 {
                if !is_inside(&(x, p1.1)) || !is_inside(&(x, p2.1)) {
                    continue 'scan;
                }
            }

            for y in y1..=y2 {
                if !is_inside(&(p1.0, y)) || !is_inside(&(p2.0, y)) {
                    continue 'scan;
                }
            }

            largest_area = largest_area.max(area_between(p1, p2));
        }
    }

    let elapsed = start.elapsed();
    println!("\nPart 2: {}", largest_area);
    println!("Time: {:?}", elapsed);
}

fn main() {
    let lines = common::read_lines_from_input();

    solve_part_one(&lines);
    solve_part_two(&lines);
}
