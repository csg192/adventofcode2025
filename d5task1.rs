use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    let mut sections = input.split("\n\n");

    let ranges_section = sections.next().expect("Missing ranges section");
    let ids_section = sections.next().expect("Missing IDs section");

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in ranges_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (start_str, end_str) = line
            .split_once('-')
            .expect("Invalid range format");

        let start: u64 = start_str.parse().unwrap();
        let end: u64 = end_str.parse().unwrap();

        ranges.push((start, end));
    }

    let mut fresh_count = 0;

    for line in ids_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let id: u64 = line.parse().unwrap();

        if ranges.iter().any(|&(start, end)| id >= start && id <= end) {
            fresh_count += 1;
        }
    }

    println!("{}", fresh_count);
}
