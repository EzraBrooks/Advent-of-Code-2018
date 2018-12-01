use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);
    let mut frequency = 0;
    for line in reader.lines() {
        let parsed = line.unwrap().to_string().parse::<i32>().unwrap();
        frequency += parsed;
    }
    // Answer for part 1
    println!("Final frequency: {}", frequency);
    find_first_duplicate_frequency()
}

// Part 2 solution (pretty brute-forced but ¯\_(ツ)_/¯)
fn find_first_duplicate_frequency() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(&file);
    let mut frequency = 0;
    let mut frequency_history: HashMap<i32, bool> = HashMap::new();
    let mut lines = String::new();
    reader.read_to_string(&mut lines).ok();
    loop {
        for line in lines.as_str().lines() {
            let parsed = line.to_string().parse::<i32>().unwrap();
            frequency += parsed;
            if frequency_history.contains_key(&frequency) {
                // Answer for part 2
                println!("Found duplicate frequency: {}", frequency);
                return Ok(());
            } else {
                frequency_history.insert(frequency, true);
            }
        }
    }
}
