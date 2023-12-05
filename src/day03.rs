use crate::{get_string_from_input, Solver};
use std::{collections::HashSet, io::BufRead};

#[derive(Clone, Copy, Debug)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Clone, Debug)]
struct Group {
    coords: Vec<Coord>,
    digits: String,
}

impl Group {
    fn new() -> Self {
        Group {
            coords: vec![],
            digits: "".to_string(),
        }
    }
    fn add_number(&mut self, c: char, row: usize, col: usize) {
        self.coords.push(Coord {
            row: row as i32,
            col: col as i32,
        });
        self.digits.push_str(&c.to_string());
    }
    fn len(&self) -> usize {
        self.coords.len()
    }
}

// SOLN: 522726
// 377123 low
// 518219 low (after fixing check right adjacents)
fn first(input: &mut dyn BufRead) -> String {
    let mut digit_groups: Vec<Group> = vec![];
    let mut symbols_coords: Vec<Coord> = vec![];
    let mut unique_symbols = HashSet::new();

    let mut row = 0;
    for line in get_string_from_input(input).lines()
    // .skip(137).take(3)
    {
        // build an "object map" of part numbers and non '.' symbols
        // - Digit Groups: ea group is a collection of coords
        // - Symbol: any character not a numeric or '.'

        let mut group = Group::new();
        for (col, c) in line.char_indices() {
            if c.is_numeric() {
                group.add_number(c, row, col);
            } else if c == '.' {
                if group.len() > 0 {
                    digit_groups.push(group.clone());
                    group = Group::new();
                }
            } else {
                if group.len() > 0 {
                    digit_groups.push(group.clone());
                    group = Group::new();
                }
                unique_symbols.insert(c);
                symbols_coords.push(Coord {
                    row: row as i32,
                    col: col as i32,
                });
            }
        }
        if group.len() > 0 {
            digit_groups.push(group.clone());
        }
        row += 1;
    }
    println!("unique symbols: {:?}", unique_symbols);

    // TODO Identify Part Numbers
    // - if any digit in a group is adjacent to a symbol, group is a part number
    // - assume part numbers cannot connect vertically, IOW they are only defined horizontally
    let mut part_numbers: Vec<Group> = vec![];

    for group in digit_groups {
        let length = group.len();
        for (i, coord) in group.coords.iter().enumerate() {
            // * as soon as adjacent symbol detected, tag as part number and skip to next group

            let (row, col) = (coord.row, coord.col);

            // be smart about checks, use a series of if's (not else if's)
            // - first digit in a group checks column adjacent to left
            // - every digit checks above and below
            // - last digit in a group checks column adjacent to right

            // always check vertical
            // possible adjacent symbols: directly up and down
            let is_any_symbol_above_or_below = symbols_coords.iter().find(|symbol_coord| {
                (symbol_coord.row == row - 1 || symbol_coord.row == row + 1)
                    && symbol_coord.col == col
            });
            match is_any_symbol_above_or_below {
                Some(_) => {
                    part_numbers.push(group.clone());
                    break;
                }
                None => {}
            }

            // check left adjacent
            if i == 0 {
                // possible adjacent symbols: up left, left, down left
                let is_any_symbol_left_adjacent = symbols_coords.iter().find(|symbol_coord| {
                    (symbol_coord.row == row - 1
                        || symbol_coord.row == row + 1
                        || symbol_coord.row == row)
                        && symbol_coord.col == col - 1
                });
                match is_any_symbol_left_adjacent {
                    Some(_) => {
                        part_numbers.push(group.clone());
                        break;
                    }
                    None => {}
                }
            }

            // check right adjacent
            if i == length - 1 {
                // possible adjacent symbols: up right, right, down right
                let is_any_symbol_right_adjacent = symbols_coords.iter().find(|symbol_coord| {
                    (symbol_coord.row == row - 1
                        || symbol_coord.row == row + 1
                        || symbol_coord.row == row)
                        && symbol_coord.col == col + 1
                });
                match is_any_symbol_right_adjacent {
                    Some(_) => {
                        part_numbers.push(group.clone());
                        break;
                    }
                    None => {}
                }
            }
        }
    }
    // println!("========= part numbers found: =========",);
    // for x in &part_numbers {
    //     println!("{:?}", x.digits);
    // }

    let sum: u32 = part_numbers
        .iter()
        .map(|x| x.digits.parse::<u32>().unwrap())
        .sum();

    sum.to_string()
}

// SOLN: 81721933
// 3607 low
fn second(input: &mut dyn BufRead) -> String {
    // for each '*' with exactly 2 adjacent digit groups, multiply the digit group values with each other
    // sum the mults

    let mut digit_groups: Vec<Group> = vec![];
    let mut star_coords: Vec<Coord> = vec![];

    let mut row = 0;
    for line in get_string_from_input(input).lines() {
        let mut group = Group::new();
        for (col, c) in line.char_indices() {
            if c.is_numeric() {
                group.add_number(c, row, col);
            } else {
                if group.len() > 0 {
                    digit_groups.push(group.clone());
                    group = Group::new();
                }
                if c == '*' {
                    star_coords.push(Coord {
                        row: row as i32,
                        col: col as i32,
                    });
                }
            }
        }
        if group.len() > 0 {
            digit_groups.push(group.clone());
        }
        row += 1;
    }

    let mut sum = 0;
    'stars: for star in star_coords {
        let (star_row, star_col) = (star.row, star.col);

        let mut count = 0;
        let mut digits = vec![];

        for d_row in -1..=1 {
            for d_col in -1..=1 {
                if d_row == 0 && d_col == 0 {
                    continue;
                }

                // as soon as a digit from a part number group is found adjacent, add to digits and remove from digit_groups so it won't be iterated over again (counted twice)
                if let Some(idx) = digit_groups.iter().position(|group| {
                    group
                        .coords
                        .iter()
                        .any(|coord| coord.row == star_row + d_row && coord.col == star_col + d_col)
                }) {
                    let group = digit_groups.remove(idx);
                    digits.push(group.digits.to_owned());
                    count += 1;
                }

                if count > 2 {
                    continue 'stars;
                }
            }
        }
        if count == 2 {
            sum += digits[0].parse::<u32>().unwrap() * digits[1].parse::<u32>().unwrap();
        }
    }

    sum.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
