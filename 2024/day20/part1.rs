mod graph;
use graph::Graph;
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
        if y == 0 || y == grid.len() - 1 {
            continue;
        }
        for (x, &ch) in row.iter().enumerate() {
            if x == 0 || x == row.len() - 1 {
                continue;
            }
            if ch == 'S' {
                start = (x as i32, y as i32);
            } else if ch == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }
    assert!(start != (0, 0));
    assert!(end != (0, 0));

    // Create the graph
    let mut g = Graph::new();
    let mut curr = start;
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
                g.add_edge(curr, new_curr);
                visited.insert(curr);
                curr = new_curr;
                break;
            }
            if next_ch == 'E' {
                g.add_edge(curr, new_curr);
                break 'done;
            }
        }
    }

    // For every combination, calculate the path len
    let path = get_path(g.clone(), start);

    let mut times = HashMap::new();
    for i in 0..path.len() - 2 {
        for j in i + 2..path.len() {
            let dx = (path[i].0 - path[j].0).abs();
            let dy = (path[i].1 - path[j].1).abs();
            if !(dx == 0 && dy == 2 || dy == 0 && dx == 2) {
                continue;
            }
            let mid = ((path[i].0 + path[j].0) / 2, (path[i].1 + path[j].1) / 2);
            if grid[mid.1 as usize][mid.0 as usize] != '#' {
                continue;
            }
            g.remove_edge(&path[i], &path[i + 1]);
            g.add_edge(path[i], path[j]);

            let new_len = get_path_len(&g, &start);
            *times.entry(new_len).or_insert(0) += 1;

            g.remove_edge(&path[i], &path[j]);
            g.add_edge(path[i], path[i + 1]);
        }
    }

    let mut result = 0;
    for (k, v) in times {
        let time_saved = path.len() - k - 2;
        if time_saved >= 100 {
            result += v;
        }
    }
    dbg!(result);
    Ok(())
}

fn get_path(g: Graph<Position>, start: Position) -> Vec<Position> {
    let mut curr = start;
    let mut path = Vec::new();
    path.push(start);
    loop {
        if let Some(n) = g.neighbors(&curr) {
            curr = *n.iter().next().expect("Path should have a continuation");
            path.push(curr);
        } else {
            break;
        }
    }

    path
}
fn get_path_len(g: &Graph<Position>, start: &Position) -> usize {
    let mut len = 0;
    let mut curr = start;
    loop {
        if let Some(n) = g.neighbors(curr) {
            curr = n.iter().next().expect("Path should have a continuation");
            len += 1;
        } else {
            break;
        }
    }

    len
}
