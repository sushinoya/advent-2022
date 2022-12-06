// https://adventofcode.com/2022/day/1

mod utils;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::Iterator;

type BinaryMinHeap<T> = BinaryHeap<Reverse<T>>;

fn fixed_size_heap_push<T: Ord>(val: T, heap: &mut BinaryMinHeap<T>) {
    if heap.len() == heap.capacity() {
        heap.pop();
    }
    heap.push(Reverse(val))
}

fn main() -> Result<(), std::io::Error> {
    let mut heap: BinaryMinHeap<i64> = BinaryHeap::with_capacity(3);
    let mut curr_calories_sum: i64 = 0;

    for calories in utils::input_for_day(1) {
        if !calories.is_empty() {
            curr_calories_sum += calories.parse::<i64>().unwrap();
            continue;
        }

        if heap.is_empty() || heap.peek().unwrap().0 < curr_calories_sum {
            fixed_size_heap_push(curr_calories_sum, &mut heap);
        }

        curr_calories_sum = 0;
    }

    let sum_of_top_three: i64 = heap.iter().map(|rev_val| rev_val.0).sum();
    println!("{}", sum_of_top_three);
    Ok(())
}
