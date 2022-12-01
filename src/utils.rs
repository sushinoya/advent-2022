use log::info;
use std::fs::File;
use std::io::BufReader;

pub fn input_for_day(day: i8) -> BufReader<File> {
    let input_filepath = format!("inputs/day-{:02}.txt", day);
    info!("Using input file - {}", input_filepath);
    let file: File = File::open(input_filepath).unwrap();
    return BufReader::new(file);
}
