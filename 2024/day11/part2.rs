use std::collections::HashMap;
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

fn recursive_stone_count(
    initial_stones: &[usize],
    remaining_iterations: usize,
    memo: &mut HashMap<(Vec<usize>, usize), usize>,
) -> usize {
    if remaining_iterations == 0 {
        return initial_stones.len();
    }

    let key = (initial_stones.to_vec(), remaining_iterations);
    if let Some(&cached_result) = memo.get(&key) {
        return cached_result;
    }

    let next_stones: Vec<usize> = initial_stones
        .iter()
        .flat_map(|&stone| match stone {
            0 => vec![1],
            x if x.to_string().len() % 2 == 0 => {
                let len = x.to_string().len();
                let divisor = 10_usize.pow(len as u32 / 2);
                vec![x / divisor, x % divisor]
            }
            x => vec![x * 2024],
        })
        .collect();

    let result = recursive_stone_count(&next_stones, remaining_iterations - 1, memo);
    memo.insert(key, result);
    result
}

fn process_file(file_path: &str) -> io::Result<()> {
    let file_content = fs::read_to_string(file_path)?;
    let initial_stones: Vec<usize> = file_content
        .lines()
        .next()
        .expect("File is empty")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let iterations = 40;
    let mut memo = HashMap::new();

    let result = recursive_stone_count(&initial_stones, iterations, &mut memo);
    dbg!(result);

    Ok(())
}
