use std::collections::HashMap;
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
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    read_lines(file_path)?
        .filter_map(Result::ok) // Skip lines with errors
        .filter_map(|line| {
            let mut parts = line.split_whitespace().map(str::parse::<i32>);
            match (parts.next(), parts.next()) {
                (Some(Ok(l)), Some(Ok(r))) => Some((l, r)),
                _ => None,
            }
        })
        .for_each(|(l, r)| {
            left.push(l);
            *right.entry(r).or_insert(0) += 1;
        });

    let sum: i32 = left.iter().map(|l| (l * right.get(l).unwrap_or(&0))).sum();

    println!("result: {}", sum);
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
