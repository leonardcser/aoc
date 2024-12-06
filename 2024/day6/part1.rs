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
    let lines: Vec<Vec<char>> = read_lines(file_path)?
        .filter_map(Result::ok)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut x = 0;
    let mut y = 0;
    let w = lines[0].len();
    let h = lines.len();

    // Find start position
    'found: for j in 0..h {
        for i in 0..w {
            let c = lines[j][i];
            if c == '^' {
                x = i;
                y = j;
                break 'found;
            }
        }
    }

    let mut visited = vec![vec![false; w]; h];
    visited[y][x] = true;

    let mut dir = vec![0, -1];
    loop {
        let tmp_x = (x as i32 + dir[0]) as usize;
        let tmp_y = (y as i32 + dir[1]) as usize;
        if lines[tmp_y][tmp_x] == '#' {
            dir[1] *= -1;
            dir.swap(1, 0);
            x = (x as i32 + dir[0]) as usize;
            y = (y as i32 + dir[1]) as usize;
        } else {
            x = tmp_x;
            y = tmp_y;
        }
        visited[y][x] = true;

        if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
            break;
        }
    }

    let result = visited
        .iter()
        .flat_map(|row| row.iter().collect::<Vec<_>>())
        .filter(|&&value| value)
        .count();

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
