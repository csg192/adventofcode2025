use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");

    let mut input = fs::read_to_string(path)
        .expect("Failed to read file");
    

        let mut position: i32 = 50;
        let mut zero_count: i32 = 0;

        for line in input.lines(){

        if line.trim().is_empty(){
            continue;
        }
        let (dir, value_str) = line.split_at(1);
        let value: i32 = value_str.parse().expect("Failed to parse value");

        match dir {
            "L" =>{
                position = (position - value).rem_euclid(100);
            }
            "R" => {
                position = (position + value).rem_euclid(100);
            }
            _ => panic!("Invalid direction: {}", dir),
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    println!("{}", zero_count);
}