use std::{fs, io::BufReader};
use std::io::BufRead;

const PATH: &str = "./src/input.txt";

fn main() {
    let fully_contained_parts = count_fully_contained_pairs();
    let overlapping_pairs = count_overlapping_pairs();
    println!("{} {}", fully_contained_parts, overlapping_pairs);
}

fn count_fully_contained_pairs() -> usize {
    BufReader::new(fs::File::open(PATH).unwrap())
        .lines()
        .map(|x| {
            fn range(value: &str) -> (usize, usize) {
                value.split_once('-')
                    .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                    .unwrap()
            }

            let value = x.unwrap();
            let (first, second) = value.split_once(',').unwrap();
            (range(&first), range(&second))
        })
        .fold(0, |acc, (first, second)| {
            acc + (if first.0 <= second.0 && first.1 >= second.1 ||
                second.0 <= first.0 && second.1 >= first.1 { 1 } else { 0 })
        })
}

fn count_overlapping_pairs() -> usize {
    BufReader::new(fs::File::open(PATH).unwrap())
        .lines()
        .map(|x| {
            fn range(value: &str) -> (usize, usize) {
                value.split_once('-')
                    .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                    .unwrap()
            }

            let value = x.unwrap();
            let (first, second) = value.split_once(',').unwrap();
            (range(&first), range(&second))
        })
        .fold(0, |acc, (first, second)| {
            acc + (if first.0 <= second.0 && first.1 >= second.0 ||
                second.0 <= first.0 && second.1 >= first.0 { 1 } else { 0 })
        })
}