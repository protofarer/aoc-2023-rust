#![allow(unused)]
use aoc_2023_rust::day12::*;
use aoc_2023_rust::special::parse_args_special;
use std::io::BufRead;

fn run_with_input(input: &mut dyn BufRead) {
    second(input);
}

fn run_without_input() {}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}
