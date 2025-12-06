use std::time::Instant;

fn split_lines(lines: &Vec<String>) -> Vec<Vec<&str>> {
    lines
        .iter()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect()
}

fn parse_expressions(lines: &Vec<String>) -> Vec<Vec<i64>> {
    let split_lines = split_lines(lines);
    let mut numbers: Vec<Vec<i64>> = vec![vec![0; lines.len() - 1]; split_lines[0].len()];

    for x in 0..split_lines.len() - 1 {
        let line = &split_lines[x];
        for y in 0..line.len() {
            let part = line[y].parse::<i64>().unwrap();
            numbers[y][x] = part;
        }
    }

    numbers
}

fn parse_right_to_left_expressions(lines: &Vec<String>) -> Vec<Vec<i64>> {
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    numbers.push(Vec::new());

    for x in (0..lines[0].len()).rev() {
        let mut digits: Vec<char> = Vec::new();

        for y in 0..lines.len() - 1 {
            let line_bytes = lines[y].as_bytes();
            if x < line_bytes.len() {
                let digit = line_bytes[x] as char;
                if digit != ' ' {
                    digits.push(digit);
                }
            }
        }

        if digits.len() == 0 {
            numbers.push(Vec::new());
        }

        if digits.len() > 0 {
            let number_str = String::from_iter(digits);
            let number = number_str.parse::<i64>().unwrap();
            if let Some(last) = numbers.last_mut() {
                last.push(number);
            }
        }
    }

    numbers
}

fn parse_operators(lines: &Vec<String>) -> Vec<char> {
    lines[lines.len() - 1]
        .split_whitespace()
        .map(|c| c.as_bytes()[0] as char)
        .collect()
}

fn solve_expressions(expressions: &Vec<Vec<i64>>, operators: &Vec<char>) -> i64 {
    let mut total = 0;

    for (x, expression) in expressions.iter().enumerate() {
        let operator = operators[x];
        let mut result = expression[0];

        for y in 1..expression.len() {
            let number = expression[y];

            result = match operator {
                '*' => result * number,
                '+' => result + number,
                _ => panic!("Invalid operator"),
            };
        }

        total += result;
    }

    total
}

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let expressions = parse_expressions(&lines);
    let operators = parse_operators(&lines);

    let rtl_expressions = parse_right_to_left_expressions(&lines);
    let rtl_operators = operators.iter().rev().cloned().collect();

    let elapsed = start.elapsed();
    println!("Part 1: {}", solve_expressions(&expressions, &operators));
    println!(
        "Part 2: {}",
        solve_expressions(&rtl_expressions, &rtl_operators)
    );
    println!("Time: {:?}", elapsed);
}
