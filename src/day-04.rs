// https://adventofcode.com/2022/day/4

mod utils;

use crate::utils::input_for_day;

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(range_str: &str) -> Self {
        let (start_str, end_str) = range_str.split_once("-").unwrap();
        let (start, end) = (
            start_str.parse::<i32>().unwrap(),
            end_str.parse::<i32>().unwrap(),
        );
        return Range { start, end };
    }
}

fn parse_line(line: &str) -> (Range, Range) {
    let (range_str_a, range_str_b) = line.split_once(",").unwrap();
    (Range::from_str(&range_str_a), Range::from_str(&range_str_b))
}

fn is_subset(range: &Range, other_range: &Range) -> bool {
    other_range.start <= range.start && other_range.end >= range.end
}

fn is_either_subset(range_a: &Range, range_b: &Range) -> bool {
    is_subset(&range_a, &range_b) || is_subset(&range_b, &range_a)
}

fn is_overlapping_range(range: &Range, other_range: &Range) -> bool {
    let mut ranges = vec![range, other_range];
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let (early_range, later_range) = (ranges[0], ranges[1]);
    early_range.end >= later_range.start
}

fn count_filtered_ranges(filter_fn: fn(&Range, &Range) -> bool) -> usize {
    let lines = input_for_day(4);
    let range_pairs = lines.map(|line| parse_line(&line));
    let subset_pairs = range_pairs.filter(|(a, b)| filter_fn(a, b));
    subset_pairs.count()
}

fn part_one() -> usize {
    count_filtered_ranges(is_either_subset)
}

fn part_two() -> usize {
    count_filtered_ranges(is_overlapping_range)
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two());
    Ok(())
}