fn main() {
    let (_, input) = common::read_input();

    let mut part_one: i64 = 0;
    let mut part_two: i64 = 0;

    for range in input.split(",") {
        let (start, end) = range.split_once("-").unwrap();

        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();

        for i in start..=end {
            let digits = i.to_string();

            // Solve part one
            let (first, second) = digits.split_at(digits.len() / 2);
            if first == second {
                part_one += i;
            }

            // Solve part two
            // This checks all possible patterns up to half the length of the digits.
            // A bit brute-forcey, but it's pretty fast.
            'j: for j in 1..=(digits.len() / 2) {
                if digits.len() % j != 0 {
                    continue;
                }

                let curr_slice = &digits[..j];
                let step = curr_slice.len();

                let mut k = j;
                while k < digits.len() {
                    let next_slice = &digits[k..k + step];
                    if curr_slice != next_slice || k + step > digits.len() {
                        continue 'j;
                    }

                    k += step;
                }

                part_two += digits.parse::<i64>().unwrap();

                // We can break out because we found a pattern.
                break;
            }
        }
    }

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}
