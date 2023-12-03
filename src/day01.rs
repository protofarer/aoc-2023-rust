use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// 55172
fn first(input: &mut dyn BufRead) -> String {
    let mut sum: u32 = 0;

    for line in get_string_from_input(input).lines() {
        let a = line.chars().find_map(|c| c.to_digit(10)).unwrap();

        let mut chars = line.chars().collect::<Vec<char>>();
        chars.reverse();
        let b = chars.into_iter().find_map(|c| c.to_digit(10)).unwrap();

        let concat = format!("{}{}", a, b).parse::<u8>().unwrap();

        sum += concat as u32;
    }

    sum.to_string()
}

pub const SOLVERS: &[Solver] = &[first];
