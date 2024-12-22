use std::collections::HashMap;
use std::collections::HashSet;
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

    let mut sequence_results: Vec<HashMap<Vec<i64>, i64>> = Vec::new();

    // Pre-compute all sequences and their results for each buyer
    for &initial_secret in &lines {
        let mut secret = initial_secret;
        let mut prices = vec![secret % 10];
        let mut changes = Vec::new();
        let mut sequences: HashMap<Vec<i64>, i64> = HashMap::new();

        // Generate all prices and changes
        for _ in 0..2000 {
            let new_secret = next_secret(secret);
            let new_price = new_secret % 10;
            prices.push(new_price);

            let change = new_price as i64 - prices[prices.len() - 2] as i64;
            changes.push(change);

            // Record all 4-change sequences that end at this position
            if changes.len() >= 4 {
                let sequence = changes[changes.len() - 4..].to_vec();
                if !sequences.contains_key(&sequence) {
                    sequences.insert(sequence, new_price);
                }
            }

            secret = new_secret;
        }

        sequence_results.push(sequences);
    }

    // Find common sequences across buyers
    let mut common_sequences: HashSet<Vec<i64>> = HashSet::new();
    if let Some(first_map) = sequence_results.first() {
        common_sequences = first_map.keys().cloned().collect();
    }

    // Find the sequence that gives the maximum banana count
    let mut max_bananas = 0;
    let mut best_sequence = Vec::new();

    for sequence in common_sequences {
        let mut total = 0;
        for buyer_sequences in &sequence_results {
            if let Some(&price) = buyer_sequences.get(&sequence) {
                total += price;
            }
        }

        if total > max_bananas {
            max_bananas = total;
            best_sequence = sequence;
        }
    }

    println!("Best sequence: {:?}", best_sequence);
    println!("Total bananas: {}", max_bananas);
    Ok(())
}

fn next_secret(secret: i64) -> i64 {
    let mut num = secret;
    num = ((num * 64) ^ num) % 16777216;
    num = ((num / 32) ^ num) % 16777216;
    num = ((num * 2048) ^ num) % 16777216;
    num
}
