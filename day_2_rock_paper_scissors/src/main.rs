use std::{fs, io::BufReader};
use std::io::BufRead;

const PATH: &str = "./src/input.txt";
const DRAW_POINTS: usize = 3;
const WIN_POINTS: usize = 6;

fn main() {
    let total_points = count_total_points();
    println!("{}", total_points);
}

fn count_total_points() -> usize {
    let file = fs::File::open(PATH).unwrap();
    let reader = BufReader::new(file);
    let mut total_points = 0;

    let to_digit = |x: &str, pos: usize| x.chars().next().unwrap() as usize - pos;

    reader.lines().for_each(|line| {
        let (digit1, digit2) = line.unwrap().split_once(' ')
            .map(|x| (to_digit(x.0, 64), to_digit(x.1, 87)))
            .unwrap();

        if digit2 == 1 {
            total_points += if digit1 == 1 { 3 } else { digit1 - 1 };
            return;
        }

        if digit2 == 2 {
            total_points += digit1 + DRAW_POINTS;
            return;
        }

        if digit2 == 3 {
            total_points += if digit1 == 3 { 1 } else { digit1 + 1 } + WIN_POINTS;
            return;
        }
    });

    total_points
}