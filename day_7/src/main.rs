use std::time::Instant;

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut splits = 0;
    let mut beams: Vec<u64> = vec![0; lines[0].len()];

    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();

        for x in 0..beams.len() {
            let char = chars.get(x);

            match char {
                Some('S') => {
                    beams[x] = 1;
                }
                Some('^') => {
                    if beams[x] == 0 {
                        continue;
                    }

                    splits += 1;
                    beams[x + 1] += beams[x];
                    beams[x - 1] += beams[x];
                    beams[x] = 0;
                }
                _ => {}
            }
        }
    }

    let total_beams = beams.iter().sum::<u64>();

    println!("splits: {}", splits);
    println!("total beams: {}", total_beams);

    let elapsed = start.elapsed();
    println!("Part 1: {}", 0);
    println!("Part 2: {}", 0);
    println!("Time: {:?}", elapsed);
}
