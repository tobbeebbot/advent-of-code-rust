#![allow(unused)]

use std::collections::{btree_map, BTreeMap};

use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, newline, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Kind {
    fn from_value_counts(vcs: Vec<&usize>) -> Kind {
        let mut occurances = vcs.into_iter().sorted().rev();

        let largest = occurances.next();
        let second_largest = occurances.next();

        match (largest, second_largest) {
            (Some(_), None) => Kind::FiveOfAKind,
            (Some(4), Some(_)) => Kind::FourOfAKind,
            (Some(3), Some(2)) => Kind::FullHouse,
            (Some(3), Some(_)) => Kind::ThreeOfAKind,
            (Some(2), Some(2)) => Kind::TwoPair,
            (Some(2), Some(_)) => Kind::OnePair,
            (Some(_), Some(_)) => Kind::HighCard,
            (None, _) => panic!("Something went very wrong"),
        }
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

impl Hand<'_> {
    fn joker_kind(&self) -> Kind {
        let mut occurances = self.cards.chars().counts();

        if let Some(j_count) = occurances.remove(&'J') {
            if let Some((c, c_count)) = occurances.iter().max_by(|(k, v), (k2, v2)| v.cmp(v2)) {
                occurances.insert(*c, c_count + j_count);
            } else {
                occurances.insert('J', j_count);
            }
        }

        Kind::from_value_counts(occurances.values().collect_vec())
    }

    fn kind(&self) -> Kind {
        let mut occurances = self.cards.chars().counts();
        Kind::from_value_counts(occurances.values().collect_vec())
    }

    fn card_values(&self, value_map: &BTreeMap<char, u32>) -> Vec<u32> {
        self.cards
            .chars()
            .flat_map(|c| {
                value_map
                    .get(&c)
                    .or(c.to_digit(10).as_ref())
                    .map(|i| i.clone())
            })
            .collect()
    }
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    fn parse_hand(input: &str) -> IResult<&str, Hand> {
        map(
            separated_pair(alphanumeric1, space1, u32),
            |(cards, bid)| Hand { cards, bid },
        )(input)
    }

    separated_list1(newline, parse_hand)(input)
}

pub fn solve_part1(input: &str) -> String {
    let value_mapping = BTreeMap::from(
        [('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]);

    parse_hands(input)
        .expect("parsing works")
        .1
        .iter()
        .sorted_by(|a, b| {
            b.kind().cmp(&a.kind()).then(
                a.card_values(&value_mapping)
                    .cmp(&b.card_values(&value_mapping)),
            )
        })
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

pub fn solve_part2(input: &str) -> String {
    let value_mapping = BTreeMap::from(
        [('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)]);

    parse_hands(input)
        .expect("parsing works")
        .1
        .into_iter()
        .sorted_by(|a, b| {
            b.joker_kind().cmp(&a.joker_kind()).then(
                a.card_values(&value_mapping)
                    .cmp(&b.card_values(&value_mapping)),
            )
        })
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let expected = "6440";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let expected = "5905";
        assert_eq!(expected, solve_part2(input))
    }
}
