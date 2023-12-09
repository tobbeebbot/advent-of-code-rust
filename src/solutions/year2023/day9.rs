#![allow(unused)]

use itertools::Itertools;

fn next_number(numbers: Vec<i64>) -> i64 {
    if numbers.iter().all(|&i| i == 0) {
        0
    } else {
        let last = numbers.last().expect("all() is true if list empty");
        last + next_number(
            numbers
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec(),
        )
    }
}

fn prev_number(numbers: Vec<i64>) -> i64 {
    let numbers = numbers.into_iter().rev().collect();
    next_number(numbers)
}

fn parse_num_vecs(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("is number"))
                .collect_vec()
        })
        .collect_vec()
}

pub fn solve_part1(input: &str) -> String {
    parse_num_vecs(input)
        .into_iter()
        .map(next_number)
        .sum::<i64>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    parse_num_vecs(input)
        .into_iter()
        .map(prev_number)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod test_day9 {
    use super::*;

    #[test]
    fn test_part1_a() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = 18;
        assert_eq!(expected, next_number(input))
    }
    #[test]
    fn test_part1_b() {
        let input = vec![1, 3, 6, 10, 15, 21];
        let expected = 28;
        assert_eq!(expected, next_number(input))
    }
    #[test]
    fn test_part1_c() {
        let input = vec![10, 13, 16, 21, 30, 45];
        let expected = 68;
        assert_eq!(expected, next_number(input))
    }
    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = "114";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let expected = "2";
        assert_eq!(expected, solve_part2(input))
    }
}
