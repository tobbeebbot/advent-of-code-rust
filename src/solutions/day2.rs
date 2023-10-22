#![allow(unused)]

// https://adventofcode.com/2016/day/2

pub fn solve_part1(input: &str) -> String {
    input
        .lines()
        .scan(5u32, |last_button_pressed, row| {
            let new_press =
                row.chars()
                    .fold(last_button_pressed.clone(), |number, dir| match dir {
                        'U' if number > 3 => number - 3,
                        'D' if number < 7 => number + 3,
                        'L' if number % 3 != 1 => number - 1,
                        'R' if number % 3 != 0 => number + 1,
                        _ => number,
                    });
            *last_button_pressed = new_press;
            Some(new_press.to_string())
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct KeyPos {
    x: u32,
    y: u32,
}

impl KeyPos {
    fn up(self) -> KeyPos {
        KeyPos {
            y: self.y - 1,
            ..self
        }
    }
    fn down(self) -> KeyPos {
        KeyPos {
            y: self.y + 1,
            ..self
        }
    }
    fn left(self) -> KeyPos {
        KeyPos {
            x: self.x - 1,
            ..self
        }
    }
    fn right(self) -> KeyPos {
        KeyPos {
            x: self.x + 1,
            ..self
        }
    }

    fn get_key(&self) -> char {
        let key_pad = vec![
            vec!['_', '_', '_', '_', '_', '_', '_'],
            vec!['_', '_', '_', '1', '_', '_', '_'],
            vec!['_', '_', '2', '3', '4', '_', '_'],
            vec!['_', '5', '6', '7', '8', '9', '_'],
            vec!['_', '_', 'A', 'B', 'C', '_', '_'],
            vec!['_', '_', '_', 'D', '_', '_', '_'],
            vec!['_', '_', '_', '_', '_', '_', '_'],
        ];
        key_pad[self.y as usize][self.x as usize]
    }

    fn valid_or(self, alternative: KeyPos) -> KeyPos {
        let key = self.get_key();

        if key == '_' {
            alternative
        } else {
            self
        }
    }
}

pub fn solve_part2(input: &str) -> String {
    let start_pos = KeyPos { x: 1, y: 4 }; // The five

    input
        .lines()
        .scan(start_pos, |previous_click_pos, line| {
            let new_click_pos =
                line.chars()
                    .fold(previous_click_pos.clone(), |previous_finder_pos, dir| {
                        let new_finger_pos = match dir {
                            'U' => previous_finder_pos.up(),
                            'D' => previous_finder_pos.down(),
                            'L' => previous_finder_pos.left(),
                            'R' => previous_finder_pos.right(),
                            _ => panic!("Invalid input"),
                        };

                        new_finger_pos.valid_or(previous_finder_pos)
                    });
            *previous_click_pos = new_click_pos;
            Some(new_click_pos.get_key())
        })
        .collect()
}

#[cfg(test)]
mod test_day2 {
    use crate::solutions::day2::*;

    #[test]
    fn test_parse_one() {
        let input = "ULL";
        let expected = "1";

        assert_eq!(expected, solve_part1(input));
    }

    #[test]
    fn test_part1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let expected = "1985";

        assert_eq!(expected, solve_part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        let expected = "5DB3";

        assert_eq!(expected, solve_part2(input));
    }
}
