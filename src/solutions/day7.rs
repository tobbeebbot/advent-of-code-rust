#![allow(unused)]

// https://adventofcode.com/2016/day/7

use std::{collections::{VecDeque, HashSet}, hash::Hash};

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

struct IPv7Section<'a> {
    sequence: &'a str,
    kind: SectionKind,
}

fn parse_ipv7_sequence(input: &str) -> Vec<IPv7Section> {
    fn parse_hypernet(input: &str) -> IResult<&str, IPv7Section> {
        delimited(complete::char('['), alpha0, complete::char(']'))(input)
            .map(|(i, s)| (i, IPv7Section { sequence: s, kind: SectionKind::Hypernet }))
    }

    fn parse_supernet(input: &str) -> IResult<&str, IPv7Section> {
        alpha1(input).map(|(i, s)| (i, IPv7Section {sequence: s, kind: SectionKind::Supernet} ))
    }

    let (_, v) = many0(alt((parse_supernet, parse_hypernet)))(input).unwrap_or_default();
    v
}

fn supports_tls(ip: &str) -> bool {
    fn has_abba(input: &str) -> bool {
        input
            .chars()
            .scan(VecDeque::new(), |queue, ch| {
                queue.push_back(ch);

                if queue.len() < 4 {
                    return Some(false);
                }

                let is_abba = queue[0] == queue[3] && queue[1] == queue[2] && queue[0] != queue[1];

                queue.pop_front();

                Some(is_abba)
            })
            .any(|b| b)
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

fn supports_ssl(ip: &str) -> bool {
    fn find_unique_abas(input: &str) -> HashSet<String> {
        let mut unique_abas = HashSet::new();
        input
            .chars()
            .scan(VecDeque::new(), |queue, ch| {
                queue.push_back(ch);

                if queue.len() < 3 {
                    return Some(false);
                }

                let is_aba = queue[0] == queue[2] && queue[0] != queue[1];

                if is_aba {
                    unique_abas.insert(queue.clone().iter().collect());
                }
                queue.pop_front();
                
                Some(is_aba)
            });
        unique_abas
    }

    fn find_match_bab(input: &str, set: &HashSet<String>) -> bool {
        input
            .chars()
            .scan(VecDeque::new(), |queue, ch| {
                queue.push_back(ch);

                if queue.len() < 3 {
                    return Some(false);
                }

                let is_aba = queue[0] == queue[2] && queue[0] != queue[1];
                let bab: String = [queue[1], queue[0], queue[1]].clone().iter().collect();

                queue.pop_front();

                if is_aba && set.contains(&bab) {
                        Some(true)
                } else {
                    Some(false)
                }
            }).any(|b| b)
    }
    let ip_sec = parse_ipv7_sequence(ip);
    let supernets = ip_sec.iter().filter(|sect| sect.kind == SectionKind::Supernet);

    let super_abas = supernets.flat_map(|s | {
        find_unique_abas(s.sequence)
    }).collect::<HashSet<String>>();

    println!("{:?}", super_abas);

    let mut hypernets = ip_sec.iter().filter(|sect| sect.kind == SectionKind::Hypernet);

    let has_matching_aba_bab = hypernets.any(|hn| find_match_bab(hn.sequence, &super_abas));

    has_matching_aba_bab
}

pub fn solve_part1(input: &str) -> u32 {
    input.lines().filter(|ip| supports_tls(ip)).count() as u32
}

pub fn solve_part2(input: &str) -> String {
    "unimplemented".to_string()
}

#[cfg(test)]
mod test_day7 {
    use crate::solutions::day7::*;
    use SectionKind::*;

    #[test]
    fn test_parse_sequence() {
        assert_eq!(
            Vec::from([Supernet, Hypernet, Supernet]),
            parse_ipv7_sequence("abba[mnop]qrst").into_iter().map(|ipsec| ipsec.kind).collect::<Vec<SectionKind>>()
        );

        assert_eq!(
            Vec::from(["abba", "mnop", "qrst"]),
            parse_ipv7_sequence("abba[mnop]qrst").into_iter().map(|ipsec| ipsec.sequence).collect::<Vec<&str>>()
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
    fn test_supports_ssl() {
        assert_eq!(true, supports_ssl("aba[bab]xyz"));
    }

    #[test]
    fn test_not_supports_ssl() {
        assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
    }

    #[test]
    fn test_part1() {
        let input = "abba[mnop]qrst\nabcd[oxxo]xyyx\nioxxoj[asdfgh]zxcvbn";
        let expected = 2;
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "todo";
        let expected = "todo";
        assert_eq!(expected, solve_part2(input))
    }
}
