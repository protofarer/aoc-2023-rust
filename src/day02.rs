#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// SOLN: 2720
// 16520 too high
// 21498 too high
// 13245 too high

const BAG_MAXES: (u8, u8, u8) = (12, 13, 14); // r g b

fn first(input: &mut dyn BufRead) -> String {
    let mut sum: u32 = 0;

    for line in get_string_from_input(input).lines() {
        // consume first 2 "words" "Game ID:" and store ID in var
        let id = read_id(line);
        println!("=========== Game {} ============", id);

        let mut game_maxes: (u32, u32, u32) = (0, 0, 0);

        // a subset is constituted of the pattern: `NUMBER [color], ... ;`
        // - `;` and EOL delimit the subsets
        // - `, ` delimit color quantities
        // - colors in subset can be in any order, must read the [color]

        // read each subset ?incrementally (faster) rather than all subsets at once
        let subsets = read_subsets(line);

        for subset in subsets {
            // println!("subset: {}", subset);
            for color_entry in subset.trim().split(',') {
                let values: Vec<&str> = color_entry.trim().split_whitespace().collect();
                // println!("val1: {}  val2: {}", values[0], values[1]);
                let count: u32 = values[0].parse().unwrap();

                // ? CSDR instead break out of game block in any arm below when a bad max is read
                match values[1] {
                    "red" => {
                        if count > game_maxes.0 {
                            game_maxes.0 = count;
                        }
                    }
                    "green" => {
                        if count > game_maxes.1 {
                            game_maxes.1 = count;
                        }
                    }
                    "blue" => {
                        if count > game_maxes.2 {
                            game_maxes.2 = count;
                        }
                    }
                    _ => {}
                }
            }
        }

        // if each max in subset <= respective bag_max, sum ID
        if game_maxes.0 <= BAG_MAXES.0 as u32
            && game_maxes.1 <= BAG_MAXES.1 as u32
            && game_maxes.2 <= BAG_MAXES.2 as u32
        {
            sum += id;
        }
    }

    sum.to_string()
}

fn read_id(line: &str) -> u32 {
    line.split_whitespace()
        .nth(1)
        .and_then(|num| num.trim_end_matches(':').parse().ok())
        .unwrap_or(0)
}

fn read_subsets(line: &str) -> std::str::Split<'_, char> {
    line.split_once(':').unwrap().1.split(';')
}

// SOLN: 71535
// 14108 low
fn second(input: &mut dyn BufRead) -> String {
    // for each game's (even invalid games) color maxes, calculate a "power" red_max * green_max * blue_max
    // sum the "powers"
    let mut sum_powers: u32 = 0;

    for line in get_string_from_input(input).lines() {
        let id = read_id(line);
        println!("=========== Game {} ============", id);

        let mut game_maxes: (u32, u32, u32) = (0, 0, 0);

        // a subset is constituted of the pattern: `NUMBER [color], ... ;`
        // - `;` and EOL delimit the subsets
        // - `, ` delimit color quantities
        // - colors in subset can be in any order, must read the [color]

        // read each subset ?incrementally (faster) rather than all subsets at once
        let subsets = read_subsets(line);

        for subset in subsets {
            for color_entry in subset.trim().split(',') {
                let values: Vec<&str> = color_entry.trim().split_whitespace().collect();
                let count: u32 = values[0].parse().unwrap();

                // ? CSDR instead break out of game block in any arm below when a bad max is read
                match values[1] {
                    "red" => {
                        if count > game_maxes.0 {
                            game_maxes.0 = count;
                        }
                    }
                    "green" => {
                        if count > game_maxes.1 {
                            game_maxes.1 = count;
                        }
                    }
                    "blue" => {
                        if count > game_maxes.2 {
                            game_maxes.2 = count;
                        }
                    }
                    _ => {}
                }
            }
        }

        sum_powers += game_maxes.0 * game_maxes.1 * game_maxes.2;
    }

    sum_powers.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
