#![allow(unused)]

use std::collections::{BTreeMap, HashMap};

fn gather_stats(input: &str) -> BTreeMap<usize, HashMap<char, u32>> {
    let mut char_counts = BTreeMap::new();

    input.lines().for_each(|line| {
        line.char_indices().for_each(|(char_pos, char)| {
            char_counts
                .entry(char_pos)
                .or_insert_with(HashMap::new)
                .entry(char)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        })
    });

    return char_counts;
}

pub fn solve_part1(input: &str) -> String {
    gather_stats(input)
        .values()
        .map(|char_count| {
            char_count
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(&ch, _)| ch)
                .unwrap()
        })
        .collect()
}

pub fn solve_part2(input: &str) -> String {
    gather_stats(input)
        .values()
        .map(|char_count| {
            char_count
                .iter()
                .min_by_key(|(_, &count)| count)
                .map(|(&ch, _)| ch)
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test_day6 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        let expected = "easter";
        assert_eq!(expected, solve_part1(input))
    }
    #[test]
    fn test_part2() {
        let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
        let expected = "advent";
        assert_eq!(expected, solve_part2(input))
    }
}
