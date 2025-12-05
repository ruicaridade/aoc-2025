use std::time::Instant;

fn compact_ranges(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut new_ranges: Vec<(i64, i64)> = Vec::new();

    if ranges.len() == 0 {
        return new_ranges;
    }

    let mut buffer: (i64, i64) = ranges[0];

    for i in 1..ranges.len() {
        let range = ranges[i];

        if range.0 <= buffer.1 {
            buffer = (buffer.0, buffer.1.max(range.1))
        } else {
            new_ranges.push(buffer);
            buffer = range;
        }
    }

    new_ranges.push(buffer);
    new_ranges
}

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut fresh_ranges: Vec<(i64, i64)> = Vec::new();
    let mut fresh_count = 0;
    let mut separator = 0;

    for i in 0..lines.len() {
        let line = &lines[i];

        if line.trim().len() == 0 {
            separator = i;
            break;
        }

        let (start_str, end_str) = line.split_once("-").unwrap();
        let curr_start = start_str.parse::<i64>().unwrap();
        let curr_end = end_str.parse::<i64>().unwrap();
        fresh_ranges.push((curr_start, curr_end));
    }

    fresh_ranges.sort_by(|(a, _), (b, _)| a.cmp(&b));
    fresh_ranges = compact_ranges(fresh_ranges);

    for i in separator + 1..lines.len() {
        let line = &lines[i];
        let n = line.parse::<i64>().unwrap();

        for j in 0..fresh_ranges.len() {
            let (start, end) = fresh_ranges[j];

            if n >= start && n <= end {
                fresh_count += 1;
                break;
            }
        }
    }

    let mut total_fresh_count = 0;
    for (start, end) in fresh_ranges {
        let diff = (end - start) + 1;
        total_fresh_count += diff;
    }

    let elapsed = start.elapsed();
    println!("Part 1: {}", fresh_count);
    println!("Part 2: {}", total_fresh_count);
    println!("Time: {:?}", elapsed);
}
