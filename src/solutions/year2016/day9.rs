#![allow(unused)]

use nom::{branch, bytes::complete, character, combinator, sequence, IResult};

#[derive(Debug, PartialEq)]
enum CompressionSequence<'a> {
    Raw(&'a str),
    Repeated(u32, &'a str),
}

fn parse_repeated(input: &str) -> IResult<&str, CompressionSequence> {
    let (input, (_, num_chars, _, num_repeated, _)) = sequence::tuple((
        character::complete::char('('),
        character::complete::u32,
        character::complete::char('x'),
        character::complete::u32,
        character::complete::char(')'),
    ))(input)?;
    let (input, out) = complete::take(num_chars)(input)?;
    Ok((input, CompressionSequence::Repeated(num_repeated, out)))
}

fn parse_raw(input: &str) -> IResult<&str, CompressionSequence> {
    combinator::map(complete::take_till1(|c| c == '('), CompressionSequence::Raw)(input)
}

fn decompress_size(input: &str) -> u32 {
    combinator::iterator(input, branch::alt((parse_repeated, parse_raw)))
        .map(|cs| match cs {
            CompressionSequence::Raw(s) => s.len() as u32,
            CompressionSequence::Repeated(r, s) => r * s.len() as u32,
        })
        .sum()
}

fn decompress_size_v2(input: &str) -> u128 {
    combinator::iterator(input, branch::alt((parse_repeated, parse_raw)))
        .map(|cs| match cs {
            CompressionSequence::Raw(s) => s.len() as u128,
            CompressionSequence::Repeated(r, s) => (r as u128) * decompress_size_v2(s),
        })
        .sum()
}

pub fn solve_part1(input: &str) -> u32 {
    decompress_size(input) as u32
}

pub fn solve_part2(input: &str) -> u128 {
    decompress_size_v2(input) as u128
}

#[cfg(test)]
mod test_day9 {
    use super::*;

    #[test]
    fn test_parse_repeated() {
        let input = "(3x2)ABCXYZ";
        assert_eq!(
            parse_repeated(input).unwrap(),
            ("XYZ", CompressionSequence::Repeated(2, "ABC"))
        )
    }

    #[test]
    fn test_parse_raw() {
        let input = "abc(1x3)";
        assert_eq!(
            parse_raw(input).unwrap(),
            ("(1x3)", CompressionSequence::Raw("abc"))
        );

        let input = "x(1x3)";
        assert_eq!(
            parse_raw(input).unwrap(),
            ("(1x3)", CompressionSequence::Raw("x"))
        );
    }

    #[test]
    fn test_decompress() {
        let input = "ADVENT";
        let expected = 6;
        assert_eq!(expected, decompress_size(input));

        let input = "X(8x2)(3x3)ABCY";
        let expected = 18;
        assert_eq!(expected, decompress_size(input));
    }

    #[test]
    fn test_decompress_v2() {
        let input = "ADVENT";
        let expected = 6;
        assert_eq!(expected, decompress_size_v2(input));

        let input = "X(8x2)(3x3)ABCY";
        let expected = 20;
        assert_eq!(expected, decompress_size_v2(input));
    }
}
