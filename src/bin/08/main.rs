#![allow(unused)]
use aoc_2023_rust::day08::*;
use aoc_2023_rust::special::parse_args_special;
use std::io::BufRead;

// Ghost
// each ghost has many paths from some starting point to an end point Z
// each ghost has a birth point (first start point)
// each path is: start string, end string, steps, count (how many of this path does ghost traverse)

// since all ghosts follow same instructions, Ghosts shouldn't know about it...

struct Ghost {
    birth_position: String,
    position: String,
    current_path: Path,
    paths: Vec<Path>,
}

impl Ghost {
    fn new(birth_position: String) -> Self {
        Ghost {
            birth_position: birth_position.clone(),
            position: birth_position.clone(),
            current_path: Path::new(birth_position),
            paths: vec![],
        }
    }

    fn position(&self) -> &str {
        &self.position
    }

    fn walk(&mut self, node: &str) {
        // add step to current path
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

    fn total_steps(&self) -> usize {
        // how far did ghost walk, IOW how many instructions did it follow, IOW how many main loop iterations
        let mut steps = 0;
        for path in &self.paths {
            steps += path.total_steps();
        }
        steps
    }

    fn get_arbitrary_path_steps(&self) -> usize {
        if self.paths.len() > 1 {
            self.paths[1].steps()
        } else {
            self.paths[0].steps()
        }
    }

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

fn run_with_input(input: &mut dyn BufRead) {
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
    // ghosts.sort();

    // print network, look for key nodes ending in Z
    // for (k, v) in network.iter() {
    //     if k.ends_with('Z') {
    //         println!("key ends Z: {} => ({} , {})", k, v.left, v.right);
    //     }
    //     if v.left.ends_with('Z') {
    //         println!("left_branch ends Z: {} => ({} , {})", k, v.left, v.right);
    //     }
    //     if v.right.ends_with('Z') {
    //         println!("right_branch ends Z: {} => ({} , {})", k, v.left, v.right);
    //     }
    // }

    // analyze input, for each path
    // 1. how many steps to first Z
    // 2. how many steps from first Z to next Z
    // 3. for every Z encountered, how many steps did it take?

    println!("----------------- FOLLOWING INSTRUCTS -----------------",);
    for ghost in ghosts.iter_mut() {
        // println!("ghost start: {}", ghost.position());

        let mut steps = 0;
        let max_steps = 1_000_000_000;
        for m in instructions.iter() {
            // Guide a single ghost
            // let next_node = node_from_instruct(ghost1.position());
            let branches = network.get(ghost.position()).unwrap();

            let node = match m {
                Move::Left => branches.left.clone(),
                Move::Right => branches.right.clone(),
            };

            ghost.walk(&node);

            // positions = positions
            //     .iter()
            //     .enumerate()
            //     .map(|(i, p)| {
            //         // iterate through positions and map to a new position based on branch and instruction m
            //         let branches = network.get(p).unwrap();

            //         let new_position = match m {
            //             Move::Left => branches.left.clone(),
            //             Move::Right => branches.right.clone(),
            //         };

            //         if new_position.ends_with("Z") {
            //             ends[i].push(new_position.clone());
            //         }
            //         new_position
            //     })
            //     .collect();

            steps += 1;

            // let mod_num = (max_steps as f64 * 0.1) as usize;
            // if steps % mod_num == 0 {
            //     println!("{} * {} steps taken", steps / mod_num, mod_num);
            // }

            if steps >= max_steps {
                break;
            }

            if ghost.has_sufficient_data() {
                break;
            }

            // if positions.iter().all(|p| p.ends_with("Z")) {
            //     break;
            // }
        }
        ghost.print_paths();
        ghost.print_current_path();
        println!("------------------------------",);
    }

    for ghost in &ghosts {
        println!("path steps ea ghost: {}", ghost.get_arbitrary_path_steps());
    }
    // let lcm: u128 = ghosts
    //     .iter()
    //     .map(|g| g.get_arbitrary_path_steps() as u128)
    //     .product();
    // println!("lcm: {}", lcm);
    let lcm = lcm(ghosts
        .iter()
        .map(|g| g.get_arbitrary_path_steps())
        .collect::<Vec<usize>>());
    println!("lcm: {}", lcm);
    // 18024643846273
}

fn run_without_input() {
    println!("run 08 w/o input",);
}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}
