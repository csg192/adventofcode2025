use std::fs;
use std::path::Path;

fn is_invalid_id(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    &s[..half] == &s[half..]
}

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    let mut sum: u128 = 0;

    // Input is one long line of comma-separated ranges
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