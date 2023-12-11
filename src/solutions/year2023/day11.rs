#![allow(unused)]

use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Star {
    x: usize,
    y: usize,
}

impl Star {
    fn distance_to(&self, other: &Star) -> usize {
        (self.x as i64 - other.x as i64).unsigned_abs() as usize
            + (self.y as i64 - other.y as i64).unsigned_abs() as usize
    }
}

fn parse_sky(input: &str) -> BTreeSet<Star> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().flat_map(move |(x, c)| match c {
                '#' => Some(Star { x, y }),
                _ => None,
            })
        })
        .collect::<BTreeSet<Star>>()
}

fn calc_distance(input: &str, expansion: usize) -> usize {
    let stars = parse_sky(input);
    let mut visited_x = BTreeSet::new();
    let mut visited_y = BTreeSet::new();

    stars.iter().for_each(|Star { x, y }| {
        visited_x.insert(x);
        visited_y.insert(y);
    });

    let expansion = expansion - 1;
    let expansion_x = visited_x
        .into_iter()
        .enumerate()
        .map(|(i, x)| (*x, (*x - i) * expansion))
        .collect::<BTreeMap<usize, usize>>();
    let expansion_y = visited_y
        .into_iter()
        .enumerate()
        .map(|(i, y)| (*y, (*y - i) * expansion))
        .collect::<BTreeMap<usize, usize>>();

    let expanded_stars = stars
        .into_iter()
        .map(|Star { x, y }| Star {
            x: x + expansion_x.get(&x).unwrap(),
            y: y + expansion_y.get(&y).unwrap(),
        })
        .collect_vec();

    expanded_stars
        .iter()
        .cartesian_product(expanded_stars.iter())
        .map(|(s1, s2)| s1.distance_to(s2))
        .sum::<usize>()
        / 2
}

pub fn solve_part1(input: &str) -> String {
    calc_distance(input, 2).to_string()
}

pub fn solve_part2(input: &str) -> String {
    calc_distance(input, 1_000_000).to_string()
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected = "374";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2_10() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected = 1030;
        assert_eq!(expected, calc_distance(input, 10))
    }

    #[test]
    fn test_part2_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected = 8410;
        assert_eq!(expected, calc_distance(input, 100))
    }
}
