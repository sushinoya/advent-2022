// https://adventofcode.com/2022/day/8
#![feature(type_alias_impl_trait)]

mod utils;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::ops::Range;

use itertools::Itertools;

use crate::utils::input_for_day;

type TreePos = (usize, usize);
type TreePosIter = impl DoubleEndedIterator<Item = TreePos>;
type NestedTreePosIter = impl Iterator<Item = TreePosIter>;

fn parse_lines(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    lines
        .map(|line| {
            line.chars()
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

// Iterates through two ranges of indices, row indices and col indices and constructs a nested Iterator
// of TreePos i.e (usize, usize). If for example we want a Nested Iterator of all indices, we'll pass
// (0..num_rows) and (0..num_cols) as params.
fn tree_lines_for_ranges(
    range_a: Range<usize>,
    range_b: Range<usize>,
    b_major: bool,
    reversed: bool,
) -> NestedTreePosIter {
    range_a.map(move |row_idx| {
        let pos_iter = range_b.clone().map(move |col_idx| match b_major {
            true => (col_idx, row_idx),
            false => (row_idx, col_idx),
        });
        match reversed {
            true => pos_iter.rev().collect::<Vec<TreePos>>().into_iter(),
            false => pos_iter.collect::<Vec<TreePos>>().into_iter(),
        }
    })
}

fn get_visible_trees_pos(mut tree_line: TreePosIter, tree_grid: &Vec<Vec<u8>>) -> HashSet<TreePos> {
    let (first_tree_row, first_tree_col) = tree_line.next().unwrap();
    let mut curr_max_height = tree_grid[first_tree_row][first_tree_col];
    let mut visible_trees = HashSet::new();

    for (row_idx, col_idx) in tree_line {
        let tree_height: u8 = tree_grid[row_idx][col_idx];
        if tree_height <= curr_max_height {
            continue;
        }
        visible_trees.insert((row_idx, col_idx));
        curr_max_height = tree_height
    }
    visible_trees
}

fn part_one() -> u32 {
    let tree_grid = parse_lines(input_for_day(8));
    let (num_rows, num_cols) = (tree_grid.len(), tree_grid[0].len());
    let is_on_edge =
        |(x, y): &TreePos| *x == 0 || *x == num_rows - 1 || *y == 0 || *y == num_cols - 1;

    // Rows and cols (iterators) to check for tree line
    let rows: NestedTreePosIter = tree_lines_for_ranges(0..num_rows, 0..num_cols, false, false);
    let cols: NestedTreePosIter = tree_lines_for_ranges(0..num_cols, 0..num_cols, true, false);
    let rev_rows: NestedTreePosIter = tree_lines_for_ranges(0..num_rows, 0..num_cols, false, true);
    let rev_cols: NestedTreePosIter = tree_lines_for_ranges(0..num_cols, 0..num_rows, true, true);

    // Edge trees are directly visible
    let edge_positions = tree_lines_for_ranges(0..num_cols, 0..num_rows, false, false)
        .flatten()
        .filter(is_on_edge);

    // Add all visible positions to a set
    let mut positions: HashSet<TreePos> = HashSet::from_iter(edge_positions);
    for line_iter in [rows, cols, rev_rows, rev_cols] {
        for line in line_iter {
            positions.extend(get_visible_trees_pos(line, &tree_grid));
        }
    }

    positions.len() as u32
}

pub fn compute_num_visible_trees(tree_line: Vec<u8>) -> Vec<usize> {
    let mut num_visible_trees = (0..tree_line.len()).collect_vec();
    let mut stack = Vec::new();

    for idx in 0..tree_line.len() {
        while !stack.is_empty() && tree_line[idx] >= tree_line[*stack.last().unwrap()] {
            let i = stack.pop().unwrap();
            num_visible_trees[i] = idx - i;
        }
        stack.push(idx);
    }
    while !stack.is_empty() {
        let i = stack.pop().unwrap();
        num_visible_trees[i] = tree_line.len() - i - 1;
    }
    num_visible_trees
}

fn part_two() -> u32 {
    let tree_grid = parse_lines(input_for_day(8));
    let (num_rows, num_cols) = (tree_grid.len(), tree_grid[0].len());
    let is_on_edge =
        |(x, y): &TreePos| *x == 0 || *x == num_rows - 1 || *y == 0 || *y == num_cols - 1;

    // Rows and cols (iterators) to check for tree line
    let rows: NestedTreePosIter = tree_lines_for_ranges(0..num_rows, 0..num_cols, false, false);
    let cols: NestedTreePosIter = tree_lines_for_ranges(0..num_cols, 0..num_cols, true, false);
    let rev_rows: NestedTreePosIter = tree_lines_for_ranges(0..num_rows, 0..num_cols, false, true);
    let rev_cols: NestedTreePosIter = tree_lines_for_ranges(0..num_cols, 0..num_rows, true, true);

    // HashMap to store score for indices
    let mut num_visible_from_pos: HashMap<TreePos, usize> = HashMap::new();

    for line_iter in [rows, cols, rev_rows, rev_cols] {
        let line_iter: NestedTreePosIter = line_iter;

        for line_indices in line_iter {
            let indices = line_indices.collect_vec();
            let tree_line: Vec<u8> = indices.iter().map(|(x, y)| tree_grid[*x][*y]).collect_vec();
            let num_visible_trees = compute_num_visible_trees(tree_line);

            for (pos, num_visible) in indices.iter().zip(num_visible_trees.iter()) {
                if num_visible_from_pos.contains_key(&pos) {
                    *num_visible_from_pos.get_mut(&pos).unwrap() *= num_visible;
                } else {
                    num_visible_from_pos.insert(*pos, *num_visible);
                }
            }
        }
    }

    *num_visible_from_pos.values().max().unwrap() as u32
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two());
    Ok(())
}
