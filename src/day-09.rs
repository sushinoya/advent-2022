// https://adventofcode.com/2022/day/9

mod utils;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops::Add;

use crate::utils::input_for_day;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i16,
    y: i16,
}

impl<'a> Add<&'a Pos> for Pos {
    type Output = Pos;
    fn add(self, other: &'a Pos) -> Pos {
        Pos {
            x: self.x + &other.x,
            y: self.y + &other.y,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> Pos {
        match *self {
            Direction::Up => Pos { x: 0, y: 1 },
            Direction::Down => Pos { x: 0, y: -1 },
            Direction::Left => Pos { x: -1, y: 0 },
            Direction::Right => Pos { x: 1, y: 0 },
        }
    }
}

struct Move {
    dir: Direction,
    steps: i8,
}

impl Move {
    fn from_str(line: String) -> Self {
        let (dir_str, steps_str) = line.split_once(" ").unwrap();
        let steps = steps_str.parse::<i8>().unwrap();

        match dir_str {
            "U" => Move {
                dir: Direction::Up,
                steps,
            },
            "D" => Move {
                dir: Direction::Down,
                steps,
            },
            "L" => Move {
                dir: Direction::Left,
                steps,
            },
            "R" => Move {
                dir: Direction::Right,
                steps,
            },
            _ => unreachable!(),
        }
    }
}

fn is_adjacent(head: Pos, tail: Pos) -> bool {
    (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1
}

fn next_tail_pos(head: Pos, tail: Pos) -> Pos {
    if is_adjacent(head, tail) {
        return tail;
    }

    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    // Same column
    if x_diff == 0 && y_diff.abs() == 1 {
        return Pos {
            x: tail.x,
            y: tail.y + y_diff.signum(),
        };
    }

    // Same row
    if y_diff == 0 && x_diff.abs() == 1 {
        return Pos {
            x: tail.x + x_diff.signum(),
            y: tail.y,
        };
    }

    // Diagonal
    Pos {
        x: tail.x + x_diff.signum(),
        y: tail.y + y_diff.signum(),
    }
}

fn part_one() -> usize {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(tail);

    let moves = input_for_day(9).map(Move::from_str);

    for _move in moves {
        for _ in 0.._move.steps {
            head = head + &_move.dir.delta();
            tail = next_tail_pos(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part_two() -> usize {
    let mut positions: HashMap<i32, Pos> =
        HashMap::from_iter((0..10).map(|part| (part, Pos { x: 0, y: 0 })));
    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(positions[&9]);

    let moves = input_for_day(9).map(Move::from_str);

    for _move in moves {
        for _ in 0.._move.steps {
            positions.insert(0, *positions.get(&0).unwrap() + &_move.dir.delta());

            for part in 1..10 {
                positions.insert(
                    part,
                    next_tail_pos(
                        *positions.get(&(&part - 1)).unwrap(),
                        *positions.get(&part).unwrap(),
                    ),
                );
            }
            visited.insert(*positions.get(&9).unwrap());
        }
    }

    visited.len()
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two());
    Ok(())
}
