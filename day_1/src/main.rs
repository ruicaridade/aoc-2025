const MAX: i32 = 99;

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

        let mut p = position;
        for _ in 0..steps {
            p += multiplier;

            if p > MAX {
                p = 0;
            } else if p < 0 {
                p = MAX;
            }

            if p == 0 {
                part_two += 1;
            }
        }

        if p == 0 {
            part_one += 1;
        }

        position = p;
    }

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}
