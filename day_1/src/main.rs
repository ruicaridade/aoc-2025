const MAX: i32 = 100;

fn main() {
    let lines = common::read_lines_from_input();

    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;

    let mut current_number: i32 = 50;

    for line in lines {
        let (direction, steps_str) = line.split_at(1);

        let steps = steps_str.parse::<i32>().unwrap();
        let rotations = steps / MAX;
        let remainder = steps % MAX;

        let multiplier: i32 = match direction {
            "L" => -1,
            "R" => 1,
            _ => panic!("Invalid direction: {}", direction),
        };

        let mut new_number = current_number + remainder * multiplier;

        part_two += rotations;
        if (multiplier == 1 && new_number >= MAX) || (multiplier == -1 && new_number < 1) {
            part_two += 1;
        }

        new_number = new_number % MAX;
        if new_number == 0 {
            part_one += 1;
        }

        current_number = new_number;
    }

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}
