use std::time::Instant;

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut result = 0;

    // bruh...
    // there's no way this is the intended way surely
    
    for line in lines {
        if line.contains('x') {
            let (dimensions, boxes) = line.split_once(':').unwrap();
            let (width, height) = dimensions
                .split_once('x')
                .map(|d| (d.0.parse::<i32>().unwrap(), d.1.parse::<i32>().unwrap()))
                .unwrap();
            let area = (width / 3) * (height / 3);

            let total_boxes = boxes
                .split_whitespace()
                .map(|b| b.trim().parse::<i32>().unwrap())
                .sum::<i32>();

            if total_boxes <= area {
                result += 1;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Part 1: {}", result);
    println!("Time: {:.2?}", elapsed);
}
