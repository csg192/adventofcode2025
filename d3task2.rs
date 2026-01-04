use std::fs;
use std::path::Path;

fn max_joltage_k(bank: &str, k: usize) -> u128 {
    let digits: Vec<u8> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut to_remove = digits.len() - k;

    for &d in &digits {
        while to_remove > 0
            && !stack.is_empty()
            && *stack.last().unwrap() < d
        {
            stack.pop();
            to_remove -= 1;
        }

        stack.push(d);
    }

    // Keep exactly k digits
    stack.truncate(k);

    // Convert digits to number
    let mut value: u128 = 0;
    for d in stack {
        value = value * 10 + d as u128;
    }

    value
}

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    let mut total: u128 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        total += max_joltage_k(line, 12);
    }

    println!("{}", total);
}