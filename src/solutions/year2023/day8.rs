#![allow(unused)]

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{self, alpha1, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Node<'a> {
    key: &'a str,
    left: &'a str,
    right: &'a str,
}

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(complete::char('L'), |_| Instruction::Left),
        map(complete::char('R'), |_| Instruction::Right),
    )))(input)
}

fn nodes(input: &str) -> IResult<&str, HashMap<&str, Node>> {
    fn node(input: &str) -> IResult<&str, Node> {
        let (input, (key, (left, right))) = separated_pair(
            alpha1,
            tag(" = "),
            delimited(
                complete::char('('),
                separated_pair(alpha1, tag(", "), alpha1),
                complete::char(')'),
            ),
        )(input)?;

        Ok((input, Node { key, left, right }))
    }
    let (input, nodes) = separated_list1(newline, node)(input)?;
    let graph = nodes
        .into_iter()
        .map(|node| (node.key, node))
        .collect::<HashMap<&str, Node>>();

    Ok((input, graph))
}

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<&str, Node>) {
    let (_, (instructions, graph)) =
        separated_pair(instructions, take_till(|c: char| c.is_alphabetic()), nodes)(input)
            .expect("Parser works");

    (instructions, graph)
}

pub fn solve_part1(input: &str) -> String {
    let (instructions, graph) = parse_input(input);

    let iter_count = instructions
        .into_iter()
        .cycle()
        .scan("AAA", |mut key, instruction| {
            let opt_node = graph.get(key);

            if let Some(node) = opt_node {
                match instruction {
                    Instruction::Left => *key = node.left,
                    Instruction::Right => *key = node.right,
                }
            }
            opt_node
        })
        .take_while(|node| node.key != "ZZZ")
        .count();

    iter_count.to_string()
}

pub fn solve_part2(input: &str) -> String {
    "unimplemented".to_string()
}

#[cfg(test)]
mod test_ {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let expected = "2";
        assert_eq!(expected, solve_part1(input))
    }

    #[test]
    fn test_part1_again() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let expected = "6";
        assert_eq!(expected, solve_part1(input))
    }

    #[test]
    fn test_part2() {
        let input = "todo";
        let expected = "todo";
        assert_eq!(expected, solve_part2(input))
    }
}
