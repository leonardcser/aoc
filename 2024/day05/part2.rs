mod graph;
use graph::Graph;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};
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
    let content = read_to_string(file_path)?
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut g = Graph::new();
    content[0]
        .trim()
        .split("\n")
        .map(|r| {
            let mut parts = r.split("|").map(|i| i.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .for_each(|(l, r)| {
            g.add_edge(l, r);
        });

    let updates: Vec<Vec<i32>> = content[1]
        .trim()
        .split("\n")
        .map(|u| u.split(",").map(|i| i.parse::<i32>().unwrap()).collect())
        .collect();

    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();
    for k in 0..updates.len() {
        let u = &updates[k];
        'next: for i in 0..u.len() - 1 {
            for j in i + 1..u.len() {
                if !g.has_edge(u[i], u[j]) {
                    invalid_updates.push(u.clone());
                    break 'next;
                }
            }
        }
    }

    let mut result = 0;
    for k in 0..invalid_updates.len() {
        let u = &mut invalid_updates[k];
        for i in 0..u.len() - 1 {
            for j in i + 1..u.len() {
                let mut swap_i = 0;
                while !g.has_edge(u[i - swap_i], u[j - swap_i]) {
                    u.swap(i - swap_i, j - swap_i);
                    if i - swap_i == 0 {
                        break;
                    }
                    swap_i += 1;
                }
            }
        }
        result += u[u.len() / 2];
    }

    dbg!(result);
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

fn read_to_string<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
