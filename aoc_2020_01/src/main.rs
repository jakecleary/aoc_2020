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

    let pair = get_pair(&values, TOTAL).unwrap();
    let trio = get_trio(&values, TOTAL).unwrap();

    println!("Part 1: {}", pair.0 * pair.1);
    println!("Part 2: {}", trio.0 * trio.1 * trio.2);
}

fn get_pair(values: &HashSet<i32>, total: i32) -> Option<(i32, i32)> {
    for value in values {
        let remainder_needed = total - value;
        if values.contains(&remainder_needed) {
            return Some((*value, remainder_needed));
        }
    }
    None
}

fn get_trio(values: &HashSet<i32>, total: i32) -> Option<(i32, i32, i32)> {
    for value in values {
        let remainder_needed = total - value;
        if let Some((first, second)) = get_pair(values, remainder_needed) {
            if first != second && first != remainder_needed && second != remainder_needed {
                return Some((*value, first, second));
            }
        }
    }
    None
}
