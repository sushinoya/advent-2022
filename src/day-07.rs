// https://adventofcode.com/2022/day/7

mod utils;
use crate::utils::input_for_day;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type RCFile = Rc<File>;
type RCDirectory = Rc<Directory>;

const DISK_SPACE: u32 = 70000000;
const MIN_REQUIRED_SPACE: u32 = 30000000;

#[derive(Debug, Default)]
struct Directory {
    _name: String,
    parent: Weak<Directory>,
    files: RefCell<HashMap<String, RCFile>>,
    dirs: RefCell<HashMap<String, RCDirectory>>,
}

impl Directory {
    fn size(&self) -> u32 {
        let files_size: u32 = self.files.borrow().values().map(|file| file.size).sum();
        let subdirs_size: u32 = self.dirs.borrow().values().map(|dir| dir.size()).sum();
        files_size + subdirs_size
    }

    fn descendent_dirs(&self) -> Vec<RCDirectory> {
        let mut subdirs: Vec<RCDirectory> = self
            .dirs
            .borrow()
            .values()
            .clone()
            .map(|dir| dir.clone())
            .collect();
        let mut other_descendents: Vec<RCDirectory> = subdirs
            .iter()
            .map(|dir| dir.descendent_dirs())
            .flatten()
            .collect();
        subdirs.append(&mut other_descendents);
        subdirs
    }
}

#[derive(Debug)]
struct File {
    _name: String,
    size: u32,
    _parent: Weak<Directory>,
}

fn parse_lines(lines: impl Iterator<Item = String>) -> Directory {
    let base_dir = Rc::new(Directory {
        _name: String::from("/"),
        ..Default::default()
    });
    let mut curr_dir = Rc::clone(&base_dir);

    for line in lines {
        let (first_tkn, second_tkn) = line.split_once(" ").unwrap();

        match first_tkn {
            "$" => match second_tkn {
                // LS
                "ls" => {}
                // CD
                s if s.starts_with("cd") => {
                    let (_, subdir) = second_tkn.split_once(" ").unwrap();

                    match subdir {
                        // CD /
                        "/" => {
                            curr_dir = Rc::clone(&base_dir);
                        }
                        // CD ..
                        ".." => {
                            let parent = curr_dir.parent.upgrade().unwrap();
                            curr_dir = Rc::clone(&parent);
                        }
                        // CD sub-directory
                        _ => {
                            let subdirectories = curr_dir.dirs.borrow();
                            let subdir = Rc::clone(subdirectories.get(subdir).unwrap());
                            drop(subdirectories);
                            curr_dir = Rc::clone(&subdir);
                        }
                    }
                }
                _ => unreachable!(),
            },

            // dir <directory-name>
            "dir" => {
                let new_dir = Rc::new(Directory {
                    _name: second_tkn.to_string(),
                    parent: Rc::downgrade(&curr_dir),
                    ..Default::default()
                });
                curr_dir
                    .dirs
                    .borrow_mut()
                    .insert(String::from(second_tkn), new_dir);
            }
            // <size> <file-name>
            _ => {
                let new_file = Rc::new(File {
                    size: first_tkn.parse().unwrap(),
                    _name: String::from(second_tkn),
                    _parent: Rc::downgrade(&curr_dir),
                });
                curr_dir
                    .files
                    .borrow_mut()
                    .insert(String::from(second_tkn), new_file);
            }
        }
    }

    Rc::try_unwrap(base_dir).unwrap()
}

fn part_one() -> u32 {
    let file_system = parse_lines(input_for_day(7));
    file_system
        .descendent_dirs()
        .iter()
        .map(|dir| dir.size())
        .filter(|size| size <= &100000)
        .sum()
}

fn part_two() -> Option<u32> {
    let file_system = parse_lines(input_for_day(7));
    let unused_space: u32 = DISK_SPACE - file_system.size();
    if unused_space >= MIN_REQUIRED_SPACE {
        return None;
    }
    let space_to_free = MIN_REQUIRED_SPACE - unused_space;
    file_system
        .descendent_dirs()
        .iter()
        .map(|dir| dir.size())
        .filter(|size| size >= &space_to_free)
        .min()
}

fn main() -> Result<(), std::io::Error> {
    println!("{}", part_one());
    println!("{}", part_two().unwrap());
    Ok(())
}
