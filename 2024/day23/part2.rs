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

    for start in g.nodes() {
        let mut visited = HashSet::new();
        dfs(&g, start, start, &mut visited);
        dbg!(visited.len());
        // break;
    }

    Ok(())
}

fn dfs<'a>(g: &'a Graph<&str>, start: &str, node: &'a str, visited: &mut HashSet<&'a str>) {
    if visited.contains(&node) {
        if node == start {
            // println!("FOUND PATH");
        }
        return;
    }
    visited.insert(node);
    if let Some(neighbors) = g.neighbors(&node) {
        for n in neighbors {
            dfs(g, start, n, visited);
        }
    }
    // visited.remove(node);
}
