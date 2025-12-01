const MAX: i32 = 100;

fn main() {
    let lines = common::read_lines_from_input();

    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;
    let mut position: i32 = 50;

    for line in lines {
        let (direction, steps_str) = line.split_at(1);

        let steps = steps_str.parse::<i32>().unwrap();
        let multiplier: i32 = match direction {
            "L" => -1,
            "R" => 1,
            _ => panic!("Invalid direction: {}", direction),
        };

        let rotations = steps / MAX;
        let remainder = steps % MAX;

        part_two += rotations;

        let mut new_position = position + remainder * multiplier;

        // Check if we pass through 0 during the remainder rotation (before wrapping)
        if multiplier == 1 {
            // Right: we pass 0 if position + i = MAX for some i in [1, remainder]
            if position < MAX && position + remainder >= MAX {
                part_two += 1;
            }
        } else {
            // Left: we pass 0 if position - i = 0 for some i in [1, remainder]
            if position > 0 && position - remainder <= 0 {
                part_two += 1;
            }
        }

        if new_position >= MAX {
            new_position = new_position - MAX;
        } else if new_position < 0 {
            new_position = MAX - new_position.abs();
        };

        if new_position == 0 {
            part_one += 1;
        }

        println!(
            "{} + {}{} = {}",
            position, direction, steps_str, new_position
        );

        position = new_position;
    }

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}
