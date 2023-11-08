#![allow(unused)]

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::{collections::BTreeSet, fmt::Display};

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

#[derive(Debug, PartialEq)]
enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

#[derive(Clone)]
struct Screen {
    pixels: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn modify(&mut self, command: &Command) {
        match *command {
            Command::Rect(width, height) => {
                for row in 0..height {
                    for col in 0..width {
                        self.pixels[row as usize][col as usize] = true;
                    }
                }
            }
            Command::RotateRow(y, by) => {
                let temp_vec = self.pixels[y].clone();

                for i in 0..SCREEN_WIDTH {
                    let index = (SCREEN_WIDTH + i - by) % SCREEN_WIDTH;
                    self.pixels[y][i] = temp_vec[index];
                }
            }
            Command::RotateCol(x, by) => {
                let temp_vec = self.pixels.iter().map(|row| row[x]).collect::<Vec<bool>>();

                for i in 0..SCREEN_HEIGHT {
                    let index = (SCREEN_HEIGHT + i - by) % SCREEN_HEIGHT;
                    self.pixels[i][x] = temp_vec[index];
                }
            }
        }
    }

    fn count_pixels(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&b| b).count()
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pixels.iter().for_each(|pixel_row| {
            pixel_row.iter().for_each(|&pixel| {
                let ch = if pixel { '#' } else { ' ' };
                write!(f, "{}", ch);
            });
            write!(f, "\n");
        });
        Ok(())
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    fn parse_rect(input: &str) -> IResult<&str, Command> {
        preceded(
            tag("rect "),
            nom::combinator::map(
                separated_pair(complete::u32, tag("x"), complete::u32),
                |(width, height)| Command::Rect(width as usize, height as usize),
            ),
        )(input)
    }

    fn parse_rotate(input: &str) -> IResult<&str, Command> {
        let rotate_row_parser = preceded(
            tag("row y="),
            nom::combinator::map(
                separated_pair(complete::u32, tag(" by "), complete::u32),
                |(idx, px)| Command::RotateRow(idx as usize, px as usize),
            ),
        );
        let rotate_col_parser = preceded(
            tag("column x="),
            nom::combinator::map(
                separated_pair(complete::u32, tag(" by "), complete::u32),
                |(idx, px)| Command::RotateCol(idx as usize, px as usize),
            ),
        );

        preceded(tag("rotate "), alt((rotate_col_parser, rotate_row_parser)))(input)
    }

    alt((parse_rect, parse_rotate))(input)
}

fn parse_commands(input: &str) -> Vec<Command> {
    separated_list0(newline, parse_command)(input)
        .unwrap_or_default()
        .1
}

pub fn solve_part1(input: &str) -> String {
    let mut screen = Screen::new();
    parse_commands(input)
        .iter()
        .for_each(|cmd| screen.modify(cmd));

    format!("{}", screen.count_pixels())
}

pub fn solve_part2(input: &str) -> String {
    let mut screen = Screen::new();
    parse_commands(input)
        .iter()
        .for_each(|cmd| screen.modify(cmd));

    format!("{}", screen)
}

#[cfg(test)]
mod test_day8 {
    use crate::solutions::day8::*;

    #[test]
    fn test_parse_commands() {
        assert_eq!(Command::Rect(3, 2), parse_command("rect 3x2").unwrap().1);
        assert_eq!(
            Command::RotateCol(1, 1),
            parse_command("rotate column x=1 by 1").unwrap().1
        );
        assert_eq!(
            Command::RotateRow(0, 4),
            parse_command("rotate row y=0 by 4").unwrap().1
        );
    }

    #[test]
    fn test_parse_command_list() {
        assert_eq!(
            vec![Command::Rect(3, 2), Command::RotateCol(5, 2)],
            parse_commands("rect 3x2\nrotate column x=5 by 2")
        )
    }

    #[test]
    fn test_part1() {
        let input = "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1";
        let expected = "6";
        assert_eq!(expected, solve_part1(input))
    }
}
