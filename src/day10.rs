#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// TODO instead of passing tuples around, make them Locations. Checks map bounds.
// - add, subtract easily
// TODO Pipes method returns opening loc deltas

// Depending on where a pipe is entered determines where you can go next

#[derive(PartialEq, Debug)]
pub enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

#[derive(PartialEq, Debug)]
pub enum Pipe {
    NS,
    WE,
    EN,
    ES,
    WS,
    WN,
}

#[derive(PartialEq)]
pub enum Opening {
    N,
    E,
    S,
    W,
}

impl Opening {
    fn opposite(&self) -> Opening {
        match self {
            Opening::N => Opening::S,
            Opening::E => Opening::W,
            Opening::S => Opening::N,
            Opening::W => Opening::E,
        }
    }

    fn d_exit(&self) -> (isize, isize) {
        match self {
            Opening::N => (0, -1),
            Opening::E => (1, 0),
            Opening::S => (0, 1),
            Opening::W => (-1, 0),
        }
    }

    fn d_enter(&self) -> (isize, isize) {
        match self {
            Opening::N => (0, 1),
            Opening::E => (-1, 0),
            Opening::S => (0, -1),
            Opening::W => (1, 0),
        }
    }
}

impl Pipe {
    fn d_exits(&self) -> Vec<(isize, isize)> {
        let openings = self.openings();
        openings.iter().map(|o| o.d_exit()).collect()
    }

    fn openings(&self) -> [Opening; 2] {
        match self {
            Pipe::NS => [Opening::N, Opening::S],
            Pipe::WE => [Opening::W, Opening::E],
            Pipe::EN => [Opening::E, Opening::N],
            Pipe::ES => [Opening::E, Opening::S],
            Pipe::WN => [Opening::W, Opening::N],
            Pipe::WS => [Opening::W, Opening::S],
        }
    }
    fn can_enter_from(&self, rel_enter_from: (isize, isize)) -> bool {
        // which opening does the enter_from move into for next pipe
        let opening = convert_exit_move_to_matching_entry_opening(rel_enter_from);
        self.openings().contains(&opening)
    }

    fn get_exit_opening(&self, enter_from: Opening) -> Option<Opening> {
        match self {
            Pipe::NS => match enter_from {
                Opening::N => Some(Opening::S),
                Opening::S => Some(Opening::N),
                _ => None,
            },
            Pipe::WE => match enter_from {
                Opening::W => Some(Opening::E),
                Opening::E => Some(Opening::W),
                _ => None,
            },
            Pipe::EN => match enter_from {
                Opening::N => Some(Opening::E),
                Opening::E => Some(Opening::N),
                _ => None,
            },
            Pipe::ES => match enter_from {
                Opening::E => Some(Opening::S),
                Opening::S => Some(Opening::E),
                _ => None,
            },
            Pipe::WS => match enter_from {
                Opening::W => Some(Opening::S),
                Opening::S => Some(Opening::W),
                _ => None,
            },
            Pipe::WN => match enter_from {
                Opening::N => Some(Opening::W),
                Opening::W => Some(Opening::N),
                _ => None,
            },
        }
    }
}

pub fn convert_exit_move_to_matching_entry_opening(rel_move: (isize, isize)) -> Opening {
    match rel_move {
        (0, 1) => Opening::N, // computer coords, pos y is down
        (1, 0) => Opening::W,
        (0, -1) => Opening::S, // computer coords, neg y is up
        (-1, 0) => Opening::E,
        _ => panic!("Invalid entry move: {:?}", rel_move),
    }
}

pub fn validate_entry(rel_move: (isize, isize), pipe: &Pipe) -> bool {
    let entering_from = convert_exit_move_to_matching_entry_opening(rel_move);
    pipe.openings().contains(&entering_from)
}

pub struct Animal {
    steps: usize,
    location: (usize, usize),
}

impl Animal {
    fn new(rel_move: (isize, isize), location: (usize, usize)) -> Self {
        let entered_from = convert_exit_move_to_matching_entry_opening(rel_move);
        Animal { steps: 1, location }
    }
    fn walk_rel(&mut self, rel_move: (isize, isize)) {
        self.steps += 1;
    }
    fn entered_from(toward: Opening) -> Opening {
        match toward {
            Opening::N => Opening::S,
            Opening::E => Opening::W,
            Opening::S => Opening::N,
            Opening::W => Opening::E,
        }
    }
    fn steps(&self) -> usize {
        self.steps
    }
}

pub fn parse_tile_to_char(t: &Tile) -> char {
    match t {
        Tile::Ground => '.',
        Tile::Start => 'S',
        Tile::Pipe(Pipe::NS) => '|',
        Tile::Pipe(Pipe::WE) => '-',
        Tile::Pipe(Pipe::EN) => 'L',
        Tile::Pipe(Pipe::ES) => 'F',
        Tile::Pipe(Pipe::WS) => '7',
        Tile::Pipe(Pipe::WN) => 'J',
        Tile::Pipe(Pipe::EN) => 'L',
        _ => panic!("Illegal tile"),
    }
}

pub fn parse_char_to_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Ground,
        'S' => Tile::Start,
        '|' => Tile::Pipe(Pipe::NS),
        '-' => Tile::Pipe(Pipe::WE),
        'L' => Tile::Pipe(Pipe::EN),
        'F' => Tile::Pipe(Pipe::ES),
        '7' => Tile::Pipe(Pipe::WS),
        'J' => Tile::Pipe(Pipe::WN),
        'L' => Tile::Pipe(Pipe::EN),
        _ => panic!("Illegal tile char: {}", c),
    }
}

pub fn print_map(map: &Vec<Vec<Tile>>) {
    for row in map {
        println!();
        for tile in row {
            print!("{}", parse_tile_to_char(tile));
        }
    }
    println!();
}

// use for checking all adjacent tiles, eg when standing on ground or start
// and ignores non-pipe tiles
pub fn get_valid_next_locs(
    curr: (isize, isize),
    prev: (isize, isize),
    map: &Map,
) -> Vec<(isize, isize)> {
    let mut valid_adjacent_locations = vec![];
    println!("prev{:?}", prev);

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        let x = curr.0 as isize + *dx;
        let y = curr.1 as isize + *dy;
        if x >= 0 && x < map[0].len() as isize && y >= 0 && y < map.len() as isize {
            if (x, y) != (prev.0 as isize, prev.1 as isize) {
                println!("pos{:?}", (x, y));
                match &map[y as usize][x as usize] {
                    Tile::Pipe(pipe) => {
                        if validate_entry((*dx, *dy), pipe) {
                            valid_adjacent_locations.push((x as isize, y as isize));
                            // rel_steps.push((*dx, *dy));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    valid_adjacent_locations
}

type Map = Vec<Vec<Tile>>;

// SOLN: 6,846 (fix initial non-loop step increment; use correct final return val)
// 13,690 high (use much shorter impl, using filtering pipe exits where exit != prev_loc) OH used steps instead of longest_distance value
pub fn first(input: &mut dyn BufRead) -> String {
    // There is no possibility for dead ends because there is no branching
    // Follow loop around and back to start, halve total steps and round up
    let mut map: Map = vec![];
    let mut start = None;
    for (y, line) in get_string_from_input(input).lines().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            let tile = parse_char_to_tile(c);
            if tile == Tile::Start {
                start = Some((x as isize, y as isize));
            }
            map[y].push(tile);
        }
    }

    let start = match start {
        None => panic!("No start tile found"),
        Some(vals) => vals,
    };

    // print_map(&map);

    let mut curr_loc = start;
    println!(
        "start_tile:{:?}",
        &map[curr_loc.1 as usize][curr_loc.0 as usize]
    );
    let mut prev_loc = curr_loc;
    let next_locs = get_valid_next_locs((curr_loc.0, curr_loc.1), prev_loc, &map);
    curr_loc = next_locs[0];
    println!("chose{:?}", curr_loc);
    println!(
        "relmove{:?}",
        (curr_loc.0 - prev_loc.0, curr_loc.1 - prev_loc.1)
    );

    let mut curr_tile = &map[curr_loc.1 as usize][curr_loc.0 as usize];
    println!("{:?} {:?}", curr_loc, curr_tile);
    println!("-----------------------",);
    let mut steps = 1;

    // |||
    // JSL
    // 7L7

    loop {
        if let Tile::Pipe(pipe) = curr_tile {
            let rel_move = *pipe
                .d_exits()
                .iter()
                .find(|&&loc| (curr_loc.0 + loc.0, curr_loc.1 + loc.1) != prev_loc)
                .unwrap();
            prev_loc = curr_loc;
            curr_loc = (rel_move.0 + curr_loc.0, rel_move.1 + curr_loc.1);
        }

        // println!("moveTo{:?} {:?}", curr_loc, curr_tile);
        // println!("-----------------------",);

        // ... handle other Tiles

        curr_tile = &map[curr_loc.1 as usize][curr_loc.0 as usize];

        if *curr_tile == Tile::Start {
            break;
        }

        steps += 1;
    }
    println!("steps: {}", steps);

    let longest_distance = (steps as f64 / 2.).ceil();

    longest_distance.to_string()
}

// SOLN:
fn second(input: &mut dyn BufRead) -> String {
    for line in get_string_from_input(input).lines() {}

    "".to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
