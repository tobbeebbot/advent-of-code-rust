
mod solutions;

use std::fs;

// https://adventofcode.com/2016/

fn main() {
    let day = "day8";

    let file_path = format!("inputs/{day}.txt");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let answer = solutions::year2023::day8::solve_part1(&contents);
    println!("Solition to {day} part1 is: \n{answer}");

    let answer = solutions::year2023::day8::solve_part2(&contents);
    println!("Solition to {day} part2 is: \n{answer}");
}