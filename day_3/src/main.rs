use std::time::Instant;

fn get_largest_number_combination(
    length: usize,
    digits: &[char],
    start: usize,
    combination: String,
) -> String {
    if combination.len() == length {
        return combination;
    }

    let numbers_required = length - combination.len();

    let mut max = '0';
    let mut index = start;

    for i in start..=digits.len() - numbers_required {
        let digit = digits[i];

        if digit > max {
            max = digit;
            index = i;
        }
    }

    get_largest_number_combination(length, digits, index + 1, combination + &max.to_string())
}

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut part_one: u64 = 0;
    let mut part_two: u64 = 0;

    for line in lines {
        let digits = line.chars().collect::<Vec<char>>();

        // Part One
        let part_one_combination = get_largest_number_combination(2, &digits, 0, String::new());
        part_one += part_one_combination.parse::<u64>().unwrap();

        // Part Two
        let part_one_combination = get_largest_number_combination(12, &digits, 0, String::new());
        part_two += part_one_combination.parse::<u64>().unwrap();
    }

    let elapsed = start.elapsed();
    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
    println!("Time: {:?}", elapsed);
}
