use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
extern crate regex;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut map = vec![vec![vec![]; 1000]; 1000];
    let mut non_overlapping_claims: Vec<i32> = Vec::new();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(&file);
    let mut file_content = String::new();
    reader.read_to_string(&mut file_content).ok();
    for line in file_content.lines() {
        for capture in re.captures_iter(line) {
            let id = capture[1].parse::<i32>().unwrap();
            non_overlapping_claims.push(id);
            let pos_x = capture[2].parse::<i32>().unwrap();
            let pos_y = capture[3].parse::<i32>().unwrap();
            let size_x = capture[4].parse::<i32>().unwrap();
            let size_y = capture[5].parse::<i32>().unwrap();
            for x in pos_x..pos_x + size_x {
                for y in pos_y..pos_y + size_y {
                    map[x as usize][y as usize].push(id);
                }
            }
        }
    }
    let mut overlaps = 0;
    for row in map.into_iter() {
        for column in row.into_iter() {
            if column.len() > 1 {
                overlaps += 1;
                for id in column {
                    non_overlapping_claims.retain(|&value| value != id);
                }
            }
        }
    }
    // Solution to part 1
    println!("{}", overlaps);
    // Solution to part 2
    println!("{}", non_overlapping_claims[0]);
    Ok(())
}
