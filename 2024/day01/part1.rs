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
    let mut right: Vec<i32> = Vec::new();

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
            insert_sorted(&mut left, l);
            insert_sorted(&mut right, r);
        });

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("result: {}", sum);
    Ok(())
}

fn insert_sorted(vec: &mut Vec<i32>, new: i32) {
    let mut lo: usize = 0;
    let mut hi: usize = vec.len();

    while lo < hi {
        let mid: usize = (lo + hi) / 2;
        if vec[mid] < new {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    vec.insert(lo, new);
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
