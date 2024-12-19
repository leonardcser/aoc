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
    let mut sections = file_content.trim().split("\n\n");

    let subseqs: Vec<&str> = sections
        .next()
        .expect("Expected patterns")
        .split(", ")
        .collect();

    let seqs: Vec<&str> = sections
        .next()
        .expect("Expected sequences")
        .lines()
        .collect();

    let mut result = 0;
    for seq in seqs {
        result += count_subseqs(&seq, &subseqs);
    }
    dbg!(result);
    Ok(())
}

fn count_subseqs(seq: &str, subseqs: &Vec<&str>) -> usize {
    let n = seq.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for subseq in subseqs {
            let m = subseq.len();
            if i >= m && seq[i - m..i] == **subseq {
                dp[i] += dp[i - m];
            }
        }
    }
    dp[n]
}
