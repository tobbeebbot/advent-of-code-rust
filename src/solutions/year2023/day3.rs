#![allow(unused)]

use std::{collections::{BTreeMap, BTreeSet}, rc::Rc};

use itertools::{enumerate, Itertools};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum Token {
    Number(i32, u32), // i32 multiplication "hash"
    Symbol
}

pub fn solve_part1(input: &str) -> u64 {
    let width = input.find('\n').expect("Must have newlines");
    
    let mut map = BTreeMap::<(i32, i32), Token>::new();
    let mut number = 0;
    let mut n_digits = 0;

    let mut number_set = BTreeSet::<Token>::new();
    enumerate(input.lines()).for_each(|(i, line)| {
        enumerate(line.chars()).for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            if c.is_numeric() {
                number = number * 10 + c.to_digit(10).unwrap();
                n_digits += 1;
            } else {
                if n_digits > 0 {
                    (1..=n_digits).for_each(|d| {
                        map.insert((i, j - d), Token::Number(i * j + j, number));
                    });

                    // before reset, search for earlier Symbols
                    let coordinates = (0..=n_digits).map(|d| (i - 1, j - d)).chain(vec!((i, j - n_digits - 1)).into_iter());
                    let any_symbols = coordinates.flat_map(|coord| map.get(&coord)).any(|asd| match asd {
                        Token::Number(_, _) => false,
                        Token::Symbol => true,
                    });

                    if any_symbols {
                        number_set.insert(Token::Number(i * j + j, number));
                    }

                    n_digits = 0;
                    number = 0;
                }

                if c != '.' {
                    map.insert((i, j), Token::Symbol);
                    let numbers: Vec<Token> = [(i - 1, j - 1), (i - 1, j), (i - 1, j + 1), (i, j - 1)].into_iter().flat_map(|coord| map.get(&coord).and_then(|tok| match tok {
                        Token::Symbol => None,
                        number => Some(*number)
                    })).collect();

                    number_set.extend(numbers)
                }
            }

        })
    });

    number_set.into_iter().flat_map(|token| match token {
        Token::Number(_, value) => Some(value as u64),
        Token::Symbol => None,
    }).sum::<u64>()
}

pub fn solve_part2(input: &str) -> String {
    "unimplemented".to_string()
}

#[cfg(test)]
mod test_day3 {
    use super::*;

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
        let input = "todo";
        let expected = "todo";
        assert_eq!(expected, solve_part2(input))
    }
}
