use std::{fs, io::BufReader};
use std::collections::HashSet;
use std::io::BufRead;

const PATH1: &str = "./src/input_1.txt";
const PATH2: &str = "./src/input_2.txt";

fn main() {
    let priority_1 = count_priority_part_1();
    let priority_2 = count_priority_part_2();
    println!("{} {}", priority_1, priority_2);
}

fn count_priority_part_1() -> usize {
    BufReader::new(fs::File::open(PATH1).unwrap())
        .lines()
        .map(|x| {
            let value = x.unwrap();
            let half = value.len() / 2;
            let to_set = |x: &str| x.chars().collect::<HashSet<char>>();
            (to_set(&value[..half]), to_set(&value[half..]))
        })
        .fold(0, |acc, x| {
            let priority = x.0.intersection(&x.1).next().unwrap();
            acc + *priority as usize - (if priority.is_lowercase() { 96 } else { 38 })
        })
}

fn count_priority_part_2() -> usize {
    BufReader::new(fs::File::open(PATH2).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|x| {
            let to_set = |x: &str| x.chars().collect::<HashSet<char>>();
            (to_set(&x[0]), to_set(&x[1]), to_set(&x[2]))
        })
        .fold(0, |acc, x| {
            let priority = x.0
                .iter()
                .filter(|c| x.1.contains(c) && x.2.contains(c))
                .next()
                .unwrap();

            acc + *priority as usize - (if priority.is_lowercase() { 96 } else { 38 })
        })
}