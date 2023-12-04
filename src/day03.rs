use crate::{get_string_from_input, Solver};
use std::io::BufRead;

#[derive(Clone, Copy, Debug)]
struct Coord {
    row: i32,
    col: i32,
}
// SOLN:
fn first(input: &mut dyn BufRead) -> String {
    let mut digit_groups: Vec<Vec<Coord>> = vec![];
    let mut symbols: Vec<Coord> = vec![];

    let mut row = 0;
    for line in get_string_from_input(input).lines() {
        // build an "object map" of part numbers and non '.' symbols
        // - Digit Groups: ea group is a collection of coords
        // - Symbol: any character not a numeric or '.'

        let mut group = vec![];

        for (col, c) in line.char_indices() {
            if c.is_numeric() {
                group.push(Coord {
                    row,
                    col: col as i32,
                });
            } else if c == '.' {
                if group.len() > 0 {
                    digit_groups.push(group.clone());
                    group.clear();
                }
            } else {
                if group.len() > 0 {
                    digit_groups.push(group.clone());
                    group.clear();
                }
                symbols.push(Coord {
                    row,
                    col: col as i32,
                });
            }
        }
        row += 1;
    }

    // TODO Identify Part Numbers
    // - if any digit in a group is adjacent to a symbol, group is a part number
    // - assume part numbers cannot connect vertically, IOW they are only defined horizontally
    let part_numbers: Vec<Vec<Coord>> = vec![];

    // be smart about checks, use a series of if's (not else if's)
    // - first digit in a group checks column adjacent to left
    // - every digit checks above and below
    // - last digit in a group checks column adjacent to right

    "".to_string()
}

// SOLN:
fn second(input: &mut dyn BufRead) -> String {
    for line in get_string_from_input(input).lines() {}

    "".to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
