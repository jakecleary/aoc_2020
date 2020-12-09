use std::fs::File;
use std::collections::HashSet;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to read file.");
    let reader = BufReader::new(file);
    const TOTAL: i32 = 2020;

    let values: HashSet<i32> = reader.lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    for value in &values {
        let remainder_needed = TOTAL - value;
        if values.contains(&remainder_needed) {
            println!("{}", value * remainder_needed);
            break;
        }
    }
}
