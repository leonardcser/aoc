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
    let lines: Vec<(u128, Vec<u128>)> = fs::read_to_string(file_path)?
        .lines()
        .filter_map(|l| {
            let mut parts = l.split(": ");
            let key = parts.next()?.parse::<u128>().expect("Cannot parse number");
            let values = parts
                .next()?
                .split(' ')
                .map(|p| p.parse::<u128>().expect("Cannot parse number"))
                .collect::<Vec<u128>>();
            Some((key, values))
        })
        .collect();

    let mut result = 0;
    for l in &lines {
        let (sum, nodes) = l;
        if nodes.len() == 1 && nodes[0] != *sum {
            continue;
        }
        // Total combinations of operations: 3^(nodes.len() - 1)
        let n_ops = nodes.len() - 1;
        for op_mask in 0..(3_usize.pow(n_ops as u32)) {
            let mut res = nodes[0];
            let mut current_mask = op_mask;
            for j in 0..n_ops {
                let op = current_mask % 3;
                current_mask /= 3;
                match op {
                    0 => {
                        // Addition
                        res += nodes[j + 1];
                    }
                    1 => {
                        // Multiplication
                        res *= nodes[j + 1];
                    }
                    2 => {
                        // Concatenation
                        res =
                            res * 10_u128.pow(nodes[j + 1].to_string().len() as u32) + nodes[j + 1];
                    }
                    _ => unreachable!(),
                }
            }
            if res == *sum {
                result += *sum;
                break;
            }
        }
    }

    dbg!(result);
    Ok(())
}
