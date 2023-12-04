#![allow(unused)]

use std::{
    collections::{BTreeMap, BTreeSet},
    default,
    rc::Rc,
    usize,
};

use itertools::Itertools;
use nom::{
    self,
    branch::alt,
    character::complete::{anychar, digit0},
    multi::{many0, many1, many1_count},
};
use nom::{
    character::{self, complete::digit1},
    combinator::map,
    multi::many0_count,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Number {
    y: usize,
    start_x: usize,
    end_x: usize,
    value: u32,
}

impl Number {
    fn neighbours(&self) -> Vec<Pos> {
        ((self.start_x as i32 - 1)..=(self.end_x as i32 + 1))
            .map(|x| {
                let y = self.y as i32;
                [Pos::new(x, y - 1), Pos::new(x, y), Pos::new(x, y + 1)]
            })
            .flatten()
            .collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pos {
    x: i32,
    y: i32,
}
impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { y, x }
    }
}

#[derive(Debug)]
struct Symbol {
    y: i32,
    x: i32,
    value: char,
}

impl Symbol {
    fn neighbours(&self) -> Vec<Pos> {
        ((self.x as i32 - 1)..=(self.x as i32 + 1))
            .map(|x| {
                let y = self.y as i32;
                [Pos::new(x, y - 1), Pos::new(x, y), Pos::new(x, y + 1)]
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Number(usize),
    Symbol(char),
    Empty,
}

#[derive(Debug, PartialEq)]
struct ParseToken {
    token_type: TokenType,
    size: usize,
}

fn parse_line(input: &str) -> IResult<&str, Vec<ParseToken>> {
    many1(map(
        alt((
            map(many1_count(character::complete::char('.')), |size| {
                (size, TokenType::Empty)
            }),
            map(digit1, |digits: &str| {
                (
                    digits.len(),
                    TokenType::Number(digits.parse::<usize>().expect("Should be digits")),
                )
            }),
            map(anychar, |c| (1, TokenType::Symbol(c))),
        )),
        |(size, token_type)| ParseToken { token_type, size },
    ))(input)
}

pub fn solve_part1(input: &str) -> u32 {
    let mut numbers = Vec::new();
    let mut symbol_positions = BTreeSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0;
        parse_line(line)
            .into_iter()
            .flat_map(|res| Some(res.1))
            .for_each(|tokens| {
                tokens.iter().for_each(|ParseToken { token_type, size }| {
                    match token_type {
                        TokenType::Number(value) => numbers.push(Number {
                            y,
                            start_x: x,
                            end_x: x + size - 1,
                            value: *value as u32,
                        }),
                        TokenType::Symbol(c) => {
                            symbol_positions.insert(Pos::new(x as i32, y as i32));
                        }
                        TokenType::Empty => (),
                    }
                    x += size;
                })
            })
    });

    numbers
        .iter()
        .filter(|&number| {
            number
                .neighbours()
                .iter()
                .any(|neigh| symbol_positions.contains(neigh))
        })
        .map(|num| num.value)
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    let mut cogs = Vec::new();
    let mut number_positions = BTreeMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0;
        parse_line(line)
            .into_iter()
            .flat_map(|res| Some(res.1))
            .for_each(|tokens| {
                tokens.iter().for_each(|ParseToken { token_type, size }| {
                    match token_type {
                        TokenType::Number(value) => {
                            let number = Number {
                                y,
                                start_x: x,
                                end_x: x + size - 1,
                                value: *value as u32,
                            };
                            number_positions.extend(
                                (number.start_x..=number.end_x)
                                    .map(|x| (Pos::new(x as i32, y as i32), number)),
                            );
                        }
                        TokenType::Symbol('*') => {
                            cogs.push(Symbol {
                                y: y as i32,
                                x: x as i32,
                                value: '*',
                            });
                        }
                        _ => (),
                    }
                    x += size;
                })
            })
    });

    cogs.iter()
        .filter_map(|cog| {
            let mut adjecent_numbers = cog
                .neighbours()
                .into_iter()
                .filter_map(|n| number_positions.get(&n))
                .unique();

            let (first, last) = (adjecent_numbers.next(), adjecent_numbers.next());

            let product = first.and_then(|f| last.map(|l| f.value * l.value));

            // Make sure the cog desnt have >2 neighbours. Undefined!
            product.filter(|_| adjecent_numbers.next().is_none())
        })
        .sum()
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    #[test]
    fn test_parse_line() {
        use TokenType::*;
        let input = "....35x...67";

        assert_eq!(
            parse_line(input),
            Ok((
                "",
                vec![
                    ParseToken {
                        token_type: Empty,
                        size: 4
                    },
                    ParseToken {
                        token_type: Number(35),
                        size: 2
                    },
                    ParseToken {
                        token_type: Symbol('x'),
                        size: 1
                    },
                    ParseToken {
                        token_type: Empty,
                        size: 3
                    },
                    ParseToken {
                        token_type: Number(67),
                        size: 2
                    }
                ]
            ))
        )
    }

    #[test]
    fn test_part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let expected = 4361;
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let expected = 467835;
        assert_eq!(expected, solve_part2(input))
    }
}
