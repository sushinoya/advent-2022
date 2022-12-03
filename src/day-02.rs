// https://adventofcode.com/2022/day/2

mod utils;

use crate::utils::input_for_day;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::identity;
use std::io::BufRead;
use std::iter::Iterator;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i8 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        return match (self, other) {
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        };
    }
}

lazy_static! {
    static ref CHAR_TO_MOVE: HashMap<&'static str, Move> = {
        HashMap::from([
            ("A", Move::Rock),
            ("B", Move::Paper),
            ("C", Move::Scissors),

            // Un-used for part 2 after definition for {X, Y, Z} changed
            ("X", Move::Rock),
            ("Y", Move::Paper),
            ("Z", Move::Scissors),
        ])
    };
}

fn score_for_round((their_move, your_move): (Move, Move)) -> i32 {
    if their_move < your_move {
        return i32::from(6 + your_move.score());
    } else if their_move == your_move {
        return i32::from(3 + your_move.score());
    } else {
        return i32::from(your_move.score());
    }
}

// Generic input parser to parse lines into an Iterator<Move, Move> for part one
// and into Iterator<Move, Outcome> for part two of the problem.
fn parse_input<T: 'static>(
    parser: fn((&str, &str)) -> (Move, T),
) -> Box<dyn Iterator<Item = (Move, T)>> {
    let rounds = input_for_day(2)
        .lines()
        .map(move |line| line.unwrap().split_once(" ").map(parser))
        .filter_map(identity);
    Box::new(rounds)
}

fn score_for_moves(moves: Box<dyn Iterator<Item = (Move, Move)>>) -> i32 {
    return moves.map(|round_moves| score_for_round(round_moves)).sum();
}

type Outcome = String;

fn move_to_play(their_move: Move, wanted_outcome: Outcome) -> Move {
    for possible_move in [Move::Rock, Move::Paper, Move::Scissors] {
        let move_matches_wanted_outcome = (wanted_outcome == "X" && possible_move < their_move)
            || (wanted_outcome == "Y" && possible_move == their_move)
            || (wanted_outcome == "Z" && possible_move > their_move);

        if move_matches_wanted_outcome {
            return possible_move;
        }
    }
    unreachable!() // One of the possible move is guaranteed to be returned
}

#[allow(dead_code)]
fn part_one() -> Result<(), std::io::Error> {
    let players_moves = parse_input(|round_moves: (&str, &str)| {
        (CHAR_TO_MOVE[&round_moves.0], CHAR_TO_MOVE[&round_moves.1])
    });
    println!("{}", score_for_moves(players_moves));
    Ok(())
}

fn part_two() -> Result<(), std::io::Error> {
    let player_moves = parse_input(|round_moves: (&str, &str)| {
        (
            CHAR_TO_MOVE[&round_moves.0],
            move_to_play(CHAR_TO_MOVE[&round_moves.0], Outcome::from(round_moves.1)),
        )
    });
    println!("{}", score_for_moves(player_moves));
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    part_two()
}
