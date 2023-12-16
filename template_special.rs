#![allow(unused)]
use aoc_2023_rust::special::parse_args_special;
use std::io::BufRead;
// INJECT_USE_DAY_MODULE

fn run_with_input(input: &mut dyn BufRead) {}

fn run_without_input() {}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}
