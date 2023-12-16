#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::{collections::HashMap, io::BufRead};

enum Move {
    Left,
    Right,
}

struct Branches {
    left: String,
    right: String,
}

type Network = HashMap<String, Branches>;

struct Instructions {
    sequence: Vec<Move>,
}

impl Instructions {
    fn new(input: &str) -> Self {
        let sequence: Vec<Move> = input
            .chars()
            .map(|c| match c {
                'R' => Move::Right,
                'L' => Move::Left,
                _ => {
                    panic!("Illegal move character encountered: {}", c);
                }
            })
            .collect();

        Instructions { sequence }
    }

    fn iter(&self) -> impl Iterator<Item = &Move> + '_ {
        self.sequence.iter().cycle()
    }
}

fn parse_line(line: &str) -> Result<(String, Branches), String> {
    let cleaned: String = line
        .chars()
        .filter(|&c| !c.is_whitespace() && c != '(' && c != ')')
        .collect();

    let (key, branches) = cleaned
        .split_once('=')
        .ok_or_else(|| "Invalid format: '=' not found".to_string())?;

    let (left, right) = branches
        .split_once(',')
        .ok_or_else(|| "Invalid format: ',' not found".to_string())?;

    Ok((
        key.to_string(),
        Branches {
            left: left.to_string(),
            right: right.to_string(),
        },
    ))
}

// SOLN: 18,727 (oneshot)
fn first(input: &mut dyn BufRead) -> String {
    let input_str = get_string_from_input(input);
    let mut lines = input_str.lines();

    // Read in instructions
    let instruction_str = lines.next().unwrap();
    let instructions = Instructions::new(instruction_str);

    // Read in nodes
    let mut network: Network = HashMap::new();

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let (key, branches) = match parse_line(line) {
            Ok((key, branches)) => (key, branches),
            Err(e) => panic!("{}", e),
        };

        network.insert(key.to_string(), branches);
    }

    // for (key, value) in &network {
    //     println!("{} = ({}, {})", key, value.left, value.right);
    // }

    // Navigate by starting at `AAA` and ending upon arrival at `ZZZ`
    // let mut key = "AAA";
    // let mut steps = 0;
    // for m in instructions.iter() {
    //     let branches = network.get(key).unwrap();

    //     key = match m {
    //         Move::Left => &branches.left,
    //         Move::Right => &branches.right,
    //     };

    //     // println!("arrived at: {}", key);

    //     steps += 1;

    //     if key == "ZZZ" {
    //         break;
    //     }
    // }

    // steps.to_string()
    "".to_string()
}

fn process_input(input: &mut dyn BufRead) -> (Instructions, Network) {
    let input_str = get_string_from_input(input);
    let mut lines = input_str.lines();

    let instruction_str = lines.next().unwrap();
    let instructions = Instructions::new(instruction_str);

    let mut network: Network = HashMap::new();

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let (key, branches) = match parse_line(line) {
            Ok((key, branches)) => (key, branches),
            Err(e) => panic!("{}", e),
        };

        network.insert(key.to_string(), branches);
    }
    (instructions, network)
}

// SOLN:
fn second(input: &mut dyn BufRead) -> String {
    let (instructions, mut network) = process_input(input);

    let mut positions: Vec<String> = network
        .iter()
        .filter_map(|(key, _branches)| {
            if key.ends_with("A") {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect();

    let mut ends: Vec<Vec<String>> = vec![vec![]; positions.len()];
    // analyze input, for each path
    // 1. how many steps to first Z
    // 2. how many steps from first Z to next Z
    // 3. for every Z encountered, how many steps did it take?

    let mut steps = 0;
    let max_steps = 1_000_000;
    for m in instructions.iter() {
        positions = positions
            .iter()
            .enumerate()
            .map(|(i, p)| {
                // iterate through positions and map to a new position based on branch and instruction m
                let branches = network.get(p).unwrap();

                let new_position = match m {
                    Move::Left => branches.left.clone(),
                    Move::Right => branches.right.clone(),
                };

                if new_position.ends_with("Z") {
                    ends[i].push(new_position.clone());
                }
                new_position
            })
            .collect();

        steps += 1;

        if steps >= max_steps {
            break;
        }

        if positions.iter().all(|p| p.ends_with("Z")) {
            break;
        }
    }
    println!("ends: {:?}", ends);

    steps.to_string()
    // "".to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
