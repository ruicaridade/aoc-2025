use std::time::Instant;

const PART_1_LEN: usize = 2;
const PART_2_LEN: usize = 12;

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut part_one: i32 = 0;
    let mut part_two: i32 = 0;

    for line in lines {
        let mut slice: Vec<(i32, char)> = line.char_indices().map(|(i, c)| (i as i32, c)).collect();
        slice.sort_by(|a, b| b.1.cmp(&a.1));

        let mut new = slice.iter().take(PART_1_LEN);

        println!("{} -> {:?}", line, new);

        // for i in 0..digits.len() {
        //     let a = digits[i];

        //     let mut b = None;
        //     for j in 0..digits.len() {
        //         if digits[j].0 > a.0 {
        //             b = Some(digits[j]);
        //             break;
        //         }
        //     }

        //     if b.is_some() {
        //         let combined = if a.0 < b.unwrap().0 {
        //             format!("{}{}", a.1, b.unwrap().1)
        //         } else {
        //             format!("{}{}", b.unwrap().1, a.1)
        //         };

        //         println!("{} -> {}", line, combined);
        //         part_one += combined.parse::<i32>().unwrap();
        //         break;
        //     }
        // }
    }

    let elapsed = start.elapsed();
    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
    println!("Time: {:?}", elapsed);
}
