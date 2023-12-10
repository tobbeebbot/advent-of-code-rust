#![allow(unused)]

use std::collections::{HashMap, HashSet};

use itertools::{iterate, unfold, Itertools};
use nom::{
    character::complete::{anychar, multispace0},
    multi::many1,
    sequence::terminated,
    IResult, Map, Parser,
};
use nom_locate::LocatedSpan;
use num::Integer;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq)]
enum PipeType {
    CornerUp,
    CornerDown,
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
struct Pipe {
    end1: Direction,
    end2: Direction,
}
impl Pipe {
    fn has_connection(&self, from: &Direction) -> bool {
        [&self.end1, &self.end2].contains(&from)
    }

    fn get_type(&self) -> PipeType {
        match (self.end1.clone(), self.end2.clone()) {
            (Direction::North, Direction::North) => panic!(),
            (Direction::East, Direction::East) => panic!(),
            (Direction::South, Direction::South) => panic!(),
            (Direction::West, Direction::West) => panic!(),
            (Direction::North, Direction::South) => PipeType::Vertical,
            (Direction::South, Direction::North) => PipeType::Vertical,
            (Direction::East, Direction::West) => PipeType::Horizontal,
            (Direction::West, Direction::East) => PipeType::Horizontal,
            (Direction::North, Direction::West) => PipeType::CornerUp,
            (Direction::North, Direction::East) => PipeType::CornerUp,
            (Direction::East, Direction::North) => PipeType::CornerUp,
            (Direction::West, Direction::North) => PipeType::CornerUp,
            (Direction::South, Direction::East) => PipeType::CornerDown,
            (Direction::South, Direction::West) => PipeType::CornerDown,
            (Direction::East, Direction::South) => PipeType::CornerDown,
            (Direction::West, Direction::South) => PipeType::CornerDown,
        }
    }
}

enum MapElement {
    Pipe(Pipe),
    Start,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn travel(&self, dir: &Direction) -> (Direction, Coordinate) {
        (
            dir.opposite(),
            match dir {
                Direction::North => Coordinate {
                    x: self.x,
                    y: self.y - 1,
                },
                Direction::East => Coordinate {
                    x: self.x + 1,
                    y: self.y,
                },
                Direction::South => Coordinate {
                    x: self.x,
                    y: self.y + 1,
                },
                Direction::West => Coordinate {
                    x: self.x - 1,
                    y: self.y,
                },
            },
        )
    }
}

fn parse_start_and_pipes(input: Span) -> (Coordinate, HashMap<Coordinate, Pipe>) {
    fn parse_map_element(input: Span) -> IResult<Span, (Coordinate, MapElement)> {
        use Direction::*;

        let pos = Coordinate {
            x: input.get_column(),
            y: input.location_line() as usize,
        };
        let (input, c) = anychar(input)?;

        let elem = match c {
            '-' => MapElement::Pipe(Pipe {
                end1: West,
                end2: East,
            }),
            '|' => MapElement::Pipe(Pipe {
                end1: North,
                end2: South,
            }),
            'L' => MapElement::Pipe(Pipe {
                end1: North,
                end2: East,
            }),
            'F' => MapElement::Pipe(Pipe {
                end1: South,
                end2: East,
            }),
            'J' => MapElement::Pipe(Pipe {
                end1: North,
                end2: West,
            }),
            '7' => MapElement::Pipe(Pipe {
                end1: West,
                end2: South,
            }),
            'S' => MapElement::Start,
            '.' => MapElement::Empty,
            _ => panic!("Unknown character"),
        };

        Ok((input, (pos, elem)))
    }

    let (_, elements) =
        many1(terminated(parse_map_element, multispace0))(input).expect("Parsing works");

    let mut map = HashMap::new();
    let mut start = None;

    elements.into_iter().for_each(|(pos, elem)| match elem {
        MapElement::Pipe(pipe) => {
            map.insert(pos, pipe);
        }
        MapElement::Start => {
            start = Some(pos);
        }
        MapElement::Empty => {}
    });

    (start.expect("Should find start"), map)
}

pub fn solve_part1(input: &str) -> String {
    let (start, map) = parse_start_and_pipes(Span::new(input));

    // Assumption that first step is to the east. Works for puzzle input.
    let begin = start.travel(&Direction::East);

    let loop_length = iterate(begin, |(came_from, pos)| {
        let Some(pipe) = map.get(pos) else {
            // Need this hack since start is not in the hashmap
            return (Direction::North, start.clone());
        };

        let next_dir = if (pipe.end1 == *came_from) {
            pipe.end2.clone()
        } else {
            pipe.end1.clone()
        };

        pos.travel(&next_dir)
    })
    .take_while(|(_, pos)| *pos != start)
    .count();

    // Add two because we don't count the last step or the first..
    let half_loop = (loop_length + 2) / 2;
    half_loop.to_string()
}

fn start_pipe(pos: &Coordinate, map: &HashMap<Coordinate, Pipe>) -> Pipe {
    let mut neighs = [
        Direction::East,
        Direction::West,
        Direction::North,
        Direction::South,
    ]
    .into_iter()
    .map(|dir| pos.travel(&dir))
    .flat_map(|(from, coord)| {
        map.get(&coord)
            .and_then(|p| p.has_connection(&from).then_some(from.opposite()))
    });

    Pipe {
        end1: neighs
            .next()
            .expect("Start point has two compatable neighbors"),
        end2: neighs
            .next()
            .expect("Start point has two compatable neighbors"),
    }
}

pub fn solve_part2(input: &str) -> String {
    let (start, mut map) = parse_start_and_pipes(Span::new(input));

    // Figure out the type of the start pipe
    let first_pipe = start_pipe(&start, &map);
    let start_dir = first_pipe.end1.clone();
    map.insert(start.clone(), first_pipe);

    // Find the loop of pipes and create a new hashmap
    let pipes = iterate((start_dir, start), |(came_from, pos)| {
        let pipe = map.get(pos).expect("should find pipe");

        let next_dir = if (pipe.end1 == *came_from) {
            &pipe.end2
        } else {
            &pipe.end1
        };

        pos.travel(next_dir)
    })
    .map(|(_, pos)| {
        (
            pos,
            map.get(&pos)
                .and_then(|pipe| Some(pipe.get_type()))
                .expect("Will find a pipe"),
        )
    })
    .skip(1)
    .take_while_inclusive(|(pos, _)| *pos != start)
    .collect::<HashMap<Coordinate, PipeType>>();

    // Loop through the indices of the input line by line and count inside coords
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let (in_count, _, _) = line.chars().enumerate().fold(
                (0_u32, 0_u32, None),
                |(in_count, pipe_count, last_corner), (x, _)| {
                    let coord = Coordinate { x: x + 1, y: y + 1 };

                    let pipe_type = pipes.get(&coord);

                    match pipe_type {
                        Some(PipeType::Vertical) => (in_count, pipe_count + 1, last_corner),
                        Some(PipeType::Horizontal) => (in_count, pipe_count, last_corner),
                        Some(corner) if last_corner.is_none() => {
                            (in_count, pipe_count, Some(corner))
                        }
                        Some(corner) => {
                            if corner == last_corner.unwrap() {
                                (in_count, pipe_count, last_corner)
                            } else {
                                (in_count, pipe_count + 1, last_corner)
                            }
                        }
                        None if pipe_count.is_odd() => (in_count + 1, pipe_count, last_corner),
                        None => (in_count, pipe_count, last_corner),
                    }
                },
            );
            dbg!(in_count)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let expected = "4";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let expected = "4";
        assert_eq!(expected, solve_part2(input))
    }

    #[test]
    fn test_part2b() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let expected = "1";
        assert_eq!(expected, solve_part2(input))
    }

    #[test]
    fn test_part2c() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let output = "8";

        assert_eq!(output, solve_part2(input))
    }
}
