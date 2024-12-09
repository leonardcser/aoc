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
    let mut nums: Vec<usize> = fs::read_to_string(file_path)?
        .lines()
        .next()
        .expect("File is empty")
        .chars()
        .map(|c| c.to_digit(10).expect("Not a digit") as usize)
        .collect();

    let size: usize = nums.clone().into_iter().sum();
    let mut ids: Vec<i16> = (0..nums.len() / 2 + 1)
        .flat_map(|id| vec![id as i16, -1])
        .collect();
    // 0x1x2x3x4x5x6x7x8x9
    // 0x9x1x2x3x4x5x6x7x8x9
    // 2333133121414131402
    // 2021331331214141314002
    //    ^                 ^
    // 00...111...2...333.44.5555.6666.777.888899
    // 23
    // 2021
    // 2
    // 02
    let mut l = 1;
    let mut r = nums.len() - 1;
    if nums.len() % 2 == 0 {
        r -= 1;
    }
    while l < r {
        let mut tmp_l = l;
        while tmp_l <= r && r - tmp_l > 1 {
            print_storage(&ids);
            print_nums(&nums);
            dbg!(tmp_l, r, nums[tmp_l], nums[r]);
            if nums[tmp_l] == nums[r] {
                println!("PERFECT");
                // Fits perfectly

                nums[tmp_l] = nums[r];
                nums.insert(tmp_l, 0);
                nums.remove(r + 1);
                nums.remove(tmp_l + 2);
                nums[tmp_l + 2] = 0;

                ids.insert(tmp_l, ids[r]);
                ids.insert(tmp_l, -1);
                ids[r + 2] = -1;
            } else if nums[tmp_l] > nums[r] {
                println!("FIT");
                // Fits but need to update nums
                ids.insert(tmp_l, -1);
                ids.insert(tmp_l + 1, ids[r + 1]);
                ids[r + 2] = -1;

                let remainder = nums[tmp_l] - nums[r];
                nums.insert(tmp_l, 0);
                tmp_l += 1;
                r += 1;
                nums[tmp_l] = nums[r];
                nums.insert(r, 0);
                tmp_l += 1;
                nums.insert(tmp_l, remainder);
                tmp_l -= 2;
                r -= 1;
            }
            while r > 0 && nums[r] == 0 {
                r -= 2;
            }
            // print_storage(&ids);
            // print_nums(&nums);
            // dbg!(tmp_l, r, nums[tmp_l], nums[r]);
            // let mut buffer = String::new();
            // let stdin = io::stdin(); // We get `Stdin` here.
            // stdin.read_line(&mut buffer)?;
            tmp_l += 2;
        }
        r -= 2;
        while r > 0 && nums[r] == 0 {
            r -= 2;
        }
    }

    ids = ids.into_iter().filter(|&x| x >= 0).collect();
    let mut storage: Vec<i16> = vec![-1; size];

    let mut id = 0;
    let mut s_idx = 0;
    for i in 0..nums.len() {
        let d = nums[i];
        if i % 2 == 0 {
            for _ in 0..d {
                storage[s_idx] = ids[id];
                s_idx += 1;
            }
            id += 1;
            if id >= ids.len() {
                break;
            }
        } else {
            s_idx += d;
        }
    }

    print_storage(&storage);

    let mut checksum: u64 = 0;
    for i in 0..size {
        if storage[i] == -1 {
            continue;
        }
        checksum += storage[i] as u64 * i as u64;
    }
    dbg!(checksum);
    Ok(())
}

fn print_nums(nums: &Vec<usize>) {
    for n in nums {
        print!("{n}");
    }
    println!("");
}
fn print_storage(storage: &Vec<i16>) {
    for s in storage {
        if *s == -1 {
            print!(".");
        } else {
            print!("{s}");
        }
    }
    println!("");
}
