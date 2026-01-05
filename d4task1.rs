use std::fs;
use std::path::Path;

fn main(){
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).expect("Failed to read input.txt");

    let grid: Vec<Vec<char>> = input.lines().filter(|l| !l.trim().is_empty()).map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [
        (-1,-1),(-1,0),(-1,1),
        (0,-1),        (0,1),
        (1,-1), (1,0), (1,1),
    ];

    let mut accessible =0;

    for r in 0..rows{
        for c in 0..cols{
            if grid [r][c] != '@'{
                continue
            }

            let mut neighbors = 0;

            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 
                && nr < rows as isize
                && nc >= 0
                && nc < cols as isize
                && grid[nr as usize][nc as usize] == '@'
                {
                    neighbors +=1;
                }
            }

            if neighbors < 4 {
                accessible +=1;
            }
        }

    }

    println!("{}", accessible);
}