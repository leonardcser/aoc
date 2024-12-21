use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::VecDeque;
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

fn precompute_paths(
    keypad: Vec<Vec<char>>,
    dirs: Vec<(isize, isize, char)>,
) -> HashMap<(char, char), String> {
    let mut rng = thread_rng();

    let mut key_positions = HashMap::new();
    for (row, line) in keypad.iter().enumerate() {
        for (col, &key) in line.iter().enumerate() {
            if key != ' ' {
                key_positions.insert(key, (row as isize, col as isize));
            }
        }
    }

    let mut paths = HashMap::new();

    for (&start_key, &start_pos) in &key_positions {
        for (&end_key, &end_pos) in &key_positions {
            if start_key == end_key {
                paths.insert((start_key, end_key), "A".into());
                continue;
            }

            // BFS to find the shortest path
            let mut queue = VecDeque::new();
            queue.push_back((start_pos, String::new()));
            let mut visited = HashMap::new();
            visited.insert(start_pos, true);

            while let Some((current_pos, path)) = queue.pop_front() {
                if current_pos == end_pos {
                    paths.insert((start_key, end_key), format!("{}A", path));
                    break;
                }

                let mut rand_dirs = dirs.clone();
                rand_dirs.shuffle(&mut rng);
                for &(dr, dc, dir) in &dirs {
                    let next_pos = (current_pos.0 + dr, current_pos.1 + dc);
                    if key_positions.values().any(|&pos| pos == next_pos)
                        && !visited.contains_key(&next_pos)
                    {
                        visited.insert(next_pos, true);
                        let mut next_path = path.clone();
                        next_path.push(dir);
                        queue.push_back((next_pos, next_path));
                    }
                }
            }
        }
    }

    paths
}

fn resolve_paths(
    nums_paths: &HashMap<(char, char), String>,
    dirs_paths: &HashMap<(char, char), String>,
    input: &String,
) -> String {
    let mut result = String::new();
    let mut curr1 = 'A';
    let mut curr2 = 'A';
    let mut curr3 = 'A';

    for ch1 in input.chars() {
        // Get the path from `nums_paths`
        let p1 = &nums_paths[&(curr1, ch1)];
        for ch2 in p1.chars() {
            // Get the path from `dirs_paths`
            let p2 = &dirs_paths[&(curr2, ch2)];

            for ch3 in p2.chars() {
                // Resolve the final step
                let p3 = &dirs_paths[&(curr3, ch3)];
                result.push_str(p3);
                curr3 = ch3;
            }

            curr2 = ch2;
        }

        curr1 = ch1;
    }

    result
}
fn process_file(file_path: &str) -> io::Result<()> {
    let lines: Vec<String> = fs::read_to_string(file_path)?
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut result = 0;
    for line in lines.iter() {
        let mut min_len = usize::MAX;
        for _ in 0..100 {
            let nums_paths = precompute_paths(
                vec![
                    vec!['7', '8', '9'],
                    vec!['4', '5', '6'],
                    vec!['1', '2', '3'],
                    vec![' ', '0', 'A'],
                ],
                vec![(-1, 0, '^'), (1, 0, 'v'), (0, -1, '<'), (0, 1, '>')],
            );

            let dirs_paths = precompute_paths(
                vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']],
                vec![(-1, 0, '^'), (1, 0, 'v'), (0, -1, '<'), (0, 1, '>')],
            );
            let p = resolve_paths(&nums_paths, &dirs_paths, line);
            if p.len() < min_len {
                min_len = p.len();
            }
        }
        let num = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        println!("{} * {}", min_len, num);
        result += min_len * num;
    }
    dbg!(result);
    Ok(())
}
