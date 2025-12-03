use std::env;
use std::fs;
use std::path::Path;

pub fn get_input_path() -> String {
    let args: Vec<String> = env::args().collect();

    // If no input file is provided, default to day_N/input.txt based on binary name
    if args.len() < 2 {
        let binary_path = &args[0];
        let binary_name = Path::new(binary_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("day_unknown");

        let default_path = format!("{}/input.txt", binary_name);
        return default_path;
    }

    args[1].clone()
}

pub fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("Failed to read file: {}", file_path))
}

pub fn read_input() -> (String, String) {
    let file_path = get_input_path();
    let contents = read_input_file(&file_path);
    (file_path, contents)
}

pub fn read_lines_from_input() -> Vec<String> {
    let (_file_path, contents) = read_input();
    contents
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
