use std::time::Instant;

const PAPER_CELL: char = '@';
const ACCESSIBLE_CELL: char = 'X';
const EMPTY_CELL: char = '.';

fn is_paper_roll(a: char) -> bool {
    a == PAPER_CELL || a == ACCESSIBLE_CELL
}

fn count_adjacent_rolls(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut adjacent_count: usize = 0;

    let current_x = x as i32;
    let current_y = y as i32;

    for adjacent_y in current_y - 1..=current_y + 1 {
        for adjacent_x in current_x - 1..=current_x + 1 {
            if adjacent_y >= 0
                && adjacent_x >= 0
                && adjacent_y < grid.len() as i32
                && adjacent_x < grid[0].len() as i32
                && (adjacent_x != current_x || adjacent_y != current_y)
                && is_paper_roll(grid[adjacent_y as usize][adjacent_x as usize])
            {
                adjacent_count += 1;
            }
        }
    }

    adjacent_count
}

fn remove_accessible_rolls(grid: &mut Vec<Vec<char>>) -> usize {
    let mut removed_count = 0;
    let mut delete = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_paper_roll(grid[y][x]) && count_adjacent_rolls(&grid, x, y) < 4 {
                grid[y][x] = ACCESSIBLE_CELL;
                removed_count += 1;
                delete.push((x, y));
            }
        }
    }

    for (x, y) in delete {
        grid[y][x] = EMPTY_CELL;
    }

    removed_count
}

fn main() {
    let start = Instant::now();
    let lines = common::read_lines_from_input();

    let mut grid = vec![vec!['.'; lines[0].len()]; lines.len()];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;
        }
    }

    let mut first: bool = true;
    let mut first_removed = 0;
    let mut total_removed = 0;

    loop {
        let removed_count = remove_accessible_rolls(&mut grid);
        if removed_count == 0 {
            break;
        }

        // Part one.
        if first {
            first_removed = removed_count;
            first = false;
        }

        // Part two.
        total_removed += removed_count
    }

    println!();

    let elapsed = start.elapsed();
    println!("Part 1: {}", first_removed);
    println!("Part 2: {}", total_removed);
    println!("Time: {:?}", elapsed);
}
