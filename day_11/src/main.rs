use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Device {
    fn new(input: &String) -> Self {
        let (name_str, outputs_str) = input.split_once(':').unwrap();

        let name = name_str.trim().to_string();
        let outputs: Vec<String> = outputs_str
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .collect();

        Self {
            name: name,
            outputs: outputs,
        }
    }
}

fn traverse(
    devices: &HashMap<String, Device>,
    current_device: &Device,
    found_dac: bool,
    found_fft: bool,
) -> usize {
    let mut count = 0;
    let new_found_dac = found_dac || current_device.name == "dac";
    let new_found_fft = found_fft || current_device.name == "fft";

    for output in &current_device.outputs {
        if output == "out" {
            if new_found_dac && new_found_fft {
                count += 1;
            }
        } else {
            let next_device = devices.get(output).unwrap();
            count += traverse(devices, &next_device, new_found_dac, new_found_fft);
        }
    }

    count
}

fn solve_part_one(lines: &Vec<String>) {
    let start = Instant::now();

    let devices = lines
        .iter()
        .map(|line| Device::new(line))
        .map(|d| (d.name.clone(), d))
        .collect::<HashMap<String, Device>>();

    let start_device = devices.get("you").unwrap();
    let result = traverse(&devices, &start_device, false, false);

    let elapsed = start.elapsed();
    println!("Part 1: {}", result);
    println!("Time: {:.2?}", elapsed);
}

fn solve_part_two(lines: &Vec<String>) {
    let start = Instant::now();

    let devices = lines
        .iter()
        .map(|line| Device::new(line))
        .map(|d| (d.name.clone(), d))
        .collect::<HashMap<String, Device>>();

    let start_device = devices.get("svr").unwrap();
    let result = traverse(&devices, start_device, false, false);

    let elapsed = start.elapsed();
    println!("Part 2: {}", result);
    println!("Time: {:.2?}", elapsed);
}

fn main() {
    let lines = common::read_lines_from_input();

    solve_part_one(&lines);
    solve_part_two(&lines);
}
