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
    let nums: Vec<usize> = fs::read_to_string(file_path)?
        .lines()
        .next()
        .expect("File is empty")
        .chars()
        .map(|c| c.to_digit(10).expect("Not a digit") as usize)
        .collect();

    let size: usize = nums.clone().into_iter().sum();
    let mut storage: Vec<i16> = vec![-1; size];

    let mut id: i16 = 0;
    let mut s_idx = 0;
    for i in 0..nums.len() {
        let d = nums[i];
        if i % 2 == 0 {
            for _ in 0..d {
                storage[s_idx] = id;
                s_idx += 1;
            }
            id += 1;
        } else {
            s_idx += d;
        }
    }

    let mut l = 0;
    let mut r = size - 1;
    loop {
        while l < r && storage[r] < 0 {
            r -= 1;
        }
        while l < r && storage[l] != -1 {
            l += 1;
        }
        if l >= r {
            break;
        }

        storage.swap(l, r);
    }

    let mut checksum: u64 = 0;
    for i in 0..size {
        if storage[i] == -1 {
            break;
        }
        checksum += storage[i] as u64 * i as u64;
    }
    dbg!(checksum);
    Ok(())
}
