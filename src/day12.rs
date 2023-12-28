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
    damaged_lengths: Vec<i32>,
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

        Self {
            springs,
            damaged_lengths: groups,
        }
    }
}

pub fn branching_match(
    damaged_lengths: Vec<i32>,
    springs: Vec<Status>,
    mut group_idx: usize,
    mut acc: i32,
) -> i32 {
    for (spring_idx, spring) in springs.iter().enumerate() {
        // println!("char:'{}'", spring.to_char());
        match spring {
            Status::Operational => {
                // if some group has acc'ed, attempt to close it
                // if acc and length dont match, return 0
                if acc > 0 {
                    group_idx += 1;
                }
                acc = 0;
            }
            Status::Damaged => {
                // NO, found new damaged group after all record groups already matched
                if group_idx == damaged_lengths.len() {
                    return 0;
                }
                acc += 1;
                // NO, more damaged in current group than specified in record
                if acc > damaged_lengths[group_idx] {
                    return 0;
                }
            }
            Status::Unknown => {
                let mut branched_with_operational = springs.clone();
                branched_with_operational[spring_idx] = Status::Operational;

                let mut branched_with_damaged = springs.clone();
                branched_with_damaged[spring_idx] = Status::Damaged;

                // println!("----------BRANCHING----------",);
                return branching_match(
                    damaged_lengths.clone(),
                    branched_with_operational,
                    group_idx,
                    acc,
                ) + branching_match(
                    damaged_lengths.clone(),
                    branched_with_damaged,
                    group_idx,
                    acc,
                );
            }
        }
        // println!("acc: {}  g_i: {}", acc, group_idx);
    }

    // reached EORow, if made it this far, good as an arrangement!
    return 1;
}

// SOLN:
pub fn first(input: &mut dyn BufRead) -> String {
    let mut records: Vec<Record> = vec![];
    for line in get_string_from_input(input).lines() {
        let (springs, groups) = line.split_once(' ').unwrap();
        records.push(Record::new(springs, groups));
    }

    let mut sum_all_arrangements = 0;
    // call a recursive fn on each record that allows for branching
    // fn foo(acc, damaged_length, springs)
    // the base case does the actual acc += 1
    //
    for (i, record) in records.iter().enumerate() {
        println!("============================",);
        println!(
            "{} {}",
            record
                .springs
                .iter()
                .map(|s| s.to_char().to_string())
                .collect::<String>(),
            record
                .damaged_lengths
                .iter()
                .map(|x| {
                    let mut s = x.to_string();
                    s.push(',');
                    s
                })
                .collect::<String>(),
        );

        let arrangements =
            branching_match(record.damaged_lengths.clone(), record.springs.clone(), 0, 0);
        println!("=> {}", arrangements);

        sum_all_arrangements += arrangements;
    }
    println!("sum: {}", sum_all_arrangements);

    "".to_string()
}

// SOLN:
fn second(input: &mut dyn BufRead) -> String {
    for line in get_string_from_input(input).lines() {}

    "".to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
