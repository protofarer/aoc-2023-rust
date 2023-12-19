#![allow(unused)]
use aoc_2023_rust::special::parse_args_special;
use aoc_2023_rust::{day10::*, get_string_from_input};
use std::io::BufRead;

fn run_with_input(input: &mut dyn BufRead) {
    // There is no possibility for dead ends because there is no branching
    // Follow loop around and back to start, halve total steps and round up
    let (mut map, start) = create_map(input);

    let mut curr_loc = start;
    // println!(
    //     "start_tile:{:?}",
    //     &map[curr_loc.1 as usize][curr_loc.0 as usize]
    // );
    let mut prev_loc = curr_loc;
    let next_locs = get_valid_next_locs(curr_loc, None, &map);

    // Replace start tile with appropriate pipe to complete loop
    // - find the pipe that matches the openings that would be situated at next_locs
    let next_rel_locs: Vec<Location> = next_locs.iter().map(|&loc| loc - curr_loc).collect();
    let start_pipe = Pipe::match_pipe_to_rel_locs(&next_rel_locs).unwrap();
    print_map(&map);
    map[start.y() as usize][start.x() as usize] = Tile::Pipe(start_pipe);
    print_map(&map);
}

fn run_without_input() {
    let mut map: Vec<Vec<usize>> = vec![vec![]];
    map.push(vec![]);
    map[0].push(1);
    map.push(vec![]);
    map.push(vec![]);
    map[2].push(999);
    println!("map: {:?}", map);
}

fn main() {
    parse_args_special(run_without_input, run_with_input);
}
