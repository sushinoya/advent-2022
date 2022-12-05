// https://adventofcode.com/2022/day/5

mod utils;

use itertools::Itertools;
use std::iter::Iterator;

use crate::utils::input_for_day;
use std::char;
use std::collections::HashMap;
use std::io::BufRead;

struct Move {
    count: i8,
    from: i8,
    to: i8,
}

impl Move {
    fn from_str(line: &str) -> Self {
        let mut parts = line
            .split(' ')
            .filter(|c| c.chars().all(|c| c.is_ascii_digit()))
            .map(|part| part.parse::<i8>().unwrap());

        Move {
            count: parts.next().unwrap(),
            from: parts.next().unwrap(),
            to: parts.next().unwrap(),
        }
    }
}

fn parse_moves() -> impl Iterator<Item = Move> {
    let lines = input_for_day(5)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| line.starts_with("move"));
    lines.map(|line| Move::from_str(&line))
}

// Each stack entry takes 3 characters. And one space between each stack entry.
// 3n + (n - 1) = str_size where n is the number of entries.
fn parse_stack_items(line: &str) -> Vec<char> {
    let chars = line.chars().collect_vec();
    (1..chars.len())
        .step_by(4) // There's one item character at every 4th char
        .map(|idx| chars.get(idx).unwrap().clone())
        .collect_vec()
}

fn parse_stacks() -> HashMap<i8, Vec<char>> {
    let stacks: &mut HashMap<i8, Vec<char>> = &mut HashMap::new();
    let lines = input_for_day(5).lines().map(|line| line.unwrap());

    for line in lines {
        // Reached the end of the stack definition; break
        if line.is_empty() {
            break;
        }

        for (i, val) in parse_stack_items(&line).iter().enumerate() {
            let stack_idx: i8 = 1 + i as i8; // Change from 0-indexed to 1-indexed
            if !stacks.contains_key(&stack_idx) {
                stacks.insert(stack_idx, vec![*val]);
            } else {
                stacks.get_mut(&stack_idx).unwrap().push(*val)
            }
        }
    }

    for stack in stacks.values_mut() {
        stack.pop(); // Remove the stack indexes
        stack.retain(|x| *x != ' '); // Filter empty chars
        stack.reverse(); // Reverse to mimic reading bottom up
    }

    stacks.clone()
}

fn repr_for_stacks(stacks: &HashMap<i8, Vec<char>>) -> String {
    let sorted_stack_vals = stacks.keys().sorted().map(|key| stacks.get(key).unwrap());
    String::from(
        &sorted_stack_vals
            .map(|stack| stack.last().unwrap_or(&' '))
            .join(""),
    )
}

fn part_one() -> String {
    let mut stacks = parse_stacks();
    for item_move in parse_moves() {
        for _ in 0..item_move.count {
            let from_stack = stacks.get_mut(&item_move.from).unwrap();
            let item_to_insert = from_stack.pop().unwrap();
            stacks.get_mut(&item_move.to).unwrap().push(item_to_insert);
        }
    }
    repr_for_stacks(&stacks)
}

fn part_two() -> String {
    let mut stacks = parse_stacks();
    for item_move in parse_moves() {
        let from_stack = stacks.get_mut(&item_move.from).unwrap();
        let mut items_to_insert: Vec<char> = Vec::new();

        for _ in 0..item_move.count {
            items_to_insert.push(from_stack.pop().unwrap());
        }

        items_to_insert.reverse();
        stacks
            .get_mut(&item_move.to)
            .unwrap()
            .append(&mut items_to_insert);
    }
    repr_for_stacks(&stacks)
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two());
    Ok(())
}
