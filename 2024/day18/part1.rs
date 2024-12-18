mod graph;
use graph::WeightedGraph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

const MEM_SIZE: usize = 71;
const STEPS: usize = 1024;

type Position = (usize, usize);

#[derive(Hash, Eq, PartialEq)]
struct State {
    position: Position,
    priority: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn process_file(file_path: &str) -> io::Result<()> {
    let file_content = fs::read_to_string(file_path)?;
    let bytes: Vec<Position> = file_content
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let first = parts.next().unwrap().parse::<usize>().unwrap();
            let second = parts.next().unwrap().parse::<usize>().unwrap();
            (first, second)
        })
        .collect();

    let mut memory: [[usize; MEM_SIZE]; MEM_SIZE] = [[0; MEM_SIZE]; MEM_SIZE];

    for (i, &byte) in bytes.iter().enumerate() {
        if i >= STEPS {
            break;
        }
        let (x, y) = byte;
        memory[y][x] = 1;
    }

    // Construct the graph
    let g = construct_graph(&memory);
    let start = (0, 0);
    let goal = (memory[0].len() - 1, memory.len() - 1);
    let (came_from, cost_so_far) = A_star(&g, start, goal);

    display_result(&memory, &came_from, &cost_so_far, goal);
    let result = get_path_length(&came_from, goal);

    dbg!(result);

    Ok(())
}

fn construct_graph(memory: &[[usize; MEM_SIZE]; MEM_SIZE]) -> WeightedGraph<Position, usize> {
    let mut graph = WeightedGraph::new();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // Up, Right, Down, Left

    for y in 0..memory.len() {
        for x in 0..memory[0].len() {
            if memory[y][x] == 0 {
                let current = (x, y);
                for (dx, dy) in &directions {
                    let new_x = x as isize + dx;
                    let new_y = y as isize + dy;

                    if new_x >= 0
                        && new_x < memory[0].len() as isize
                        && new_y >= 0
                        && new_y < memory.len() as isize
                        && memory[new_y as usize][new_x as usize] == 0
                    {
                        let neighbor = (new_x as usize, new_y as usize);
                        graph.add_edge(current, neighbor, 1);
                    }
                }
            }
        }
    }

    graph
}

fn heuristic(a: &Position, b: &Position) -> usize {
    let &(x1, y1) = a;
    let &(x2, y2) = b;

    ((x2 as i32 - x1 as i32).abs() + (y2 as i32 - y1 as i32).abs()) as usize
}

fn A_star(
    graph: &WeightedGraph<Position, usize>,
    start: Position,
    goal: Position,
) -> (
    HashMap<Position, Option<Position>>,
    HashMap<Position, usize>,
) {
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<Position, Option<Position>> = HashMap::new();
    let mut cost_so_far: HashMap<Position, usize> = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    heap.push(State {
        position: start,
        priority: 0,
    });

    while let Some(State {
        position: current, ..
    }) = heap.pop()
    {
        if current == goal {
            break;
        }

        if let Some(neighbors) = graph.neighbors(&current) {
            for next in neighbors.keys() {
                // Safely get the edge weight
                if let Some(&weight) = graph.get_weight(&current, next) {
                    let current_cost = cost_so_far.get(&current).copied().unwrap_or(usize::MAX);
                    let new_cost = current_cost + weight;

                    // Safely compare or update cost
                    if !cost_so_far.contains_key(next) || new_cost < *cost_so_far.get(next).unwrap()
                    {
                        cost_so_far.insert(*next, new_cost);
                        let priority = new_cost + heuristic(next, &goal);
                        heap.push(State {
                            position: *next,
                            priority,
                        });
                        came_from.insert(*next, Some(current));
                    }
                }
            }
        }
    }

    (came_from, cost_so_far)
}

fn display_result(
    memory: &[[usize; MEM_SIZE]; MEM_SIZE],
    came_from: &HashMap<Position, Option<Position>>,
    cost_so_far: &HashMap<Position, usize>,
    goal: Position,
) {
    let mut result = vec![vec![' '; memory[0].len() * 2]; memory.len()];

    // Initialize the result grid
    for (y, row) in memory.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 1 {
                result[y][x * 2] = '█'; // Wall
                result[y][x * 2 + 1] = '█'; // Wall
            } else {
                result[y][x * 2] = '⋅'; // Free space
                result[y][x * 2 + 1] = '⋅'; // Free space
            }
        }
    }

    // Mark the path with 'O'
    let mut current = Some(goal);
    while let Some(pos) = current {
        result[pos.1][pos.0 * 2] = 'O';
        result[pos.1][pos.0 * 2 + 1] = 'O';
        current = came_from.get(&pos).cloned().unwrap_or(None);
    }

    // Display the costs
    println!("Cost Grid:");
    for y in 0..memory.len() {
        for x in 0..memory[0].len() {
            if let Some(&cost) = cost_so_far.get(&(x, y)) {
                print!("{:3} ", cost); // Display cost padded with 2 characters
            } else {
                print!("    "); // Empty space for unreachable cells
            }
        }
        println!();
    }

    // Display the path
    println!("\nPath:");
    for row in result {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn get_path_length(came_from: &HashMap<Position, Option<Position>>, goal: Position) -> usize {
    let mut length = 0;
    let mut current = Some(goal);

    // Trace back from goal to start
    while let Some(pos) = current {
        length += 1;
        current = came_from.get(&pos).cloned().unwrap_or(None);
    }

    // If the path exists, subtract 1 to exclude the starting position itself
    if length > 0 {
        length - 1
    } else {
        0 // No path found
    }
}
