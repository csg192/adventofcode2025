use std::fs;
use std::path::Path;

fn max_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit"))
        .collect();

    let mut best = 0;

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let value = digits[i] * 10 + digits[j];
            if value > best {
                best = value;
            }
        }
    }

    best
}

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    let mut total: u32 = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        total += max_joltage(line.trim());
    }

    println!("{}", total);
}