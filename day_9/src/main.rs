use std::time::Instant;

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

fn matrix_insert(matrix: &mut [bool; 100000 * 100000], width: usize, point: &(i32, i32)) {
    matrix[(point.1 as usize) * width + (point.0 as usize)] = true;
}

fn is_inside(matrix: &[bool; 100000 * 100000], width: usize, point: &(i32, i32)) -> bool {
    matrix[(point.1 as usize) * width + (point.0 as usize)]
}

fn solve_part_two(lines: &Vec<String>) {
    //
    // This is the most horrendously slow, inefficient and dumb "algorithm" on planet Earth, run at your own peril.
    //
    // ¯\_(ツ)_/¯
    //

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

    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;

    for i in 0..points.len() {
        let p1 = &points[i];

        min_x = min_x.min(p1.0);
        max_x = max_x.max(p1.0);
        min_y = min_y.min(p1.1);
        max_y = max_y.max(p1.1);
    }

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut matrix: [bool; 100000 * 100000] = [false; 100000 * 100000];

    println!("Finding polygon edges...");

    for i in 1..points.len() {
        let p1 = &points[i - 1];
        let p2 = &points[i];

        for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
            for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                matrix_insert(&mut matrix, width, &(x, y));
            }
        }
    }

    println!("Finding polygon inner points...");

    let mut previous_point = (0, 0);

    for y in min_y..=max_y {
        let mut inside = false;

        for x in min_x..=max_x {
            let point = (x, y);

            if is_inside(&matrix, width, &point) {
                if is_inside(&matrix, width, &previous_point) {
                    inside = true;
                } else {
                    inside = !inside;
                }
            }

            if inside {
                matrix_insert(&mut matrix, width, &point);
            }

            previous_point = point;
        }
    }

    println!("Testing all rectangles against points map...");

    let mut largest_area = 0;

    for i in 0..points.len() {
        for j in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            let mut inside = true;
            'outer: for x in p1.0.min(p2.0)..=p1.0.max(p2.0) {
                for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
                    let point = (x, y);

                    if !is_inside(&matrix, width, &point) {
                        inside = false;
                        break 'outer;
                    }
                }
            }

            if inside {
                largest_area = largest_area.max(area_between(p1, p2));
            }
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
