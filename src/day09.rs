#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// SOLN: 2008960228 bad while condition, use .any (not .all)
// 1517404030 low
// 1725850617 low (after using isize instead of usize)
//
fn first(input: &mut dyn BufRead) -> String {
    let mut sum = 0;
    for line in get_string_from_input(input).lines() {
        let mut values: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        let mut tail_vals = vec![];

        while values.iter().any(|&x| x != 0) {
            // keep last value
            tail_vals.push(values[values.len() - 1]);

            // create next row
            let mut next_vals = vec![];
            for i in 0..values.len() - 1 {
                next_vals.push(values[i + 1] - values[i]);
            }
            values = next_vals;
        }
        sum += tail_vals.iter().sum::<isize>();
    }

    sum.to_string()
}

// SOLN: 1097 (was iterating over head_vals in wrong direction)
// -20863 wrong
// -137 wrong flipped fold, subtract from sum
fn second(input: &mut dyn BufRead) -> String {
    let mut sum = 0;
    for line in get_string_from_input(input).lines() {
        let mut values: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();

        let mut head_vals = vec![];

        while values.iter().any(|&x| x != 0) {
            // keep last value
            head_vals.push(values[0]);

            // create next row
            let mut next_vals = vec![];
            for i in 0..values.len() - 1 {
                next_vals.push(values[i + 1] - values[i]);
            }
            values = next_vals;
        }
        sum += head_vals.iter().rev().fold(0, |acc, curr| curr - acc);
    }

    sum.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
