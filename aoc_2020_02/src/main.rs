use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buffer = BufReader::new(file);
    let lines: HashSet<String> = buffer.lines().map(|l| l.unwrap()).collect();
    let regex = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();
    let valid_lines = lines.iter().filter(|line| regex.is_match(line));
    let valid_passwords = valid_lines
        .map(|line| unwrap_captures(regex.captures(line).unwrap()))
        .filter(|config| is_valid_password(*config));
    println!("Count: {}", valid_passwords.count());
}

fn unwrap_captures(capture: regex::Captures) -> (char, usize, usize, &str) {
    let min_count = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max_count = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = capture.get(3).unwrap().as_str().chars().next().unwrap();
    let password = capture.get(4).unwrap().as_str();
    (letter, min_count, max_count, password)
}

fn is_valid_password(config: (char, usize, usize, &str)) -> bool {
    let (letter, min_count, max_count, password) = config;
    let count = password.chars().filter(|character| character == &letter).count();
    count >= min_count && count <= max_count
}
