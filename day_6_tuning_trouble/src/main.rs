use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};

const PATH: &str  = "./src/input.txt";

fn main() {
    let result1 = count_characters(4);
    let result2 = count_characters(14);
    println!("{result1} {result2}")
}

fn count_characters(size: usize) -> usize {
    fs::read_to_string(PATH)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .map(|x| x.iter().collect::<HashSet<&char>>())
        .take_while(|x| x.len() != size)
        .fold(size, |acc, _| {
            acc + 1
        })
}