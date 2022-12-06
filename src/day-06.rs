// https://adventofcode.com/2022/day/5

mod utils;

use itertools::Itertools;
use std::iter::Iterator;

use crate::utils::input_for_day;
use std::char;
use std::collections::HashSet;

fn get_marker_idx(window_size: usize) -> usize {
    let chars = input_for_day(6).next().unwrap().chars().collect_vec();
    let sliding_windows = chars.windows(window_size).map(|window| window.to_vec());

    for (idx, window) in sliding_windows.enumerate() {
        if HashSet::<char>::from_iter(window).len() == window_size {
            return idx + window_size;
        }
    }
    unreachable!()
}

fn part_one() -> usize {
    get_marker_idx(4)
}

fn part_two() -> usize {
    get_marker_idx(14)
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two());
    Ok(())
}
