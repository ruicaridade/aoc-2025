use std::time::Instant;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
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

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

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

    let (max_x, max_y) = points
        .iter()
        .fold((0, 0), |(max_x, max_y), p| (max_x.max(p.x), max_y.max(p.y)));

    let width = max_x + 3;
    let height = max_y + 3;

    let mut grid: Vec<char> = vec!['.'; width as usize * height as usize];
    for a in points.iter() {
        for b in points.iter() {
            for y in a.y.min(b.y)..=a.y.max(b.y) {
                for x in a.x.min(b.x)..=a.x.max(b.x) {
                    grid[y as usize * width as usize + x as usize] = 'X';
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y as usize * width as usize + x as usize]);
        }
        println!();
    }

    let elapsed = start.elapsed();
    println!("Part 1: {}", largest_area);
    println!("Part 2: {}", 0);
    println!("Time: {:?}", elapsed);
}
