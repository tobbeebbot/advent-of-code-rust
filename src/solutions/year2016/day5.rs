#![allow(unused)]

// https://adventofcode.com/2016/day/5

use std::collections::HashSet;

use itertools::{Itertools, WithPosition};
use nom::HexDisplay;

pub fn solve_part1(door_id: &str) -> String {
    (0..)
        .map(|idx| {
            let input = format!("{}{}", door_id, idx);
            let digest = md5::compute(input);
            format!("{:x}", digest)
        })
        .filter_map(|hash| {
            hash.starts_with("00000")
                .then_some(hash.chars().nth(5))
                .flatten()
        })
        .take(8)
        .join("")
}

pub fn solve_part2(door_id: &str) -> String {
    let mut seen_positions = HashSet::new();

    (0..)
        .map(|idx| {
            // Create hash
            let input = format!("{}{}", door_id, idx);
            let digest = md5::compute(input);
            format!("{:x}", digest)
        })
        .filter(|hash| {
            // Initial zeroes check
            hash.starts_with("00000")
        })
        .filter_map(|hash| {
            // Create entries of the valid ones
            let char_pos = hash.chars().nth(5)?;
            let character = hash.chars().nth(6)?;

            "01234567"
                .find(char_pos)
                .and_then(|_| Some((char_pos, character)))
        })
        .filter_map(|t| {
            // Remove duplicates
            if seen_positions.insert(t.0) {
                Some(t)
            } else {
                None
            }
        })
        .take(8)
        .inspect(|s| println!("New unique found: {:?}", s))
        .sorted_by(|(pos, _), (pos2, _)| Ord::cmp(pos, pos2))
        .map(|t| t.1)
        .join("")
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    #[test]
    fn test_part1() {
        let door_name = "abc";
        let pass_code = "18f47a30";
        assert_eq!(pass_code, solve_part1(door_name))
    }

    #[test]
    fn test_part1_on_puzzle_input() {
        let door_name = "reyedfim";
        println!("{}", solve_part1(door_name));
    }

    #[test]
    fn test_part2() {
        let input = "abc";
        let expected = "05ace8e3";
        assert_eq!(expected, solve_part2(input));
    }

    #[test]
    fn test_part2_on_puzzle_input() {
        let door_name = "reyedfim";
        println!("{}", solve_part2(door_name));
    }
}
