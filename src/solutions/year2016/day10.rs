#![allow(unused)]

use itertools::Itertools;
use nom::{
    branch,
    bytes::complete::tag,
    character::{self, complete},
    combinator, multi,
    sequence::{self, preceded, tuple},
    IResult,
};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
struct Bot {
    chips: Vec<Chip>,
    logic: Option<BotLogic>,
}

#[derive(Debug, Default)]
struct Output {
    chips: Vec<Chip>,
}

#[derive(Debug)]
struct BotLogic {
    give_low: SendId,
    give_high: SendId,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Chip(u32);

#[derive(Debug, Clone, Copy)]
enum SendId {
    Bot(u32),
    Output(u32),
}

#[derive(Debug)]
enum Instruction {
    ValueTo(Chip, u32),
    BotInstr(u32, BotLogic),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    fn send_id(input: &str) -> IResult<&str, SendId> {
        branch::alt((
            combinator::map(preceded(tag("bot "), complete::u32), SendId::Bot),
            combinator::map(preceded(tag("output "), complete::u32), SendId::Output),
        ))(input)
    }

    fn bot_logic(input: &str) -> IResult<&str, BotLogic> {
        let (input, (give_low, give_high)) = tuple((
            sequence::preceded(tag("low to "), send_id),
            sequence::preceded(tag(" and high to "), send_id),
        ))(input)?;

        Ok((
            input,
            BotLogic {
                give_low,
                give_high,
            },
        ))
    }

    branch::alt((
        combinator::map(
            tuple((
                sequence::preceded(tag("value "), complete::u32),
                sequence::preceded(tag(" goes to bot "), complete::u32),
            )),
            |(value, send_id)| Instruction::ValueTo(Chip(value), send_id),
        ),
        combinator::map(
            tuple((
                sequence::preceded(tag("bot "), complete::u32),
                sequence::preceded(tag(" gives "), bot_logic),
            )),
            |(bot_id, logic)| Instruction::BotInstr(bot_id, logic),
        ),
    ))(input)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    multi::separated_list0(character::complete::newline, parse_instruction)(input)
        .unwrap()
        .1
}

pub fn solve_part1(input: &str) -> String {
    let mut bots = HashMap::<u32, Bot>::new();
    let mut bot_queue = VecDeque::<u32>::new();
    let mut outputs = HashMap::<u32, Output>::new();

    for instr in parse_instructions(input) {
        match instr {
            Instruction::ValueTo(chip, id) => {
                let bot = bots.entry(id).or_default();
                bot.chips.push(chip);
                if bot.chips.len() == 2 {
                    bot_queue.push_back(id);
                    println!("Start goes to: {:?}", id);
                }
            }
            Instruction::BotInstr(bot_id, bot_logic) => {
                bots.entry(bot_id).or_default().logic = Some(bot_logic);
            }
        }
    }

    while !bot_queue.is_empty() {
        let bot_id = bot_queue.pop_front().unwrap();
        bots.get_mut(&bot_id)
            .and_then(|bot| {
                bot.chips.sort();
                let max_chip = bot.chips.pop()?;
                let min_chip = bot.chips.pop()?;

                if (max_chip == Chip(61) && min_chip == Chip(17)) {
                    println!("Found the special bot! Id: {:?}", bot_id)
                }

                bot.logic
                    .as_ref()
                    .and_then(|l| Some([(max_chip, l.give_high), (min_chip, l.give_low)]))
            })
            .expect("The bot must be listed in the bot map.")
            .into_iter()
            .for_each(|(chip, send_id)| match send_id {
                SendId::Bot(id) => {
                    bots.entry(id).and_modify(|bot| {
                        bot.chips.push(chip);
                        if bot.chips.len() == 2 {
                            bot_queue.push_back(id)
                        };
                    });
                }
                SendId::Output(id) => {
                    let output = outputs.entry(id).or_default();
                    output.chips.push(chip);
                }
            });
    }

    let Chip(val0) = outputs.get(&0).and_then(|out| out.chips.first()).unwrap();
    let Chip(val1) = outputs.get(&1).and_then(|out| out.chips.first()).unwrap();
    let Chip(val2) = outputs.get(&2).and_then(|out| out.chips.first()).unwrap();
    format!("Output value product: {}", (val0 * val1 * val2))
}

pub fn solve_part2(input: &str) -> String {
    "Part one solves both".to_string()
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = "value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives low to output 2 and high to output 0\nvalue 2 goes to bot 2";
        let output = parse_instructions(input);
        println!("{:?}", output);
    }

    #[test]
    fn test_part1() {
        let input = "value 5 goes to bot 2\nbot 2 gives low to bot 1 and high to bot 0\nvalue 3 goes to bot 1\nbot 1 gives low to output 1 and high to bot 0\nbot 0 gives low to output 2 and high to output 0\nvalue 2 goes to bot 2";
        assert_eq!("Output value product: 30", solve_part1(input))
    }
}
