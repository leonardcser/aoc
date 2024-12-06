use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::process::exit;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

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

fn process_file(file_path: &str) -> io::Result<()> {
    let mut grid: Vec<Vec<char>> = fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut start = Point { x: 0, y: 0 };
    let w = grid[0].len();
    let h = grid.len();

    // Find start position
    'found: for j in 0..h {
        for i in 0..w {
            if grid[j][i] == '^' {
                start = Point { x: i, y: j };
                break 'found;
            }
        }
    }

    let mut result = 0;

    // Test every possible point for adding an obstruction
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '.' {
                grid[y][x] = '#';
                if is_looping(&start, &grid, w, h) {
                    result += 1;
                }
                grid[y][x] = '.';
            }
        }
    }

    dbg!(result);
    Ok(())
}

fn is_looping(start: &Point, grid: &[Vec<char>], w: usize, h: usize) -> bool {
    let mut visited = HashSet::new();
    let mut curr = *start;
    // Initial direction (up)
    let mut dir = (0, -1);

    loop {
        // Mark current position as visited with direction
        if !visited.insert((curr, dir)) {
            // Loop detected
            return true;
        }

        let (dx, dy) = dir;
        let next_x = curr.x as isize + dx;
        let next_y = curr.y as isize + dy;

        if next_x < 0 || next_x >= w as isize || next_y < 0 || next_y >= h as isize {
            // Out of bounds
            break;
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if grid[next_y][next_x] == '#' {
            // Change direction clockwise upon hitting obstruction
            dir = match dir {
                (0, -1) => (1, 0),  // Up to Right
                (1, 0) => (0, 1),   // Right to Down
                (0, 1) => (-1, 0),  // Down to Left
                (-1, 0) => (0, -1), // Left to Up
                _ => dir,
            };
        } else {
            curr = Point {
                x: next_x,
                y: next_y,
            };
        }
    }

    // No loop detected
    false
}
