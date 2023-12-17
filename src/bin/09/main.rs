#![allow(unused)]
use aoc_2023_rust::special::parse_args_special;
use aoc_2023_rust::{day09::*, get_string_from_input};
use std::io::BufRead;

fn run_with_input(input: &mut dyn BufRead) {
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

        println!("head vals {:?}", head_vals);
        let predicted = head_vals.iter().rev().fold(0, |acc, curr| curr - acc);
        println!("predicted {}", predicted);
        sum += predicted;
    }
    println!("sum: {}", sum);
}

fn run_without_input() {}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}
