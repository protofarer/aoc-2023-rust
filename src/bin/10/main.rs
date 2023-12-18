#![allow(unused)]
use aoc_2023_rust::special::parse_args_special;
use aoc_2023_rust::{day10::*, get_string_from_input};
use std::io::BufRead;

fn run_with_input(input: &mut dyn BufRead) {
    println!("longest_dist {}", first(input));
}

fn run_without_input() {
    let mut map: Vec<Vec<usize>> = vec![vec![]];
    map.push(vec![]);
    map[0].push(1);
    map.push(vec![]);
    map.push(vec![]);
    map[2].push(999);
    println!("map: {:?}", map);
}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}
