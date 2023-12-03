#![allow(unused)]

use itertools::enumerate;
use nom::FindSubstring;

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|char| char.to_digit(10));

            let first_digit = digits.next();
            let last_digit = digits.last();

            let first_digit = first_digit.expect("There needs to be at least one number");
            let last_digit = last_digit.unwrap_or(first_digit);

            first_digit * 10 + last_digit
        })
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    fn find_digit(rline: &str) -> Option<u32> {
        let string_digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let num_digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
        for (num, digit) in enumerate(string_digits) {
            if rline.starts_with(digit) {
                return Some(num as u32 + 1);
            }
        }
        for (num, digit) in enumerate(num_digits) {
            if rline.starts_with(digit) {
                return Some(num as u32 + 1);
            }
        }
        None
    }

    input
        .lines()
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|i| {
                let rline = &line[i..];
                find_digit(rline)
            });

            let first_digit = digits.next();
            let last_digit = digits.last();

            let first_digit = first_digit.expect("There needs to be at least one number");
            let last_digit = last_digit.unwrap_or(first_digit);

            first_digit * 10 + last_digit
        })
        .sum()
}

#[cfg(test)]
mod test_day1 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected = 142;
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let expected = 281;
        assert_eq!(expected, solve_part2(input))
    }
}
