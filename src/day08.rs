#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::{collections::HashMap, io::BufRead};

pub enum Move {
    Left,
    Right,
}

pub struct Branches {
    pub left: String,
    pub right: String,
}

pub type Network = HashMap<String, Branches>;

pub struct Instructions {
    pub sequence: Vec<Move>,
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

    pub fn iter(&self) -> impl Iterator<Item = &Move> + '_ {
        self.sequence.iter().cycle()
    }
}

pub fn parse_line(line: &str) -> Result<(String, Branches), String> {
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

pub fn process_input(input: &mut dyn BufRead) -> (Instructions, Network) {
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

struct Ghost {
    position: String,
    current_path: Path,
    paths: Vec<Path>,
}

impl Ghost {
    fn new(birth_position: String) -> Self {
        Ghost {
            position: birth_position.clone(),
            current_path: Path::new(birth_position),
            paths: vec![],
        }
    }

    fn position(&self) -> &str {
        &self.position
    }

    fn walk(&mut self, node: &str) {
        self.current_path.steps += 1;
        self.position = node.to_string();

        if node.ends_with('Z') {
            self.encounter_end_node(&node);
        }
    }

    fn encounter_end_node(&mut self, node: &str) {
        // if same path already saved (ghost re-traversing), increment counter
        // otherwise, store as new path
        self.current_path.end = Some(node.to_string());
        if let Some(existing_path) = self
            .paths
            .iter_mut()
            .find(|p| p.start == self.current_path.start && p.end == self.current_path.end)
        {
            existing_path.count += 1;
        } else {
            self.paths.push(self.current_path.clone());
            self.current_path = Path::new(node.to_string());
        }
    }

    // how far did ghost walk, IOW how many instructions did it follow, IOW how many main loop iterations
    fn total_steps(&self) -> usize {
        let mut steps = 0;
        for path in &self.paths {
            steps += path.total_steps();
        }
        steps
    }

    // because the 2 paths for each ghost generated are essentially the same cycle though stored in 2 different paths
    // just return steps from either path
    fn get_arbitrary_path_steps(&self) -> usize {
        if self.paths.len() > 1 {
            self.paths[1].steps()
        } else {
            self.paths[0].steps()
        }
    }

    // ghost tells main loop when to stop, after 100 paths have been traversed
    fn has_sufficient_data(&self) -> bool {
        if self.paths.iter().map(|p| p.count()).sum::<usize>() >= 100 {
            true
        } else {
            false
        }
    }

    fn print_paths(&self) {
        for path in &self.paths {
            println!(
                "({} => {}) steps:{} count:{}",
                path.start,
                path.end.as_ref().unwrap_or(&"".to_string()),
                path.steps(),
                path.count
            );
        }
    }

    fn print_current_path(&self) {
        println!(
            "current_path | start:{} end:{} steps:{}",
            self.current_path.start,
            self.current_path.end.as_ref().unwrap_or(&"".to_string()),
            self.current_path.steps
        );
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Path {
    start: String,
    end: Option<String>,
    steps: usize,
    count: usize,
}

impl Path {
    fn new(start: String) -> Self {
        Path {
            start,
            end: None,
            steps: 0,
            count: 1,
        }
    }
    fn total_steps(&self) -> usize {
        self.steps * self.count
    }
    fn steps(&self) -> usize {
        self.steps
    }
    fn count(&self) -> usize {
        self.count
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if (b == 0) {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(input: Vec<usize>) -> usize {
    if (input.len() == 0) {
        return 0;
    }

    input.iter().fold(input[0], |acc, curr| {
        ((acc * curr) as f64 / gcd(acc, *curr) as f64) as usize
    })
}

// SOLN: 18024643846273 (after using an lcm function and counting cycle lengths correctly)
// 49153966541198323241619811 high
fn second(input: &mut dyn BufRead) -> String {
    let (instructions, mut network) = process_input(input);

    let mut ghosts: Vec<Ghost> = network
        .iter()
        .filter_map(|(key, _branches)| {
            if key.ends_with("A") {
                Some(Ghost::new(key.to_string()))
            } else {
                None
            }
        })
        .collect();

    for ghost in ghosts.iter_mut() {
        let mut steps = 0;
        let max_steps = 1_000_000_000;
        for m in instructions.iter() {
            let branches = network.get(ghost.position()).unwrap();

            let node = match m {
                Move::Left => branches.left.clone(),
                Move::Right => branches.right.clone(),
            };

            ghost.walk(&node);

            steps += 1;

            // limit iterations, tho ghost should break in this case
            if steps >= max_steps {
                break;
            }

            if ghost.has_sufficient_data() {
                break;
            }
        }
    }

    let lcm = lcm(ghosts
        .iter()
        .map(|g| g.get_arbitrary_path_steps())
        .collect::<Vec<usize>>());
    lcm.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
