#![allow(unused)]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
    springs_idx_start: usize,
) -> i32 {
    let is_log_on = false;
    if springs_idx_start == 0 && is_log_on {
        println!("============================",);
        println!(
            "{} {}",
            springs
                .iter()
                .map(|s| s.to_char().to_string())
                .collect::<String>(),
            damaged_lengths
                .iter()
                .map(|x| {
                    let mut s = x.to_string();
                    s.push(',');
                    s
                })
                .collect::<String>(),
        );
    }

    for (spring_idx, spring) in springs.iter().enumerate().skip(springs_idx_start) {
        if is_log_on {
            println!("char:'{}' spring_idx: {}", spring.to_char(), spring_idx);
        }
        match spring {
            Status::Operational => {
                // if some group has acc'ed, attempt to close it
                // if acc and length dont match, return 0
                if acc > 0 {
                    if acc != damaged_lengths[group_idx] {
                        if is_log_on {
                            println!("NO, lower dmgd than group indicated",);
                        }
                        return 0;
                    }
                    group_idx += 1;
                }
                acc = 0;
            }
            Status::Damaged => {
                // NO, found new damaged group after all record groups already matched
                if group_idx == damaged_lengths.len() {
                    if is_log_on {
                        println!("NO, MATCHED but extra",);
                    }
                    return 0;
                }
                acc += 1;
                // NO, more damaged in current group than specified in record
                if acc > damaged_lengths[group_idx] {
                    if is_log_on {
                        println!("NO, extra dmg found for current dmg_group",);
                    }
                    return 0;
                }
            }
            Status::Unknown => {
                let mut branched_with_operational = springs.clone();
                branched_with_operational[spring_idx] = Status::Operational;

                let mut branched_with_damaged = springs.clone();
                branched_with_damaged[spring_idx] = Status::Damaged;

                if is_log_on {
                    println!("----------BRANCHING----------",);
                }
                let result = branching_match(
                    damaged_lengths.clone(),
                    branched_with_operational,
                    group_idx,
                    acc,
                    spring_idx,
                ) + branching_match(
                    damaged_lengths.clone(),
                    branched_with_damaged,
                    group_idx,
                    acc,
                    spring_idx,
                );
                if is_log_on {
                    println!(
                        "BRANCH RESULT: {} for {}",
                        result,
                        springs.iter().map(|s| s.to_char()).collect::<String>()
                    );
                }
                return result;
            }
        }
        if is_log_on {
            println!("i: {} acc: {}  g_i: {}", spring_idx, acc, group_idx);
        }
    }
    // need to do a closing check, since the record group length check doesnt occur if springs ends with damaged, repeat the '.' branch logic
    if acc > 0 {
        if acc != damaged_lengths[group_idx] {
            if is_log_on {
                println!("NO, lower dmgd than group indicated",);
            }
            return 0;
        }
        group_idx += 1;
    }

    // ! culls too much, because if springs ends with a damaged, this always
    // ! false; corrected with above block
    if group_idx != damaged_lengths.len() {
        return 0;
    }

    // reached EORow, if made it this far, good as an arrangement!
    return 1;
}

// SOLN: 7007 (after adding the grp check after loop, as in '.' match arm)
// 83 wrong (left the skip/take)
// 24 wrong (added return for when a branch resulted in an instantiated grp smaller than record grp, left the skip/take)
// 22746 high

// TODO need to cull when so far groups have matched but not all record groups matched
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
        let arrangements = branching_match(
            record.damaged_lengths.clone(),
            record.springs.clone(),
            0,
            0,
            0,
        );
        // println!("=> {}", arrangements);

        sum_all_arrangements += arrangements;
    }
    // println!("sum: {}", sum_all_arrangements);

    sum_all_arrangements.to_string()
}

// SOLN:
pub fn second(input: &mut dyn BufRead) -> String {
    let mut records: Vec<Record> = vec![];
    for line in get_string_from_input(input).lines() {
        let (springs, conditions) = line.split_once(' ').unwrap();

        // setup an iterator with the prepended copies of original
        let extra_springs_iter = std::iter::repeat(springs)
            .take(4)
            .map(|s| format!("?{}", s));
        // join into string
        let extra_springs_joined = extra_springs_iter.collect::<Vec<_>>().join("");
        // join to original
        let new_springs = format!("{}{}", springs, extra_springs_joined);

        // setup an iterator with the prepended copies of original
        let extra_conditions_iter = std::iter::repeat(conditions)
            .take(4)
            .map(|g| format!(",{}", g));
        // join into string
        let extra_conditions_joined = extra_conditions_iter.collect::<Vec<_>>().join("");
        // join to original
        let new_conditions = format!("{}{}", conditions, extra_conditions_joined);

        records.push(Record::new(&new_springs, &new_conditions));
    }

    let mut sum_all_arrangements = 0;
    // for record in records {
    //     let arrangements = branching_match(
    //         record.damaged_lengths.clone(),
    //         record.springs.clone(),
    //         0,
    //         0,
    //         0,
    //     );
    //     println!("=> {}", arrangements);

    //     sum_all_arrangements += arrangements;
    // }
    let mut sum: u128 = records
        .par_iter()
        .map(|record| {
            branching_match(
                record.damaged_lengths.clone(),
                record.springs.clone(),
                0,
                0,
                0,
            ) as u128
        })
        .sum();
    println!("sum: {}", sum);

    sum_all_arrangements.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
