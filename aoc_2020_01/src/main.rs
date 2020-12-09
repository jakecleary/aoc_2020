use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to read file.");
    let reader = BufReader::new(file);
    const TOTAL: i32 = 2020;

    let values: HashSet<i32> = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", get_combo(&values, TOTAL).unwrap());
}

fn get_combo(values: &HashSet<i32>, total: i32) -> Option<i32> {
    for value in values {
        let remainder_needed = total - value;
        if values.contains(&remainder_needed) {
            return Some(value * remainder_needed);
        }
    }
    None
}
