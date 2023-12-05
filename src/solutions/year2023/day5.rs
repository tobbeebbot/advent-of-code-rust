#![allow(unused)]

use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_till, take_until, take_until1, take_while, take_while1},
    character::{
        complete::{i64, newline, not_line_ending, space1, u32},
        is_digit,
    },
    multi::{self, many0, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Mapping {
    source_range: Range<i64>,
    dest_range: Range<i64>,
    offset: i64,
}

#[derive(Debug)]
struct SeedMapping {
    seeds: Vec<i64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fert: Vec<Mapping>,
    fert_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temp: Vec<Mapping>,
    temp_to_humid: Vec<Mapping>,
    humid_to_location: Vec<Mapping>,
}

fn parse(input: &str) -> IResult<&str, SeedMapping> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, i64))(input)?;

    fn skip_to_next_mappings(input: &str) -> IResult<&str, &str> {
        take_while(|ch: char| !ch.is_digit(10))(input)
    }
    fn mapping(input: &str) -> IResult<&str, Mapping> {
        tuple((i64, space1, i64, space1, i64))
            .map(|(dest, _, source, _, length)| Mapping {
                dest_range: dest..(dest + length),
                source_range: source..(source + length),
                offset: dest - source,
            })
            .parse(input)
    }
    let (input, seed_to_soil) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;
    let (input, soil_to_fert) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;
    let (input, fert_to_water) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;
    let (input, water_to_light) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;
    let (input, light_to_temp) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;
    let (input, temp_to_humid) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;
    let (input, humid_to_location) =
        preceded(skip_to_next_mappings, separated_list1(newline, mapping))(input)?;

    Ok((
        input,
        SeedMapping {
            seeds,
            seed_to_soil,
            soil_to_fert,
            fert_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humid,
            humid_to_location,
        },
    ))
}

pub fn solve_part1(input: &str) -> String {
    let input = parse(input).unwrap().1;
    println!("{:?}", input);

    let min_location = input
        .seeds
        .iter()
        .map(|seed| {
            input
                .seed_to_soil
                .iter()
                .find(|m| m.source_range.contains(seed))
                .and_then(|m| Some(seed + m.offset))
                .unwrap_or(*seed)
        })
        .map(|soil| {
            let opt = input
                .soil_to_fert
                .iter()
                .find(|m| m.source_range.contains(&soil));

            opt
                .and_then(|m| Some(soil + m.offset))
                .unwrap_or(soil)
        })
        .map(|fert| {
            input
                .fert_to_water
                .iter()
                .find(|m| m.source_range.contains(&fert))
                .and_then(|m| Some(fert + m.offset))
                .unwrap_or(fert)
        })
        .map(|water| {
            input
                .water_to_light
                .iter()
                .find(|m| m.source_range.contains(&water))
                .and_then(|m| Some(water + m.offset))
                .unwrap_or(water)
        })
        .map(|light| {
            input
                .light_to_temp
                .iter()
                .find(|m| m.source_range.contains(&light))
                .and_then(|m| Some(light + m.offset))
                .unwrap_or(light)
        })
        .map(|temp| {
            input
                .temp_to_humid
                .iter()
                .find(|m| m.source_range.contains(&temp))
                .and_then(|m| Some(temp + m.offset))
                .unwrap_or(temp)
        })
        .map(|humid| {
            input
                .humid_to_location
                .iter()
                .find(|m| m.source_range.contains(&humid))
                .and_then(|m| Some(humid + m.offset))
                .unwrap_or(humid)
        })
        .min();


    min_location.unwrap().to_string()
}

pub fn solve_part2(input: &str) -> String {
    "unimplemented".to_string()
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let expected = "35";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "todo";
        let expected = "todo";
        assert_eq!(expected, solve_part2(input))
    }
}
