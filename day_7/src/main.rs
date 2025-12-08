use std::time::Instant;

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut splits = 0;
    let mut beams = vec![0; lines[0].len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => beams[x] += 1,
                '^' => {
                    if beams[x] > 0 {
                        splits += 1;
                        beams[x] = 0;
                        beams[x + 1] += 1;
                        beams[x - 1] += 1;
                    }
                }
                _ => println!("({},{}) is {}", x, y, c),
            }
        }
    }

    let total_beams = beams.iter().sum::<usize>();

    println!("splits: {}", splits);
    println!("total beams: {}", total_beams);

    let elapsed = start.elapsed();
    println!("Part 1: {}", 0);
    println!("Part 2: {}", 0);
    println!("Time: {:?}", elapsed);
}
