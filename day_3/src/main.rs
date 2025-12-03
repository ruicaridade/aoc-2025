use std::time::Instant;

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;

    for _line in lines {
        // TODO: Implement solution
    }

    let elapsed = start.elapsed();
    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
    println!("Time: {:?}", elapsed);
}
