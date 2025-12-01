use std::env;
use std::fs;

pub fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    
    args[1].clone()
}

pub fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect(&format!("Failed to read file: {}", file_path))
}

pub fn read_input() -> (String, String) {
    let file_path = get_input_path();
    let contents = read_input_file(&file_path);
    (file_path, contents)
}

pub fn read_lines_from_input() -> Vec<String> {
    let (_file_path, contents) = read_input();
    contents.lines().map(|line| line.to_string()).collect::<Vec<String>>()
}