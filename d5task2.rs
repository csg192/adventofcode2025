use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    // Only the first section (ranges) matters
    let ranges_section = input
        .split("\n\n")
        .next()
        .expect("Missing ranges section");

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

    // Sort ranges by starting point
    ranges.sort_by_key(|&(start, end)| (start, end));

    let mut total: u128 = 0;

    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in ranges.iter().skip(1) {
        if start <= current_end + 1 {
            // Overlapping or touching ranges → merge
            if end > current_end {
                current_end = end;
            }
        } else {
            // Disjoint range → finalize previous
            total += (current_end - current_start + 1) as u128;
            current_start = start;
            current_end = end;
        }
    }

    // Final range
    total += (current_end - current_start + 1) as u128;

    println!("{}", total);
}
