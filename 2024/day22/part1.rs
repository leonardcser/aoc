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

fn process_file(file_path: &str) -> io::Result<()> {
    let lines: Vec<i64> = fs::read_to_string(file_path)?
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut result = 0;
    for line in lines {
        let mut secret = line;
        for _ in 0..2000 {
            secret = next_secret(secret);
        }
        result += secret;
    }
    dbg!(result);
    Ok(())
}

fn next_secret(secret: i64) -> i64 {
    let mut num = secret;
    num = ((num * 64) ^ num) % 16777216;
    num = ((num / 32) ^ num) % 16777216;
    num = ((num * 2048) ^ num) % 16777216;

    num
}
