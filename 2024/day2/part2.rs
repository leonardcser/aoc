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

// https://maebli.github.io/rust/2024/12/02/100rust-82.html
fn check_safe(input: &Vec<i32>) -> bool {
    let x = input
        .windows(2)
        .map(|w| w[0] - w[1])
        .fold((true, 0), |acc, x| {
            (
                acc.0 && (x.abs() > 0 && x.abs() <= 3),
                acc.1 + ((x >= 0) as u32),
            )
        });
    x.0 && (x.1 == 0 || x.1 == (input.len() - 1) as u32)
}

fn make_subsets<T: Clone>(input: &Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let mut subset = input.clone();
        subset.remove(i);
        result.push(subset);
    }
    result
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

            if check_safe(&nums) {
                return true;
            }

            make_subsets(&nums).into_iter().any(|s| check_safe(&s))
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
