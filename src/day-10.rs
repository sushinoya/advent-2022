// https://adventofcode.com/2022/day/10

mod utils;
use crate::utils::input_for_day;
use std::iter::Iterator;

enum Inst {
    NoOp,
    AddX(i16),
}

impl Inst {
    fn from_str(line: String) -> Self {
        return match &*line {
            "noop" => Inst::NoOp,
            _ => {
                let (_, val) = line.split_once(" ").unwrap();
                Inst::AddX(val.parse().unwrap())
            }
        };
    }
}

fn incr_and_update_signal(x: &i32, cycle: &mut i32, signal_strengths: &mut i32) {
    *cycle += 1;
    if (*cycle - 20) % 40 == 0 {
        *signal_strengths += *x * *cycle;
    }
}

fn part_one() -> i32 {
    let instructions = input_for_day(10).map(Inst::from_str);
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strengths: i32 = 0;

    for inst in instructions {
        match inst {
            Inst::AddX(val) => {
                incr_and_update_signal(&x, &mut cycle, &mut signal_strengths);
                incr_and_update_signal(&x, &mut cycle, &mut signal_strengths);
                x += val as i32;
            }
            Inst::NoOp => {
                incr_and_update_signal(&x, &mut cycle, &mut signal_strengths);
            }
        }
    }
    signal_strengths
}

type CRTRow<'a> = [&'a str; 40];
type CRTPanel<'a> = [CRTRow<'a>; 6];

trait StringSerializable {
    fn to_str(&self) -> String;
}

impl StringSerializable for CRTPanel<'_> {
    fn to_str(&self) -> String {
        self.map(|row| row.join(" ")).join("\n")
    }
}

fn incr_and_update_panel(x: &i32, cycle: &mut i32, panel: &mut CRTPanel) {
    *cycle += 1;
    let (cycle_x, cycle_y) = (*cycle / 40, *cycle % 40);
    for sprite_pixel in [*x, *x + 1, *x + 2] {
        if cycle_y == sprite_pixel {
            panel[cycle_x as usize][cycle_y as usize] = "#"
        }
    }
}

fn part_two() -> CRTPanel<'static> {
    let instructions = input_for_day(10).map(Inst::from_str);
    let mut panel: CRTPanel = [["."; 40]; 6];
    let mut x = 1;
    let mut cycle = 0;

    for inst in instructions {
        match inst {
            Inst::AddX(val) => {
                incr_and_update_panel(&x, &mut cycle, &mut panel);
                incr_and_update_panel(&x, &mut cycle, &mut panel);
                x += val as i32;
            }
            Inst::NoOp => {
                incr_and_update_panel(&x, &mut cycle, &mut panel);
            }
        }
    }
    panel
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two().to_str());
    Ok(())
}
