// https://adventofcode.com/2022/day/3

mod utils;

use crate::utils::input_for_day;
use itertools::{Chunk, Itertools};
use std::collections::HashSet;

fn chars_in_both_parts(line: &str) -> impl Iterator<Item = char> + '_ {
    let (first, second) = line.split_at(line.len() / 2);
    let first_set: HashSet<char> = first.chars().collect();
    return second
        .chars()
        .filter(move |c| first_set.contains(&c))
        .unique();
}

fn priority(chr: char) -> u32 {
    let val_of_chr = chr as u32;
    if chr.is_lowercase() {
        return 1 + val_of_chr - 'a' as u32;
    } else {
        return 27 + val_of_chr - 'A' as u32;
    }
}

fn part_one() -> u32 {
    let lines = input_for_day(3);
    let common_chars_in_lines =
        lines.map(|line| chars_in_both_parts(&line).collect::<Vec<_>>());
    let sum_of_priorities: u32 = common_chars_in_lines
        .map(|chars| chars.into_iter().map(priority).sum::<u32>())
        .sum();
    sum_of_priorities
}

fn common_char_in_group(group: Chunk<impl Iterator<Item = String>>) -> char {
    let mut sets_of_chars = group.map(|line| HashSet::from_iter(line.chars()));
    let first_set: HashSet<char> = sets_of_chars.next().unwrap();
    let common_chars = sets_of_chars.fold(first_set, |acc, other_set| {
        acc.intersection(&other_set).cloned().collect()
    });

    assert_eq!(common_chars.len(), 1);
    common_chars.into_iter().next().unwrap()
}

fn part_two() -> u32 {
    let binding = input_for_day(3).chunks(3);
    let groups = binding.into_iter();
    let group_badges = groups.map(|group| common_char_in_group(group));
    group_badges.map(priority).sum()
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two());
    Ok(())
}
