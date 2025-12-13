use regex::Regex;
use std::time::Instant;

struct Machine {
    diagram: u16,
    buttons: Vec<u16>,
    joltages: Vec<usize>,
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

    for machine in machines {
        let mut shortest_length = usize::MAX;
        let mut queue: Vec<(u16, usize)> = Vec::new();

        for root_button in machine.buttons.iter() {
            queue.push((*root_button, 1));

            while !queue.is_empty() {
                let (curr_button, curr_length) = queue.remove(0);

                for button in machine.buttons.iter() {
                    let new_button = curr_button ^ button;
                    let new_length = curr_length + 1;

                    if new_button == machine.diagram {
                        shortest_length = new_length;
                        break;
                    }

                    if new_length < shortest_length {
                        queue.push((new_button, new_length));
                    }
                }
            }
        }

        println!("Shortest length: {}", shortest_length);
    }

    let elapsed = start.elapsed();
    println!("\nPart 1: {}", 0);
    println!("Time: {:?}", elapsed);
}

fn solve_part_two(_lines: &Vec<String>) {
    let start = Instant::now();

    let elapsed = start.elapsed();
    println!("\nPart 2: {}", 0);
    println!("Time: {:?}", elapsed);
}

fn main() {
    let lines = common::read_lines_from_input();

    solve_part_one(&lines);
    solve_part_two(&lines);
}
