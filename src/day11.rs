#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

pub fn print_universe(x: &Vec<Vec<char>>) {
    for row in x {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

// SOLN: 9509330 oneshot
pub fn first(input: &mut dyn BufRead) -> String {
    let mut universe: Vec<Vec<char>> = vec![];
    const EXPANSION_FACTOR: i32 = 1;

    let mut cols_with_galaxies = vec![];
    let mut rows_for_expansion = vec![]; // new for no-inline
    for (y, line) in get_string_from_input(input).lines().enumerate() {
        let mut row = vec![];

        let mut has_galaxy = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cols_with_galaxies.push(x);
                has_galaxy = true;
            }
            row.push(c);
        }

        if !has_galaxy {
            rows_for_expansion.push(y);
        }

        universe.push(row);
    }

    // println!("before col expand",);
    // print_universe(&universe);
    // println!("------------------------",);

    let mut cols_for_expansion = vec![];
    for i in 0..universe[0].len() {
        if !cols_with_galaxies.contains(&i) {
            cols_for_expansion.push(i);
        }
    }
    // println!("cols to expand {:?}", cols_for_dot_insertion);

    let mut galaxy_locs: Vec<(i32, i32)> = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_locs.push((x as i32, y as i32));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxy_locs.len() - 1 {
        for j in i..galaxy_locs.len() {
            let mut x1 = galaxy_locs[i].0;
            let mut x2 = galaxy_locs[j].0;

            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }

            let n_col_expansions = cols_for_expansion
                .iter()
                .filter(|&&x| x > x1 as usize && x < x2 as usize)
                .count();

            let mut y1 = galaxy_locs[i].1;
            let mut y2 = galaxy_locs[j].1;

            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            let n_row_expansions = rows_for_expansion
                .iter()
                .filter(|&&y| y > y1 as usize && y < y2 as usize)
                .count();

            let dx = (x1 - x2).abs() + (n_col_expansions as i32 * EXPANSION_FACTOR)
                - n_col_expansions as i32;
            let dy = (y1 - y2).abs() + (n_row_expansions as i32 * EXPANSION_FACTOR)
                - n_row_expansions as i32;

            sum += dx + dy;
        }
    }

    sum.to_string()
}

// SOLN: 635832237682 (account for original space being replaced by the expansion)
// 635832873506 high
pub fn second(input: &mut dyn BufRead) -> String {
    let mut universe: Vec<Vec<char>> = vec![];
    const EXPANSION_FACTOR: i32 = 1_000_000;

    let mut cols_with_galaxies = vec![];
    let mut rows_for_expansion = vec![]; // new for no-inline
    for (y, line) in get_string_from_input(input).lines().enumerate() {
        let mut row = vec![];

        let mut has_galaxy = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cols_with_galaxies.push(x);
                has_galaxy = true;
            }
            row.push(c);
        }

        if !has_galaxy {
            rows_for_expansion.push(y);
        }

        universe.push(row);
    }

    let mut cols_for_expansion = vec![];
    for i in 0..universe[0].len() {
        if !cols_with_galaxies.contains(&i) {
            cols_for_expansion.push(i);
        }
    }

    let mut galaxy_locs: Vec<(i32, i32)> = vec![];
    for (y, row) in universe.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_locs.push((x as i32, y as i32));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxy_locs.len() - 1 {
        for j in i..galaxy_locs.len() {
            let mut x1 = galaxy_locs[i].0;
            let mut x2 = galaxy_locs[j].0;

            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }

            let n_col_expansions = cols_for_expansion
                .iter()
                .filter(|&&x| x > x1 as usize && x < x2 as usize)
                .count();

            let mut y1 = galaxy_locs[i].1;
            let mut y2 = galaxy_locs[j].1;

            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }

            let n_row_expansions = rows_for_expansion
                .iter()
                .filter(|&&y| y > y1 as usize && y < y2 as usize)
                .count();

            // last term subtracted because have to account for the expansion replacing the original space
            // that is: if 1 row expands to 3 rows, one must exclude expanded rows from the (x1 - x2).abs() value
            let dx = (x1 - x2).abs() + (n_col_expansions as i32 * EXPANSION_FACTOR)
                - n_col_expansions as i32;
            let dy = (y1 - y2).abs() + (n_row_expansions as i32 * EXPANSION_FACTOR)
                - n_row_expansions as i32;

            sum += dx as u64 + dy as u64;
        }
    }

    sum.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
