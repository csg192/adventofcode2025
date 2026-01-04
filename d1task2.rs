use std::fs;
use std::path::Path;

fn main (){
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input");

    let mut position: i32 = 50;
    let mut zero_count: i32 = 0;

    for line in input.lines() {
        if line.trim().is_empty(){
            continue;
        }

        let (dir, value_str) = line.split_at(1);
        let distance: i32 = value_str.parse().expect("Invalid_number");


        match dir {
            "R" => {
                let mut first_hit = (100 - position) %100;

                if first_hit == 0 {
                    first_hit = 100;
                }

                if first_hit <= distance {
                    zero_count += 1 + (distance - first_hit)/100;
                }
                
                position = (position + distance).rem_euclid(100); 

                }
            "L" => {
                let mut first_hit = position %100;
                if first_hit == 0 {
                    first_hit = 100;
                }

                if first_hit <= distance {
                    zero_count += 1 + (distance-first_hit)/100;
                }

                position = (position - distance).rem_euclid(100);
            }
            _ => panic!("Invalid direction"),
        
        }
    }
    println!("{}", zero_count)

}