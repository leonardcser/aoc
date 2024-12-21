use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }
    if let Err(err) = process_file(&args[1]) {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

fn blink_recursive(num: usize, depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // Check cache first
    if let Some(&result) = cache.get(&(num, depth)) {
        return result;
    }

    let result = if depth == 0 {
        1
    } else if num == 0 {
        blink_recursive(1, depth - 1, cache)
    } else {
        let digits = num.to_string();
        let len = digits.len();

        if len % 2 == 0 {
            let (first_half, second_half) = digits.split_at(len / 2);
            let num1 = first_half.parse::<usize>().unwrap();
            let num2 = second_half.parse::<usize>().unwrap();
            blink_recursive(num1, depth - 1, cache) + blink_recursive(num2, depth - 1, cache)
        } else {
            blink_recursive(num * 2024, depth - 1, cache)
        }
    };

    // Store in cache
    cache.insert((num, depth), result);
    result
}

fn process_file(file_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let numbers: Vec<usize> = content
        .lines()
        .next()
        .expect("File is empty")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let depth = 75;
    let mut cache = HashMap::new();

    let total: usize = numbers
        .iter()
        .map(|&num| blink_recursive(num, depth, &mut cache))
        .sum();

    println!("Result: {}", total);
    Ok(())
}
