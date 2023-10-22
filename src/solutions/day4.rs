#![allow(unused)]

use std::{collections::HashMap, ops::Index};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1},
    combinator::opt,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Default)]
struct RoomEntry {
    encrypted_name: String,
    sector_id: u32,
    check_sum: String,
}

impl RoomEntry {
    fn check_validity(&self) -> bool {
        let top_chars = self
            .encrypted_name
            .chars()
            .counts()
            .iter()
            .sorted_by(|(k1, v1), (k2, v2)| Ord::cmp(v2, v1).then(Ord::cmp(k1, k2)))
            .map(|(k, v)| k)
            .join("");

        top_chars.starts_with(&self.check_sum)
    }

    fn decrypt_name(&self) -> String {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let alpha_length = alphabet.len();

        let alphabet_twice = format!("{alphabet}{alphabet}");

        self.encrypted_name.chars().map(|c| {
            let shift = self.sector_id as usize % alpha_length;
            alphabet_twice.chars().skip_while(|&a| a != c).skip(shift).next().unwrap()
        }).join("")

    }
}

fn parse_room_entry(input: &str) -> IResult<&str, RoomEntry> {
    fn parse_encrypted_name(input: &str) -> IResult<&str, String> {
        let (input, some_alpha) = many1(terminated(alpha1, opt(complete::char('-'))))(input)?;
        Ok((input, some_alpha.join("")))
    }

    fn parse_sector_id(input: &str) -> IResult<&str, u32> {
        let (input, id) = complete::u32(input)?;
        Ok((input, id))
    }

    fn parse_check_sum(input: &str) -> IResult<&str, String> {
        let (input, check_sum) =
            delimited(complete::char('['), alpha1, complete::char(']'))(input)?;
        Ok((input, check_sum.to_owned()))
    }

    let (input, encrypted_name) = parse_encrypted_name(input)?;
    let (input, sector_id) = parse_sector_id(input)?;
    let (input, check_sum) = parse_check_sum(input)?;

    Ok((
        input,
        RoomEntry {
            encrypted_name,
            sector_id,
            check_sum,
        },
    ))
}

fn parse_room_entries(input: &str) -> Vec<RoomEntry> {
    let (_, res) = separated_list0(tag("\n"), parse_room_entry)(input).unwrap_or_default();
    res
}

pub fn solve_part1(input: &str) -> u32 {
    parse_room_entries(input)
        .into_iter()
        .filter(|re| re.check_validity())
        .map(|re| re.sector_id)
        .sum()
}

pub fn solve_part2(input: &str) -> String {
    let decrypted_names = parse_room_entries(input)
        .into_iter()
        .filter(|re| re.check_validity())
        .filter_map(|re| {
            let dec_name = re.decrypt_name();
            if dec_name.contains("north") {
                Some(format!("{}: {}", re.sector_id, dec_name))
            } else {
                None
            }
        })
        .join("\n");

    decrypted_names
}

#[cfg(test)]
mod test_day4 {
    use crate::solutions::day4::*;

    #[test]
    fn test_parse_room_entry() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = RoomEntry {
            encrypted_name: "aaaaabbbzyx".to_owned(),
            sector_id: 123,
            check_sum: "abxyz".to_owned(),
        };

        let (_, room_entry) = parse_room_entry(input).unwrap();
        assert_eq!(expected, room_entry);
    }

    #[test]
    fn test_room_validity() {
        let room = RoomEntry {
            encrypted_name: "aaaaabbbzyx".to_owned(),
            sector_id: 123,
            check_sum: "abxyz".to_owned(),
        };
        assert!(room.check_validity() == true);

        let (_, room) = parse_room_entry("not-a-real-room-404[oarel]").unwrap();
        assert!(room.check_validity() == true);

        let (_, room) = parse_room_entry("not-a-real-room-404[oarle]").unwrap();
        assert!(room.check_validity() == false);

        let (_, room) = parse_room_entry("totally-real-room-200[decoy]").unwrap();
        assert!(room.check_validity() == false);
    }

    #[test]
    fn test_decrypt_name() {
        let (_, room) = parse_room_entry("qzmt-zixmtkozy-ivhz-343[asdds]").unwrap();
        let decrypted = room.decrypt_name();

        assert_eq!("veryencryptedname", decrypted); // spaces dissapear but otherwise good
    }

    #[test]
    fn test_part1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]";
        let expected = 1514;
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        // Not needed
    }
}
