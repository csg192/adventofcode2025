use std::fs;
use std::path::Path;

fn is_invalid_id(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths
    for p in 1..=len / 2 {
        if len % p != 0 {
            continue;
        }

        let pattern = &s[..p];
        let repeats = len / p;

        let mut valid = true;
        for i in 1..repeats {
            if &s[i * p..(i + 1) * p] != pattern {
                valid = false;
                break;
            }
        }

        if valid {
            return true;
        }
    }

    false
}

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    let mut sum: u128 = 0;

    // One long comma-separated line of ranges
    for range in input.trim().split(',') {
        let (start_str, end_str) = range
            .split_once('-')
            .expect("Invalid range format");

        let start: u128 = start_str.parse().unwrap();
        let end: u128 = end_str.parse().unwrap();

        for id in start..=end {
            if is_invalid_id(id) {
                sum += id;
            }
        }
    }

    println!("{}", sum);
}