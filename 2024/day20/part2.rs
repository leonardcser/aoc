use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }
    if let Err(err) = process_file(&args[1]) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

type Position = (i32, i32);

const DIRS: [Position; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn process_file(file_path: &str) -> io::Result<()> {
    let grid: Vec<Vec<char>> = fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    // Find start and end position
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start = (x as i32, y as i32);
            } else if ch == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }
    assert!(start != (0, 0) && end != (0, 0));

    // Create the path
    let mut curr = start;
    let mut path = Vec::new();
    path.push(curr);
    let mut visited = HashSet::new();
    'done: loop {
        for dir in DIRS {
            let new_curr = (dir.0 + curr.0, dir.1 + curr.1);
            if visited.contains(&new_curr)
                || new_curr.0 < 0
                || new_curr.0 >= grid[0].len() as i32
                || new_curr.1 < 0
                || new_curr.1 >= grid.len() as i32
            {
                continue;
            }
            let next_ch = grid[new_curr.1 as usize][new_curr.0 as usize];
            if next_ch == '.' {
                path.push(new_curr);
                visited.insert(curr);
                curr = new_curr;
                break;
            }
            if next_ch == 'E' {
                path.push(new_curr);
                break 'done;
            }
        }
    }

    // For every combination, calculate the path len
    let pos_to_index: HashMap<Position, usize> =
        path.iter().enumerate().map(|(i, &pos)| (pos, i)).collect();

    let mut times = HashMap::new();
    for i in 0..path.len() - 2 {
        for j in i + 2..path.len() {
            let dx = (path[i].0 - path[j].0).abs();
            if dx > 20 {
                continue;
            }
            let dy = (path[i].1 - path[j].1).abs();
            let jump_len = (dx + dy) as usize;
            if jump_len > 20 {
                continue;
            }
            let jump_start = pos_to_index[&path[i]];
            let jump_end = pos_to_index[&path[j]];

            // Calculate new path length without traversing the graph
            let new_len = jump_start + jump_len + path.len() - jump_end;
            let time_saved = path.len() - new_len;

            if time_saved > 0 {
                *times.entry(new_len).or_insert(0) += 1;
            }
        }
    }

    let mut result = 0;
    for (k, v) in times {
        let time_saved = path.len() - k;
        if time_saved >= 100 {
            result += v;
        }
    }
    dbg!(result);
    Ok(())
}
