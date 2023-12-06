#![allow(unused)]

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1, u64},
    combinator::map,
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

type Time = u64;
type Record = u64;
fn parse_race_details(input: &str) -> IResult<&str, (Vec<Time>, Vec<Record>)> {
    separated_pair(
        preceded(tag("Time:"), many1(preceded(space1, u64))),
        newline,
        preceded(tag("Distance:"), many1(preceded(space1, u64))),
    )(input)
}

fn parse_race_details_kerning(input: &str) -> IResult<&str, (Time, Record)> {
    separated_pair(
        preceded(
            tag("Time:"),
            map(many1(preceded(space1, digit1)), |numbers| {
                numbers.join("").parse().unwrap()
            }),
        ),
        newline,
        preceded(
            tag("Distance:"),
            map(many1(preceded(space1, digit1)), |numbers| {
                numbers.join("").parse().unwrap()
            }),
        ),
    )(input)
}

pub fn solve_part1(input: &str) -> String {
    let (_, (times, records)) = parse_race_details(input).unwrap();

    let race_entries = times.into_iter().zip(records);

    let winning_strategies_per_race = race_entries
        .map(|(time, record)| {
            (0..=time)
                .map(|hold_time| {
                    let remaining_time = time - hold_time;
                    hold_time * remaining_time
                })
                .filter(|distance_traveled| distance_traveled > &record)
                .count()
        })
        .collect_vec();

    winning_strategies_per_race
        .iter()
        .product::<usize>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let (_, (time, record)) = parse_race_details_kerning(input).unwrap();

    (0..=time)
        .map(|hold_time| {
            let remaining_time = time - hold_time;
            hold_time * remaining_time
        })
        .filter(|distance_traveled| distance_traveled > &record)
        .count().to_string()
}

#[cfg(test)]
mod test_day6 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let expected = "288";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let expected = "71503";
        assert_eq!(expected, solve_part2(input))
    }
}
