#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet};

use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning: BTreeSet<u32>,
    numbers: BTreeSet<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (_, _, id, _, _)) =
        tuple((tag("Card"), space1, u32, tag(":"), space1))(input)?;
    let (input, winning) = separated_list1(space1, u32)(input)?;
    let (input, _) = tuple((space0, tag("|"), space0))(input)?;
    let (input, numbers) = separated_list1(space1, u32)(input)?;

    Ok((
        input,
        Card {
            id,
            winning: winning.into_iter().collect(),
            numbers: numbers.into_iter().collect(),
        },
    ))
}

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_card(line).expect("Prefer crash if parse fails."))
        .map(|(_, card)| card.winning.intersection(&card.numbers).count() as u32)
        .filter(|&nr_of_winning| nr_of_winning > 0)
        .map(|nr_of_winning| 2_u32.pow(nr_of_winning - 1))
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_card(line).expect("Prefer crash if parse fails.").1)
        .fold(BTreeMap::<u32, u32>::new(), |mut map, card| {
            let matches = card.winning.intersection(&card.numbers).count();

            let copies_of_current = map.entry(card.id).or_insert(1).clone();

            for offset in (1..=matches as u32) {
                map.entry(card.id + offset)
                    .and_modify(|e| *e += copies_of_current)
                    .or_insert(1 + copies_of_current);
            }

            map
        })
        .values()
        .sum()
}

#[cfg(test)]
mod test_day4 {
    use std::collections::{btree_set, BTreeSet};

    use super::*;

    #[test]
    fn test_parse_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(
            parse_card(input),
            Ok((
                "",
                Card {
                    id: 1,
                    winning: [41, 48, 83, 86, 17].into_iter().collect(),
                    numbers: [83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
                }
            ))
        )
    }

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve_part1(input), 13)
    }
    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = 30;
        assert_eq!(expected, solve_part2(input))
    }
}
