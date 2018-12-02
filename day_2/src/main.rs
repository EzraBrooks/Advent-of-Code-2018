use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(&file);
    let mut lines = String::new();
    let mut repeat_strings: HashMap<i32, i32> = HashMap::new();
    reader.read_to_string(&mut lines).ok();
    for line in lines.as_str().lines() {
        let mut letter_count: HashMap<char, i32> = HashMap::new();
        for character in line.chars() {
            let current_value = letter_count.entry(character).or_insert(0);
            *current_value += 1;
        }
        let deduplicated: HashSet<&i32> = letter_count
            .values()
            .filter(|&&x| x == 2 || x == 3)
            .collect();
        for count in deduplicated {
            let current_value = repeat_strings.entry(*count).or_insert(0);
            *current_value += 1;
        }
    }
    // Answer to part 1
    println!(
        "{}",
        repeat_strings.get(&2).unwrap() * repeat_strings.get(&3).unwrap()
    );
    for line in lines.as_str().lines() {
        for opposite_line in lines.as_str().lines().rev() {
            // If this isn't the same string, compare 'em
            if line != opposite_line {
                let mut difference_count = 0;
                for index in 0..line.len() {
                    if line.get(index..index + 1).unwrap()
                        != opposite_line.get(index..index + 1).unwrap()
                    {
                        difference_count += 1;
                    }
                }
                if difference_count == 1 {
                    println!(
                        "{} and {} have {} difference. The common characters are as follows: ",
                        line, opposite_line, difference_count
                    );
                    // Part 2 solution:
                    for index in 0..line.len() {
                        if line.get(index..index + 1).unwrap()
                            == opposite_line.get(index..index + 1).unwrap()
                        {
                            print!("{}", line.get(index..index + 1).unwrap());
                        }
                    }
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}
