use std::time::Instant;
use std::{fmt, usize};

struct Machine {
    diagram: u16,
    buttons: Vec<u16>,
}

impl Machine {
    fn parse_diagram(s: &str) -> u16 {
        s.chars()
            .take(16)
            .fold(0u16, |acc, ch| (acc << 1) | if ch == '#' { 1 } else { 0 })
    }

    fn parse_button(s: &str) -> u16 {
        s.split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .fold(0u16, |acc, bit_index| acc | (1 << bit_index))
    }

    fn format_diagram(&self) -> String {
        (0..16)
            .rev()
            .map(|i| {
                if (self.diagram >> i) & 1 == 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .collect()
    }

    fn format_button(button: u16) -> String {
        let indices: Vec<usize> = (0..16)
            .filter_map(|i| {
                if (button >> i) & 1 == 1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        if indices.is_empty() {
            String::new()
        } else if indices.len() == 1 {
            format!("{}", indices[0])
        } else {
            indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(",")
        }
    }

    fn find_shortest_combination(&self) -> Vec<u16> {
        let mut combination = Vec::new();

        for i in 0..self.buttons.len() {
            let basis = &self.buttons[i];
            let mut counter: usize = 0;

            while counter < combination.len() {
                for j in 0..self.buttons.len() {}

                counter += 1;
            }
        }

        combination
    }

    fn new(value: &str) -> Self {
        let mut indicators: u16 = 0;
        let mut buttons: Vec<u16> = Vec::new();
        let mut buffer = String::new();

        let chars = value.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < chars.len() {
            match &chars[i] {
                '[' => {
                    buffer.clear();

                    for j in i + 1..chars.len() {
                        let c = chars[j];
                        if c == ']' {
                            indicators = Self::parse_diagram(buffer.as_str());
                            i = j;
                            break;
                        } else {
                            buffer.push(c);
                        }
                    }
                }
                '(' => {
                    buffer.clear();

                    for j in i + 1..chars.len() {
                        let c = chars[j];
                        if c == ')' {
                            buttons.push(Self::parse_button(buffer.as_str()));
                            i = j;
                            break;
                        } else {
                            buffer.push(c);
                        }
                    }
                }
                _ => {}
            }

            i += 1;
        }

        Self {
            buttons: buttons,
            diagram: indicators,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format indicators: [.##.]
        write!(f, "[{}]", self.format_diagram())?;

        // Format buttons: (3) (1,3) (2) ...
        for button in &self.buttons {
            write!(f, " ({})", Self::format_button(*button))?;
        }

        Ok(())
    }
}

fn solve_part_one(lines: &Vec<String>) {
    let start = Instant::now();

    let mut machines: Vec<Machine> = Vec::new();

    for line in lines {
        machines.push(Machine::new(&line));
    }

    for machine in machines {
        println!("{}", machine);
        println!("{} -> {:?}", machine.diagram, machine.buttons);
        println!();
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
