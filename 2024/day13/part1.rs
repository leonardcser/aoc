use std::env;
use std::fs;
use std::io;
use std::process::exit;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64), // (X movement, Y movement)
    button_b: (i64, i64), // (X movement, Y movement)
    prize: (i64, i64),    // (X, Y)
}

#[derive(Debug)]
struct Solution {
    a_presses: i64,
    b_presses: i64,
    total_tokens: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    match process_file(&args[1]) {
        Ok(result) => {
            println!("Maximum Prizes Won: {}", result.len());
            println!(
                "Total Minimum Tokens: {}",
                result.iter().map(|s| s.total_tokens).sum::<i64>()
            );
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}

fn process_file(file_path: &str) -> io::Result<Vec<Solution>> {
    let file_content = fs::read_to_string(file_path)?;
    let machine_blocks = file_content.split("\n\n");
    let mut winnable_machines = Vec::new();

    for block in machine_blocks {
        let mut button_a = (0, 0);
        let mut button_b = (0, 0);
        let mut prize = (0, 0);

        for line in block.lines() {
            if line.starts_with("Button A:") {
                if let Some((x, y)) = parse_movements(&line[9..]) {
                    button_a = (x, y);
                }
            } else if line.starts_with("Button B:") {
                if let Some((x, y)) = parse_movements(&line[9..]) {
                    button_b = (x, y);
                }
            } else if line.starts_with("Prize:") {
                if let Some((x, y)) = parse_movements(&line[6..]) {
                    prize = (x, y);
                }
            }
        }

        let machine = ClawMachine {
            button_a,
            button_b,
            prize,
        };

        if let Some(solution) = solve_machine(&machine) {
            winnable_machines.push(solution);
        }
    }

    Ok(winnable_machines)
}

fn parse_movements(input: &str) -> Option<(i64, i64)> {
    let parts: Vec<&str> = input.trim().split(',').collect();
    if parts.len() == 2 {
        let x_part = parts[0].trim();
        let y_part = parts[1].trim();

        let x = x_part
            .strip_prefix("X+")
            .or_else(|| x_part.strip_prefix("X="))
            .and_then(|v| v.parse::<i64>().ok());

        let y = y_part
            .strip_prefix("Y+")
            .or_else(|| y_part.strip_prefix("Y="))
            .and_then(|v| v.parse::<i64>().ok());

        if let (Some(x), Some(y)) = (x, y) {
            return Some((x, y));
        }
    }
    None
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extended_gcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}

fn solve_machine(machine: &ClawMachine) -> Option<Solution> {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (px, py) = machine.prize;

    // X-axis solution
    let (gcd_x, _mx, _my) = extended_gcd(ax, bx);
    if px % gcd_x != 0 {
        return None;
    }

    // Y-axis solution
    let (gcd_y, _ma, _mb) = extended_gcd(ay, by);
    if py % gcd_y != 0 {
        return None;
    }

    // Brute force search for solution
    for a_presses in 0..=100 {
        for b_presses in 0..=100 {
            if a_presses * ax + b_presses * bx == px && a_presses * ay + b_presses * by == py {
                return Some(Solution {
                    a_presses,
                    b_presses,
                    total_tokens: a_presses * 3 + b_presses,
                });
            }
        }
    }

    None
}
