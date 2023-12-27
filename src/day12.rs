#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    pub fn to_char(&self) -> char {
        match self {
            Status::Operational => '.',
            Status::Damaged => '#',
            Status::Unknown => '?',
        }
    }
}

pub struct Record {
    springs: Vec<Status>,
    groups: Vec<i32>,
}

impl Record {
    pub fn new(springs: &str, groups: &str) -> Self {
        let springs: Vec<Status> = springs
            .chars()
            .map(|c| match c {
                '#' => Status::Damaged,
                '.' => Status::Operational,
                '?' => Status::Unknown,
                _ => panic!("Invalid char in springs string"),
            })
            .collect();
        let groups: Vec<i32> = groups
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Self { springs, groups }
    }
}

// SOLN:
pub fn first(input: &mut dyn BufRead) -> String {
    let mut records: Vec<Record> = vec![];
    for line in get_string_from_input(input).lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        records.push(Record::new(springs, groups));
    }

    let mut sum_all_arrangements = 0;
    for record in records {
        let mut arrangements = 0;
        let groups = record.groups;
        println!("============================",);
        println!(
            "{} {}",
            record
                .springs
                .iter()
                .map(|s| s.to_char().to_string())
                .collect::<String>(),
            groups
                .iter()
                .map(|x| {
                    let mut s = x.to_string();
                    s.push(',');
                    s
                })
                .collect::<String>(),
        );

        let mut groups_idx = 0;
        let mut acc = 0;
        // TODO need to recurse upon '?'
        for spring in &record.springs {
            // skip across contiguous operationals
            if *spring == Status::Operational {
                continue;
            }
            // match group (len of damaged) to springs sequence
            while acc < groups[groups_idx] {
                match spring {
                    Status::Operational => {
                        continue;
                    }
                    Status::Damaged => {
                        acc += 1;
                    }
                    Status::Unknown => {
                        acc += 1;
                    }
                }
            }
            // TODO somewhere, once a group is matched, check for EORow or Operational separator
            if acc < groups[groups_idx] {
                println!(
                    "no match for groups[{}]: {}",
                    groups_idx, groups[groups_idx]
                );
            } else {
                // ! wrong, arrangements not until EORow reached
                arrangements += 1;
            }

            // setup for next group
            groups_idx += 1;
            if groups_idx == groups.len() {
                break;
            }
            acc = 0;
        }
        println!("=> {}", arrangements);
    }

    "".to_string()
}

// SOLN:
fn second(input: &mut dyn BufRead) -> String {
    for line in get_string_from_input(input).lines() {}

    "".to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
