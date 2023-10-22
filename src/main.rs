
mod solutions;

use std::fs;

// https://adventofcode.com/2016/

fn main() {
    let problem = "day3";

    let file_path = format!("inputs/{problem}.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let answer = solutions::day3::solve_part1(&contents);
    println!("Solitions to {problem} is: \n {answer}");

    let answer = solutions::day3::solve_part2(&contents);
    println!("Solitions to {problem} is: \n {answer}");
}