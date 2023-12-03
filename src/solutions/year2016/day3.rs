#![allow(unused)]

use itertools::Itertools;

fn valid_triangle(input: &str) -> bool {
    let sides = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let valid = sides[0] < sides[1] + sides[2]
        && sides[1] < sides[0] + sides[2]
        && sides[2] < sides[0] + sides[1];

    return valid;
}

pub fn solve_part1(input: &str) -> u32 {
    input.lines().filter(|&line| valid_triangle(line)).count() as u32
}

pub fn solve_part2(input: &str) -> u32 {

    let triplets = input
        .lines()
        .enumerate()
        .map(|(idx, row)| (idx / 3, row))
        .into_group_map();

    let valid_triangles = triplets.into_iter().map(|(_, t)| {
        let t = t.join(" ");

        let triangles = [t.split_ascii_whitespace().step_by(3).join(" "),
                                      t.split_ascii_whitespace().skip(1).step_by(3).join(" "),
                                      t.split_ascii_whitespace().skip(2).step_by(3).join(" ")];

        triangles.into_iter().filter(|triangle| valid_triangle(triangle)).count() as u32

    }).sum::<u32>();
  
    valid_triangles
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    #[test]
    fn test_single_triangle() {
        let input = "4 6 32";
        let expected = false;
        assert_eq!(expected, valid_triangle(input));

        let input = "4 6 5";
        let expected = true;
        assert_eq!(expected, valid_triangle(input));
    }

    #[test]
    fn test_part1() {
        let input = "4 6 32\n4 6 5\n16 17 18";
        let expected = 2;
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "4 6 32\n4 6 5\n5 17 18\n4 2 32\n5 3 1\n3 17 18";
        let expected = 2;
        assert_eq!(expected, solve_part2(input))
    }
}
