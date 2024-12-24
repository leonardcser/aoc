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

fn process_file(file_path: &str) -> io::Result<()> {
    let content: String = fs::read_to_string(file_path)?;
    let mut sections = content.trim().split("\n\n");

    let mut deps: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut computed: HashMap<&str, bool> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(": ");
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap() != 0,
            )
        })
        .collect();

    sections.next().unwrap().lines().for_each(|l| {
        let parts = l.split_whitespace().collect::<Vec<_>>();
        assert!(!deps.contains_key(&parts[4]));
        deps.insert(parts[4], (parts[0], parts[2], parts[1]));
    });

    for dep in deps.clone().into_keys() {
        compute_res(&mut computed, &mut deps.clone(), dep);
    }

    let mut zs: Vec<&str> = computed
        .clone()
        .into_keys()
        .filter(|k| k.starts_with("z"))
        .collect();
    zs.sort();

    let result = zs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, z)| acc | (computed[z] as usize) << i);

    dbg!(result);
    Ok(())
}

fn compute_res<'a>(
    computed: &mut HashMap<&'a str, bool>,
    deps: &mut HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    key: &'a str,
) {
    if deps.len() == 0 {
        return;
    }

    let dep = deps[key];
    if !computed.contains_key(dep.0) {
        compute_res(computed, deps, dep.0)
    }
    if !computed.contains_key(dep.1) {
        compute_res(computed, deps, dep.1)
    }
    deps.remove(key);

    let a = computed[dep.0];
    let b = computed[dep.1];
    let res = match dep.2 {
        "OR" => a | b,
        "AND" => a & b,
        "XOR" => a ^ b,
        _ => unreachable!(),
    };
    computed.insert(key, res);
}
