#![allow(unused)]

// https://adventofcode.com/2016/day/1

use std::collections::HashSet;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    multi::fold_many0,
    IResult,
};

#[derive(PartialEq, Debug)]
struct WalkInstruction {
    turn_direction: TurnDirection,
    distance: u32,
}

#[derive(PartialEq, Clone, Debug)]
enum TurnDirection {
    Left,
    Right,
    Forward,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
struct Walker {
    location: Location,
    dir: Direction,
}

fn parse_turn_walk(input: &str) -> Vec<WalkInstruction> {
    fn parse_turn(input: &str) -> IResult<&str, WalkInstruction> {
        let (input, out) = alt((tag("L"), tag("R")))(input)?;
        let (input, distance) = map_res(digit1, str::parse)(input)?;
        let (input, _) = opt(tag(", "))(input)?;

        let turn_direction = match out {
            "R" => TurnDirection::Right,
            "L" => TurnDirection::Left,
            _ => panic!("should never happen"),
        };
        Ok((
            input,
            WalkInstruction {
                turn_direction,
                distance,
            },
        ))
    }

    let (_, turns) = fold_many0(parse_turn, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(input)
    .unwrap_or_default();

    turns
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }
}

impl Walker {
    fn turn_left(self) -> Walker {
        Walker {
            dir: self.dir.left(),
            ..self
        }
    }

    fn turn_right(self) -> Walker {
        Walker {
            dir: self.dir.right(),
            ..self
        }
    }

    fn walk(self, distance: &u32) -> Walker {
        let distance = *distance as i32;
        let new_location = match self.dir {
            Direction::NORTH => Location {
                y: self.location.y - distance,
                ..self.location
            },
            Direction::WEST => Location {
                x: self.location.x - distance,
                ..self.location
            },
            Direction::SOUTH => Location {
                y: self.location.y + distance,
                ..self.location
            },
            Direction::EAST => Location {
                x: self.location.x + distance,
                ..self.location
            },
        };

        Walker {
            location: new_location,
            ..self
        }
    }

    fn turn_walk(self, instr: &WalkInstruction) -> Walker {
        match instr.turn_direction {
            TurnDirection::Left => self.turn_left().walk(&instr.distance),
            TurnDirection::Right => self.turn_right().walk(&instr.distance),
            TurnDirection::Forward => self.walk(&instr.distance),
        }
    }

    fn origin() -> Walker {
        Walker {
            location: Location::origin(),
            dir: Direction::NORTH,
        }
    }
}

impl Location {
    fn origin() -> Location {
        Location { x: 0, y: 0 }
    }

    fn distance_to(&self, origin: Location) -> u32 {
        self.x.abs_diff(origin.x) + self.y.abs_diff(origin.x)
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let turns = parse_turn_walk(input);

    let end_loc = turns
        .iter()
        .fold(Walker::origin(), |acc, elem| acc.turn_walk(elem));

    end_loc.location.distance_to(Location::origin())
}

// Find the first location visited twice
pub fn solve_part2(input: &str) -> u32 {
    let turns = parse_turn_walk(input);

    let end_loc = turns
        .iter()
        .flat_map(|wi| {
            // Expands each instruction into many so that only one block is walked at a time
            (0..wi.distance).map(|i| {
                if i == 0 {
                    WalkInstruction {
                        distance: 1,
                        turn_direction: wi.turn_direction.clone(),
                    }
                } else {
                    WalkInstruction {
                        distance: 1,
                        turn_direction: TurnDirection::Forward,
                    }
                }
            })
        })
        .scan(
            (HashSet::<Location>::new(), Walker::origin()),
            |(visited, walker), instr| {
                // before we walk
                let not_duplicate = visited.insert(walker.location);

                *walker = walker.turn_walk(&instr);

                if not_duplicate {
                    Some(walker.location)
                } else {
                    None
                }
            },
        )
        .last()
        .unwrap();

    end_loc.distance_to(Location::origin())
}

#[cfg(test)]
mod test_day1 {
    use super::*;

    #[test]
    fn test_parse_turn_walk() {
        assert_eq!(
            vec![WalkInstruction {
                turn_direction: TurnDirection::Left,
                distance: 5
            }],
            parse_turn_walk("L5")
        );
        assert_eq!(
            vec![WalkInstruction {
                turn_direction: TurnDirection::Right,
                distance: 52
            }],
            parse_turn_walk("R52")
        );
        assert_eq!(
            vec![
                WalkInstruction {
                    turn_direction: TurnDirection::Left,
                    distance: 1
                },
                WalkInstruction {
                    turn_direction: TurnDirection::Right,
                    distance: 3
                },
                WalkInstruction {
                    turn_direction: TurnDirection::Left,
                    distance: 23
                }
            ],
            parse_turn_walk("L1, R3, L23")
        );
    }

    #[test]
    fn test_simple() {
        let input = "R2, L3";
        let expected = 5;
        let result = solve_part1(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_half_circle() {
        let input = "R2, R2, R2";
        let expected = 2;
        let result = solve_part1(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_long_example() {
        let input = "R5, L5, R5, R3";
        let expected = 12;
        let result = solve_part1(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_visited_twice() {
        let input = "R8, R4, R4, R8";
        let expected = 4;
        let result = solve_part2(input);

        assert_eq!(result, expected);
    }
}
