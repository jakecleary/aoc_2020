use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buffer = BufReader::new(file);
    let lines: HashSet<String> = buffer.lines().map(|l| l.unwrap()).collect();
    let regex = Regex::new(r"^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)$").unwrap();
    let parsed_lines: HashSet<(char, usize, usize, &str)> = lines.iter()
        .filter(|line| regex.is_match(line))
        .map(|line| unwrap_captures(regex.captures(line).unwrap()))
        .collect();
    let valid_sled_shop_passwords = parsed_lines
        .iter().filter(|config| is_valid_sled_rental_shop_password(config));
    let valid_nptrs_passwords = parsed_lines
        .iter().filter(|config| is_valid_nptrs_password(config));
    println!("Valid Sled Shop passwords: {}", valid_sled_shop_passwords.count());
    println!("Valid NPTRS passwords: {}", valid_nptrs_passwords.count());
}

fn unwrap_captures(capture: regex::Captures) -> (char, usize, usize, &str) {
    let min_count = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let max_count = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let letter = capture.get(3).unwrap().as_str().chars().next().unwrap();
    let password = capture.get(4).unwrap().as_str();
    (letter, min_count, max_count, password)
}

fn is_valid_sled_rental_shop_password(config: &(char, usize, usize, &str)) -> bool {
    let (letter, min_count, max_count, password) = *config;
    let count = password.chars().filter(|character| character == &letter).count();
    count >= min_count && count <= max_count
}

fn is_valid_nptrs_password(config: &(char, usize, usize, &str)) -> bool {
    let (letter, position_1, position_2, password) = *config;
    let position_1_valid = password.chars().nth(position_1 - 1) == Some(letter);
    let position_2_valid = password.chars().nth(position_2 - 1) == Some(letter);
    let neither_valid = !position_1_valid && !position_2_valid;
    let both_valid = position_1_valid && position_2_valid;
    (!neither_valid && !both_valid) && (position_1_valid || position_2_valid)
}
