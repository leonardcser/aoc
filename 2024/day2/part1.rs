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

fn process_file(file_path: &str) -> io::Result<()> {
    let result = read_lines(file_path)?
        .filter_map(Result::ok) // Skip lines with errors
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .filter_map(|word| word.parse::<i32>().ok()) // Parse integers, skip errors
                .collect::<Vec<_>>();

            if nums.len() < 2 {
                return true;
            }

            let mut is_increasing = true;
            let mut is_decreasing = true;

            for pair in nums.windows(2) {
                let dist = (pair[0] - pair[1]).abs();
                if dist < 1 || dist > 3 {
                    return false;
                }
                if pair[0] < pair[1] {
                    is_decreasing = false;
                } else if pair[0] > pair[1] {
                    is_increasing = false;
                }
            }

            return is_increasing || is_decreasing;
        })
        .count();

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
