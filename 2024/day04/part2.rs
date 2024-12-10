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
    if col >= grid[row].len() - 2 || row >= grid.len() - 2 {
        return 0;
    }

    let word_chars: Vec<char> = word.chars().collect();
    let directions = [(1, 1), (1, -1)];

    let mut curr_y = row;
    let mut curr_x = col;

    for &(dy, dx) in &directions {
        let mut reversed = false;
        if grid[curr_y][curr_x] == word_chars[word.len() - 1] {
            reversed = true;
        } else if grid[curr_y][curr_x] != word_chars[0] {
            return 0;
        }

        for i in 0..word.len() {
            if !reversed {
                if grid[curr_y][curr_x] != word_chars[i] {
                    return 0;
                }
            } else {
                if grid[curr_y][curr_x] != word_chars[word.len() - i - 1] {
                    return 0;
                }
            }

            curr_y = (curr_y as i32 + dy) as usize;
            curr_x = (curr_x as i32 + dx) as usize;
        }

        curr_y = row;
        curr_x = col + 2;
    }

    1
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

    let result = search(&grid, "MAS");
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
