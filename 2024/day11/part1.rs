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
    let file_content = fs::read_to_string(file_path)?;
    let mut stones: Vec<String> = file_content
        .lines()
        .next()
        .expect("File is empty")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let ITER = 25;

    for _ in 0..ITER {
        let mut i = 0;
        while i < stones.len() {
            let s = &stones[i];
            if *s == 0.to_string() {
                stones[i] = 1.to_string();
            } else if s.len() % 2 == 0 {
                let s1 = s[..s.len() / 2].parse::<usize>().unwrap();
                let s2 = s[s.len() / 2..].parse::<usize>().unwrap();
                stones.remove(i);
                stones.insert(i, s2.to_string());
                stones.insert(i, s1.to_string());
                i += 1;
            } else {
                let mut new_s = s.parse::<usize>().unwrap();
                new_s *= 2024;
                stones[i] = new_s.to_string();
            }
            i += 1;
        }
    }

    let result = stones.len();
    dbg!(result);
    Ok(())
}
