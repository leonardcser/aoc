// https://github.com/michel-kraemer/adventofcode-rust/blob/main/2024/day06/src/main.rs
use std::env;
use std::fs;
use std::io;
use std::process::exit;

const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

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
    let input = fs::read_to_string(file_path)?;
    let lines = input.lines().collect::<Vec<_>>();
    let mut grid: Vec<u8> = lines
        .iter()
        .flat_map(|&line| line.as_bytes())
        .copied()
        .collect();
    let width = lines[0].len();
    let height = lines.len();

    // Find the start position
    let start = lines
        .iter()
        .enumerate()
        .find_map(|(y, &line)| {
            line.as_bytes()
                .iter()
                .position(|&ch| ch == b'^')
                .map(|x| (x as i32, y as i32))
        })
        .expect("Start position not found");

    let mut seen = vec![0u8; grid.len()];
    let mut marked = vec![false; grid.len()];

    // Part 1: Find the route
    let mut route = Vec::new();
    find_loop(
        &grid,
        width,
        height,
        start,
        Some(&mut route),
        &mut seen,
        &mut marked,
    );
    println!("Route length: {}", route.len());

    // Part 2: Count obstruction positions that create a loop
    let mut loop_count = 0;
    for &i in &route {
        if grid[i] == b'#' || i == start.1 as usize * width + start.0 as usize {
            continue;
        }

        grid[i] = b'#'; // Place obstruction
        if find_loop(&grid, width, height, start, None, &mut seen, &mut marked) {
            loop_count += 1;
        }
        grid[i] = b'.'; // Remove obstruction
    }
    println!("Number of loop-inducing obstructions: {}", loop_count);

    Ok(())
}

fn find_loop(
    grid: &[u8],
    width: usize,
    height: usize,
    mut pos: (i32, i32),
    mut route: Option<&mut Vec<usize>>,
    seen: &mut [u8],
    marked: &mut [bool],
) -> bool {
    seen.fill(0); // Clear previously seen states
    if let Some(route) = &mut route {
        marked.fill(false); // Reset marking for unique route recording
    }

    let mut dir = 0;
    loop {
        let idx = pos.1 as usize * width + pos.0 as usize;

        // Record route if needed
        if let Some(ref mut route) = route {
            if !marked[idx] {
                route.push(idx);
                marked[idx] = true;
            }
        }

        let next_pos = (pos.0 + DIRS[dir].0, pos.1 + DIRS[dir].1);

        // Check for out-of-bounds
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= width as i32
            || next_pos.1 >= height as i32
        {
            return false;
        }

        let next_idx = next_pos.1 as usize * width + next_pos.0 as usize;
        if grid[next_idx] == b'#' {
            // Obstruction: turn right
            dir = (dir + 1) % 4;

            // Check if this direction at this position was already visited
            let mask = 1u8 << dir;
            if seen[idx] & mask != 0 {
                return true; // Loop detected
            }
            seen[idx] |= mask; // Mark this direction as visited
        } else {
            pos = next_pos; // Move to the next position
        }
    }
}
