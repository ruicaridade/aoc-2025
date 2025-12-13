use itertools::Itertools;
use std::time::Instant;
use std::usize;

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

    fn parse_button(s: &str, diagram_length: usize) -> u16 {
        s.split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .fold(0u16, |acc, char_index| {
                let bit_position = diagram_length - 1 - char_index;
                acc | (1 << bit_position)
            })
    }

    fn find_minimum_button_presses(&self) -> usize {
        for i in 1..=self.buttons.len() {
            for bslice in self.buttons.iter().combinations(i) {
                let result = bslice.iter().fold(0u16, |acc, &&a| acc ^ a);
                if result == self.diagram {
                    return i;
                }
            }
        }

        panic!("No valid combination found!")
    }

    fn new(value: &str) -> Self {
        let mut indicators: u16 = 0;
        let mut diagram_length: usize = 0;
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
                            diagram_length = buffer.len();
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
                            buttons.push(Self::parse_button(buffer.as_str(), diagram_length));
                            i = j;
                            break;
                        } else {
                            buffer.push(c);
                        }
                    }
                }
                '{' => {
                    // Skip everything inside curly braces
                    for j in i + 1..chars.len() {
                        if chars[j] == '}' {
                            i = j;
                            break;
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

fn solve_part_one(lines: &Vec<String>) {
    let start = Instant::now();

    let mut machines: Vec<Machine> = Vec::new();

    for line in lines {
        machines.push(Machine::new(&line));
    }

    let count = machines
        .iter()
        .map(|m| m.find_minimum_button_presses())
        .sum::<usize>();

    let elapsed = start.elapsed();
    println!("\nPart 1: {}", count);
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
