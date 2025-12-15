use regex::Regex;
use std::{collections::VecDeque, time::Instant};

struct Machine {
    diagram: u16,
    buttons: Vec<u16>,
    joltages: Vec<u8>,
}

impl Machine {
    fn new(line: &String) -> Self {
        let diagram_re = Regex::new(r"\[([.#]+)\]").unwrap();
        let buttons_re = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
        let joltages_re = Regex::new(r"\{(\d+(?:,\d+)*)\}").unwrap();

        let diagram_str = diagram_re
            .captures(line)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .chars()
            .rev()
            .map(|c| if c == '#' { '1' } else { '0' })
            .collect::<String>();

        let diagram = u16::from_str_radix(&diagram_str, 2).unwrap();

        let mut buttons: Vec<u16> = Vec::new();
        for cap in buttons_re.captures_iter(line) {
            if let Some(numbers_str) = cap.get(1) {
                let numbers: Vec<usize> = numbers_str
                    .as_str()
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect();

                let mut button_str = "0".repeat(16).chars().collect::<Vec<char>>();
                let length = button_str.len();

                for number in numbers {
                    button_str[length - number - 1] = '1';
                }

                let button_str = button_str.iter().collect::<String>();
                buttons.push(u16::from_str_radix(&button_str, 2).unwrap());
            }
        }

        let joltages = joltages_re
            .captures(line)
            .and_then(|cap| cap.get(1))
            .map(|m| {
                m.as_str()
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect()
            })
            .unwrap_or_default();

        Self {
            diagram,
            buttons,
            joltages,
        }
    }
}

fn solve_part_one(lines: &Vec<String>) {
    let start = Instant::now();

    let machines = lines
        .iter()
        .map(|line| Machine::new(line))
        .collect::<Vec<Machine>>();

    let mut total = 0;
    let mut queue: VecDeque<(u16, usize)> = VecDeque::new();

    for machine in machines.iter() {
        let mut shortest_length = usize::MAX;

        for button in machine.buttons.iter() {
            queue.push_back((*button, 1));
        }

        while let Some((curr_button, curr_length)) = queue.pop_front() {
            if curr_button == machine.diagram {
                shortest_length = curr_length.min(shortest_length);
                continue;
            }

            for next_button in machine.buttons.iter() {
                let new_button = curr_button ^ *next_button;
                let new_length = curr_length + 1;

                if new_length < shortest_length {
                    queue.push_back((new_button, new_length));
                }
            }
        }

        total += shortest_length;
    }

    let elapsed = start.elapsed();
    println!("\nPart 1: {}", total);
    println!("Time: {:?}", elapsed);
}

fn apply_joltages(joltages: &Vec<u8>, button: &u16) -> Vec<u8> {
    let button_str = format!("{:b}", button);

    let mut new_joltages = vec![0; joltages.len()];
    for (i, c) in button_str.chars().enumerate() {
        if c == '1' {
            new_joltages[i] = 1;
        }
    }

    new_joltages
}

fn is_done(joltages: &Vec<u8>, joltages_to_check: &Vec<u8>) -> bool {
    joltages
        .iter()
        .zip(joltages_to_check.iter())
        .all(|(i, j)| i >= j)
}

fn solve_part_two(lines: &Vec<String>) {
    let start = Instant::now();

    let machines = lines
        .iter()
        .map(|line| Machine::new(line))
        .collect::<Vec<Machine>>();

    let mut total = 0;
    let mut queue: VecDeque<(Vec<u8>, usize)> = VecDeque::new();

    for machine in machines.iter() {
        let mut shortest_length = usize::MAX;

        for button in machine.buttons.iter() {
            let start_joltages = vec![0; machine.joltages.len()];
            queue.push_back((apply_joltages(&start_joltages, button), 1));
        }

        while let Some((curr_joltages, curr_length)) = queue.pop_front() {
            if !is_done(&curr_joltages, &machine.joltages) {
                if curr_joltages == machine.joltages {
                    shortest_length = curr_length.min(shortest_length);
                }

                continue;
            }

            for next_button in machine.buttons.iter() {
                let new_joltages = apply_joltages(&curr_joltages, next_button);
                let new_length = curr_length + 1;

                if new_length < shortest_length {
                    queue.push_back((new_joltages, new_length));
                }
            }
        }

        total += shortest_length;
    }

    let elapsed = start.elapsed();
    println!("\nPart 2: {}", total);
    println!("Time: {:?}", elapsed);
}

fn main() {
    let lines = common::read_lines_from_input();

    solve_part_one(&lines);
    solve_part_two(&lines);
}
