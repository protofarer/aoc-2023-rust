use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// SOLN:
fn first(input: &mut dyn BufRead) -> String {
    for line in get_string_from_input(input).lines() {}

    "".to_string()
}

// SOLN:
fn second(input: &mut dyn BufRead) -> String {
    for line in get_string_from_input(input).lines() {}

    "".to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
