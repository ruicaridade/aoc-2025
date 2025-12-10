use std::time::Instant;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (other.x == self.x) && (other.y == self.y)
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn area_between(&self, other: &Point) -> i64 {
        let dx = (other.x - self.x).abs() + 1;
        let dy = (other.y - self.y).abs() + 1;
        dx * dy
    }
}

struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        Self { points: points }
    }

    fn contains_point(&self, point: &Point) -> bool {
        if self.points.is_empty() {
            return false;
        }

        let mut inside = false;
        let n = self.points.len();

        for i in 0..self.points.len() {
            let j = (i + 1) % n;

            let e1 = &self.points[i];
            let e2 = &self.points[j];

            for k in point.
        }

        inside
    }

    fn contains_polygon(&self, polygon: &Polygon) -> bool {
        if self.points.is_empty() {
            return false;
        }

        false
    }
}

fn solve_part_one(lines: &Vec<String>) {
    let start = Instant::now();

    let points: Vec<Point> = lines
        .iter()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            Point::new(parts[0], parts[1])
        })
        .collect();

    let areas: Vec<Vec<i64>> = points
        .iter()
        .map(|p1| points.iter().map(|p2| p1.area_between(p2)).collect())
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

    let points: Vec<Point> = lines
        .iter()
        .map(|line| {
            let parts = line
                .split(",")
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            Point::new(parts[0], parts[1])
        })
        .collect();

    let polygon = Polygon::new(points);
    let mut largest_area = 0;

    for i in 0..polygon.points.len() {
        for j in 0..polygon.points.len() {
            if i == j {
                continue;
            }

            let p1 = &polygon.points[i];
            let p2 = &polygon.points[j];

            let c1 = Point { x: p1.x, y: p1.y };
            let c2 = Point { x: p1.y, y: p2.x };
            let c1 = Point { x: p2.x, y: p2.y };
            let c4 = Point { x: p2.y, y: p1.x };

            let rect = Polygon::new(vec![c1, c2, c2, c4]);
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
