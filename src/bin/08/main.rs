#![allow(unused)]
use aoc_2023_rust::day08::*;
use aoc_2023_rust::special::parse_args_special;
use std::io::BufRead;

fn run_with_input(input: &mut dyn BufRead) {}

fn run_without_input() {
    println!("run 08 w/o input",);
}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}

// fn second(input: &mut dyn BufRead) -> String {
//     let (instructions, mut network) = process_input(input);

//     let mut positions: Vec<String> = network
//         .iter()
//         .filter_map(|(key, _branches)| {
//             if key.ends_with("A") {
//                 Some(key.clone())
//             } else {
//                 None
//             }
//         })
//         .collect();

//     let mut ends: Vec<Vec<String>> = vec![vec![]; positions.len()];
//     // analyze input, for each path
//     // 1. how many steps to first Z
//     // 2. how many steps from first Z to next Z
//     // 3. for every Z encountered, how many steps did it take?

//     let mut steps = 0;
//     let max_steps = 1_000_000;
//     for m in instructions.iter() {
//         positions = positions
//             .iter()
//             .enumerate()
//             .map(|(i, p)| {
//                 // iterate through positions and map to a new position based on branch and instruction m
//                 let branches = network.get(p).unwrap();

//                 let new_position = match m {
//                     Move::Left => branches.left.clone(),
//                     Move::Right => branches.right.clone(),
//                 };

//                 if new_position.ends_with("Z") {
//                     ends[i].push(new_position.clone());
//                 }
//                 new_position
//             })
//             .collect();

//         steps += 1;

//         if steps >= max_steps {
//             break;
//         }

//         if positions.iter().all(|p| p.ends_with("Z")) {
//             break;
//         }
//     }
//     println!("ends: {:?}", ends);

//     steps.to_string()
//     // "".to_string()

// }
