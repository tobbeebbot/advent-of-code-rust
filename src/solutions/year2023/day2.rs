#![allow(unused)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

enum CubeCount {
    Blue(u32),
    Green(u32),
    Red(u32),
}

fn parse_game(input: &str) -> IResult<&str, (u32, Vec<CubeCount>)> {
    let (input, game_id) = delimited(tag("Game "), u32, tag(": "))(input)?;

    let cube_draw = separated_list1(
        tag(", "),
        alt((
            map(terminated(u32, tag(" blue")), CubeCount::Blue),
            map(terminated(u32, tag(" red")), CubeCount::Red),
            map(terminated(u32, tag(" green")), CubeCount::Green),
        )),
    );

    let (input, cube_draws) = separated_list1(tag("; "), cube_draw)(input)?;

    Ok((input, (game_id, cube_draws.into_iter().flatten().collect())))
}

fn is_possible(input: &str) -> Option<u32> {
    let (_, (game_id, cube_draws)) = parse_game(input).expect("Should be able to parse");

    cube_draws
        .into_iter()
        .all(|cube_count| match cube_count {
            CubeCount::Blue(n) => n <= 14,
            CubeCount::Green(n) => n <= 13,
            CubeCount::Red(n) => n <= 12,
        })
        .then_some(game_id)
}

fn minimum_counts(input: &str) -> (u32, u32, u32) {
    let (_, (game_id, cube_draws)) = parse_game(input).expect("Should be able to parse");

    let (mut reds, mut greens, mut blues) = (0, 0, 0);
    cube_draws
        .into_iter()
        .for_each(|cube_count| match cube_count {
            CubeCount::Blue(n) if n > blues => blues = n,
            CubeCount::Green(n) if n > greens => greens = n,
            CubeCount::Red(n) if n > reds => reds = n,
            _ => (),
        });

    (reds, greens, blues)
}

pub fn solve_part1(input: &str) -> u32 {
    input.lines().filter_map(is_possible).sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(minimum_counts)
        .map(|(r, g, b)| r * g * b)
        .sum()
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn test_is_possible() {
        let possible = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let impossible = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        assert_eq!(Some(1), is_possible(possible));
        assert_eq!(None, is_possible(impossible));
    }

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 8;
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = 2286;
        assert_eq!(expected, solve_part2(input))
    }
}
