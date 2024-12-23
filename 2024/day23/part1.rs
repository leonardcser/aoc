mod graph;
use graph::Graph;
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
    let content: String = fs::read_to_string(file_path)?;
    let computers: Vec<(&str, &str)> = content
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split("-");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let mut g = Graph::new();
    computers.iter().for_each(|c| {
        g.add_edge(c.0, c.1);
        g.add_edge(c.1, c.0);
    });

    let mut triangles: HashSet<Vec<&str>> = HashSet::new();
    for (n1, n2) in g.edges() {
        if let Some(n2_neighbors) = g.neighbors(&n2) {
            for n3 in n2_neighbors {
                if g.has_edge(n3, &n1) {
                    let mut t = vec![n1, n2, n3];
                    t.sort();
                    triangles.insert(t);
                }
            }
        }
    }

    let mut result = 0;
    for t in triangles {
        if t.iter().any(|c| c.starts_with("t")) {
            result += 1;
        }
    }
    dbg!(result);

    Ok(())
}
