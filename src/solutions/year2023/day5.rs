#![allow(unused)]

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{i64, newline, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    range,
};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Mapping {
    source_range: Range<i64>,
    offset: i64,
}

impl Mapping {
    // Applies the mapping to a section of a range that overlaps
    fn partial_apply(&self, second: &Range<i64>) -> Option<Range<i64>> {
        let first = self.source_range.clone();
        let max_start = first.start.max(second.start);
        let min_end = first.end.min(second.end);

        (max_start < min_end).then(|| (max_start + self.offset)..(min_end + self.offset))
    }
}

type Mappings = Vec<Vec<Mapping>>;
type Items = Vec<i64>;

fn parse_seeds(input: &str) -> IResult<&str, Items> {
    preceded(tag("seeds: "), separated_list1(space1, i64))(input)
}

fn parse_seed_ranges(input: &str) -> IResult<&str, Vec<Range<i64>>> {
    preceded(
        tag("seeds:"),
        many1(preceded(
            space1,
            map(separated_pair(i64, space1, i64), |(start, len)| {
                start..(start + len)
            }),
        )),
    )(input)
}

fn parse_mappings(input: &str) -> IResult<&str, Mappings> {
    fn skip_to_next_mappings(input: &str) -> IResult<&str, &str> {
        take_while(|ch: char| !ch.is_digit(10))(input)
    }
    fn mapping(input: &str) -> IResult<&str, Mapping> {
        tuple((i64, space1, i64, space1, i64))
            .map(|(dest, _, source, _, length)| Mapping {
                source_range: source..(source + length),
                offset: dest - source,
            })
            .parse(input)
    }
    many1(preceded(
        skip_to_next_mappings,
        separated_list1(newline, mapping),
    ))(input)
}

fn apply_mapping_layer(item: i64, map_layer: &Vec<Mapping>) -> i64 {
    map_layer
        .iter()
        .find(|m| m.source_range.contains(&item))
        .and_then(|m| Some(item + m.offset))
        .unwrap_or(item)
}

pub fn solve_part1(input: &str) -> String {
    let (input, seeds) = parse_seeds(input).expect("Parser should work");
    let (_, mappings) = parse_mappings(input).unwrap();

    let locations = mappings.iter().fold(seeds, |items, mapping| {
        let new_prods = items
            .iter()
            .map(|&item| apply_mapping_layer(item, mapping))
            .collect();

        new_prods
    });

    locations.iter().min().unwrap().to_string()
}

pub fn solve_part2_brute(input: &str) -> String {
    let (input, seed_ranges) = parse_seed_ranges(input).expect("parser should work");
    let (_, mappings) = parse_mappings(input).unwrap();

    let number_of_seed_ranges = seed_ranges.len() as u64;

    // Brute force
    let min_location = seed_ranges
        .into_par_iter()
        .flat_map(|sr| {
            sr.map(|seed| mappings.iter().fold(seed, apply_mapping_layer))
                .min()
        })
        .progress_count(number_of_seed_ranges)
        .min();

    min_location.unwrap().to_string()
}

fn fill_gaps(mappings: Vec<Mapping>) -> Vec<Mapping> {
    let (mut result, end) =
        mappings
            .into_iter()
            .fold((Vec::new(), 0), |(mut result, last_end), range| {
                if range.source_range.start > last_end {
                    // Unit mapping
                    result.push(Mapping {
                        source_range: last_end..range.source_range.start,
                        offset: 0,
                    });
                }
                // The real mapping
                result.push(range.clone());

                let new_end = last_end.max(range.source_range.end);
                (result, new_end)
            });

    result.push(Mapping {
        source_range: end..i64::max_value(),
        offset: 0,
    });

    result
}

pub fn solve_part2(input: &str) -> String {
    let (input, seed_ranges) = parse_seed_ranges(input).unwrap();
    let (_, mapping_layers) = parse_mappings(input).unwrap();

    let filled_mapping_layers = mapping_layers
        .into_iter()
        .map(|mut layer| {
            layer.sort_by_key(|mapping| mapping.source_range.start);
            layer
        })
        .map(fill_gaps);

    let location_ranges = filled_mapping_layers.fold(seed_ranges, |item_ranges, layer| {
        item_ranges
            .iter()
            .flat_map(|item_range| {
                layer
                    .iter()
                    .filter_map(|mapping| mapping.partial_apply(item_range))
            })
            .collect_vec()
    });

    let min_location = location_ranges
        .into_iter()
        .flat_map(|range| range.min())
        .min()
        .expect("There has to be a minimum");

    min_location.to_string()
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
        let expected = "46";
        assert_eq!(expected, solve_part2(input))
    }
}
