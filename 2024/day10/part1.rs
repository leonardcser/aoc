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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn process_file(file_path: &str) -> io::Result<()> {
    let topo: Vec<Vec<usize>> = fs::read_to_string(file_path)?
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Not a digit") as usize)
                .collect()
        })
        .collect();

    let mut zeros: Vec<Point> = Vec::new();
    for y in 0..topo.len() {
        let l = &topo[y];
        for x in 0..l.len() {
            if l[x] == 0 {
                zeros.push(Point {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    let mut result = 0;
    for p in zeros.clone().into_iter() {
        result += get_route_trailheads(&topo, &p);
    }
    dbg!(result);
    Ok(())
}

const DIRS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

fn get_route_trailheads(topo: &Vec<Vec<usize>>, p: &Point) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack: Vec<Point> = Vec::new();
    visited.insert(*p);
    stack.push(*p);

    let mut trailheads = 0;
    loop {
        let mut found = false;
        let mut top = stack[stack.len() - 1];

        for dir in DIRS {
            let next_p = Point {
                x: top.x + dir.x,
                y: top.y + dir.y,
            };
            if !visited.contains(&next_p)
                && next_p.y < topo.len() as i32
                && next_p.y >= 0
                && next_p.x < topo[0].len() as i32
                && next_p.x >= 0
                && topo[next_p.y as usize][next_p.x as usize] as i32
                    - topo[top.y as usize][top.x as usize] as i32
                    == 1
            {
                found = true;
                visited.insert(next_p);
                stack.push(next_p);
                top = stack[stack.len() - 1];
                break;
            }
        }
        if !found {
            if top == *p {
                break;
            }
            stack.pop();
        } else {
            let top = stack[stack.len() - 1];
            if topo[top.y as usize][top.x as usize] == 9 {
                trailheads += 1;
                stack.pop();
            }
        }
    }
    return trailheads;
}
