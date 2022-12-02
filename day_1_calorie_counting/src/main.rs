use std::{fs, io::BufReader};
use std::io::BufRead;

const PATH: &str = "./src/elves.txt";

fn main() {
    let max_calories = find_elf_carrying_most_calories();
    let max_calories_iter = find_elf_carrying_most_calories_iter();
    println!("{}, {}", max_calories, max_calories_iter);
    assert_eq!(max_calories, max_calories_iter)
}

fn find_elf_carrying_most_calories() -> usize {
    let file = fs::File::open(PATH).unwrap();
    let reader = BufReader::new(file);

    let mut all_calories = Vec::<usize>::new();
    let mut max_calories = 0;
    let mut current_total_calories = 0;

    for line in reader.lines() {
        let value = line.unwrap();
        if value == "" {
            if current_total_calories > max_calories {
                max_calories = current_total_calories;
            }

            all_calories.push(current_total_calories);
            current_total_calories = 0;
            continue;
        }

        let calories = value.parse::<usize>().unwrap();
        current_total_calories += calories;
    }

    // Add last part which isn't ending with "\n"
    all_calories.push(current_total_calories);
    all_calories.sort();
    all_calories.iter().rev().take(3).sum()
}


fn find_elf_carrying_most_calories_iter() -> usize {
    let mut all_calories = BufReader::new(fs::File::open(PATH).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .split(|x| x.is_empty())
        .map(|x| x.iter().map(|y| y.parse::<usize>().unwrap()).sum::<usize>())
        .collect::<Vec<usize>>();

    all_calories.sort();
    all_calories.iter().rev().take(3).sum()
}