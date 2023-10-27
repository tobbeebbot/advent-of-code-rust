#![allow(unused)]

// https://adventofcode.com/2016/day/7

use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, alpha0, alpha1},
    multi::many0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
enum SectionKind {
    Hypernet,
    Supernet,
}

#[derive(Debug)]
struct IPv7Section<'a> {
    sequence: &'a str,
    kind: SectionKind,
}

fn parse_ipv7_sequence(input: &str) -> Vec<IPv7Section> {
    fn parse_hypernet(input: &str) -> IResult<&str, IPv7Section> {
        delimited(complete::char('['), alpha0, complete::char(']'))(input).map(|(i, s)| {
            (
                i,
                IPv7Section {
                    sequence: s,
                    kind: SectionKind::Hypernet,
                },
            )
        })
    }

    fn parse_supernet(input: &str) -> IResult<&str, IPv7Section> {
        alpha1(input).map(|(i, s)| {
            (
                i,
                IPv7Section {
                    sequence: s,
                    kind: SectionKind::Supernet,
                },
            )
        })
    }

    let (_, v) = many0(alt((parse_supernet, parse_hypernet)))(input).unwrap_or_default();
    v
}

fn supports_tls(ip: &str) -> bool {
    fn has_abba(input: &str) -> bool {
        input
            .chars()
            .tuple_windows()
            .any(|(a, b, bb, aa)| a == aa && b == bb && a != b)
    }

    let (any_hyper, any_other) = parse_ipv7_sequence(ip).into_iter().fold(
        (false, false),
        |(any_hyper, any_other), ip_sect| match ip_sect.kind {
            SectionKind::Hypernet => (any_hyper || has_abba(ip_sect.sequence), any_other),
            SectionKind::Supernet => (any_hyper, any_other || has_abba(ip_sect.sequence)),
        },
    );

    !any_hyper && any_other
}

fn find_unique_abas(input: &str) -> HashSet<(char, char, char)> {
    input
        .chars()
        .tuple_windows()
        .filter(|(a, b, aa)| a == aa && a != b)
        .collect()
}

fn supports_ssl(ip: &str) -> bool {
    let mut super_abas = HashSet::new();
    let mut hyper_babs = HashSet::new();

    parse_ipv7_sequence(ip).iter().for_each(|section| {
        let abas = find_unique_abas(section.sequence);
        match section.kind {
            SectionKind::Supernet => super_abas.extend(abas),
            SectionKind::Hypernet => hyper_babs.extend(
                abas    // Convert to bab
                    .into_iter()
                    .map(|(a, b, aa)| (b, a, b))
                    .collect::<HashSet<(_, _, _)>>(),
            ),
        };
    });

    !super_abas.is_disjoint(&hyper_babs)
}

pub fn solve_part1(input: &str) -> u32 {
    input.lines().filter(|ip| supports_tls(ip)).count() as u32
}

pub fn solve_part2(input: &str) -> u32 {
    input.lines().filter(|ip| supports_ssl(ip)).count() as u32
}

#[cfg(test)]
mod test_day7 {
    use crate::solutions::day7::*;
    use SectionKind::*;

    #[test]
    fn test_parse_sequence() {
        assert_eq!(
            Vec::from([Supernet, Hypernet, Supernet]),
            parse_ipv7_sequence("abba[mnop]qrst")
                .into_iter()
                .map(|ipsec| ipsec.kind)
                .collect::<Vec<SectionKind>>()
        );

        assert_eq!(
            Vec::from(["abba", "mnop", "qrst"]),
            parse_ipv7_sequence("abba[mnop]qrst")
                .into_iter()
                .map(|ipsec| ipsec.sequence)
                .collect::<Vec<&str>>()
        );
    }

    #[test]
    fn test_abba_outside() {
        assert_eq!(true, supports_tls("abba[mnop]qrst"));
    }

    #[test]
    fn test_abba_within_larger_string() {
        assert_eq!(true, supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_no_abba() {
        assert_eq!(false, supports_tls("aaaa[qwer]tyui"));
    }

    #[test]
    fn test_abba_inside_and_outside() {
        assert_eq!(false, supports_tls("abcd[oxxo]xyyx"));
    }

    #[test]
    fn test_part1() {
        let input = "abba[mnop]qrst\nabcd[oxxo]xyyx\nioxxoj[asdfgh]zxcvbn";
        let expected = 2;
        assert_eq!(expected, solve_part1(input))
    }

    #[test]
    fn test_find_abas() {
        assert_eq!(
            HashSet::from([('a', 'b', 'a'), ('x', 'o', 'x'), ('b', 'a', 'b')]),
            find_unique_abas("ababaxoxsadsadxox")
        );
    }

    #[test]
    fn test_supports_ssl() {
        assert_eq!(true, supports_ssl("aba[bab]xyz"));
    }

    #[test]
    fn test_not_supports_ssl() {
        assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
        assert_eq!(false, supports_ssl("aba[cac]xyz"));
    }
    #[test]
    fn test_part2() {
        let input = "aba[bab]xyz\nxyx[xyx]xyx\naaa[kek]eke\nzazbz[bzb]cdb\naba[cac]xyz";
        let expected = 3;
        assert_eq!(expected, solve_part2(input))
    }
}
