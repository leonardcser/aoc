use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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

fn search2d(grid: &Vec<Vec<char>>, row: usize, col: usize, word: &str) -> i32 {
    if grid[row][col] != word.chars().collect::<Vec<char>>()[0] {
        return 0;
    }

    let x: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let y: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    let n = grid.len() as i32;
    let m = grid[row].len() as i32;
    let mut occ = 0;

    for dir in 0..8 {
        let mut curr_y = row as i32 + y[dir];
        let mut curr_x = col as i32 + x[dir];
        let mut found = true;

        for i in 1..word.len() {
            if curr_y >= n || curr_y < 0 || curr_x >= m || curr_x < 0 {
                found = false;
                break;
            }
            if grid[curr_y as usize][curr_x as usize] != word.chars().collect::<Vec<char>>()[i] {
                found = false;
                break;
            }

            curr_y += y[dir];
            curr_x += x[dir];
        }

        if found {
            occ += 1;
        }
    }

    occ
}

fn search(grid: &Vec<Vec<char>>, word: &str) -> i32 {
    let mut occ = 0;

    for j in 0..grid.len() {
        let row = &grid[j];
        for i in 0..row.len() {
            occ += search2d(grid, j, i, word);
        }
    }

    occ
}

fn process_file(file_path: &str) -> io::Result<()> {
    let grid: Vec<Vec<char>> = read_lines(file_path)?
        .flatten()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let result = search(&grid, "XMAS");
    dbg!(result);
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
