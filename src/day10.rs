// #![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Location(isize, isize);

impl Location {
    fn new(loc: (isize, isize)) -> Self {
        Location(loc.0, loc.1)
    }
    pub fn x(&self) -> isize {
        self.0
    }
    pub fn y(&self) -> isize {
        self.1
    }
}

impl std::ops::Add<Location> for Location {
    type Output = Location;

    fn add(self, other: Location) -> Location {
        Location(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub<Location> for Location {
    type Output = Location;

    fn sub(self, other: Location) -> Location {
        Location(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

#[derive(PartialEq, Debug, Clone, Copy)]
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
    fn d_exit(&self) -> Location {
        match self {
            Opening::N => Location::new((0, -1)),
            Opening::E => Location::new((1, 0)),
            Opening::S => Location::new((0, 1)),
            Opening::W => Location::new((-1, 0)),
        }
    }
}

impl Pipe {
    pub fn get_all_pipes() -> Vec<Pipe> {
        vec![Pipe::NS, Pipe::WE, Pipe::EN, Pipe::ES, Pipe::WN, Pipe::WS]
    }

    pub fn match_pipe_to_rel_locs(rel_locs: &Vec<Location>) -> Option<Pipe> {
        if rel_locs.len() != 2 {
            return None;
        }
        for pipe in Pipe::get_all_pipes() {
            let d_exits = pipe.d_exits();
            if d_exits.contains(&rel_locs[0]) && d_exits.contains(&rel_locs[1]) {
                return Some(pipe);
            }
        }
        None
    }

    pub fn d_exits(&self) -> Vec<Location> {
        let openings = self.openings();
        openings.iter().map(|o| o.d_exit()).collect()
    }

    pub fn openings(&self) -> [Opening; 2] {
        match self {
            Pipe::NS => [Opening::N, Opening::S],
            Pipe::WE => [Opening::W, Opening::E],
            Pipe::EN => [Opening::E, Opening::N],
            Pipe::ES => [Opening::E, Opening::S],
            Pipe::WN => [Opening::W, Opening::N],
            Pipe::WS => [Opening::W, Opening::S],
        }
    }
    pub fn can_enter_from(&self, rel_enter_from: (isize, isize)) -> bool {
        // which opening does the enter_from move into for next pipe
        let opening = convert_exit_move_to_matching_entry_opening(rel_enter_from);
        self.openings().contains(&opening)
    }

    pub fn get_exit_opening(&self, enter_from: Opening) -> Option<Opening> {
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

    pub fn print(&self) {
        println!("{}", self.to_char());
    }

    pub fn to_string(&self) -> String {
        match self {
            Pipe::NS => String::from("|"),
            Pipe::WE => String::from("-"),
            Pipe::EN => String::from("L"),
            Pipe::ES => String::from("F"),
            Pipe::WN => String::from("J"),
            Pipe::WS => String::from("7"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Pipe::NS => '|',
            Pipe::WE => '-',
            Pipe::EN => 'L',
            Pipe::ES => 'F',
            Pipe::WN => 'J',
            Pipe::WS => '7',
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

pub fn parse_tile_to_char(t: &Tile) -> char {
    match t {
        Tile::Ground => '.',
        Tile::Start => 'S',
        Tile::Pipe(pipe) => pipe.to_char(),
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
        _ => panic!("Illegal tile char: {}", c),
    }
}

pub fn print_map(map: &Map) {
    for row in map {
        for tile in row {
            print!("{}", parse_tile_to_char(tile));
        }
        println!();
    }
}

// use for checking all adjacent tiles, eg when standing on ground or start
// and ignores non-pipe tiles
pub fn get_valid_next_locs(curr: Location, prev: Option<Location>, map: &Map) -> Vec<Location> {
    let mut valid_adjacent_locations = vec![];

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        let x = curr.0 as isize + *dx;
        let y = curr.1 as isize + *dy;
        if x >= 0 && x < map[0].len() as isize && y >= 0 && y < map.len() as isize {
            match prev {
                // validate all locs except prev_loc
                Some(prev_loc) => {
                    if (x, y) != (prev_loc.0 as isize, prev_loc.1 as isize) {
                        if let Tile::Pipe(pipe) = &map[y as usize][x as usize] {
                            if validate_entry((*dx, *dy), pipe) {
                                valid_adjacent_locations
                                    .push(Location::new((x as isize, y as isize)));
                                // rel_steps.push((*dx, *dy));
                            }
                        }
                    }
                }
                // validate all surrounding locs
                None => {
                    if let Tile::Pipe(pipe) = &map[y as usize][x as usize] {
                        if validate_entry((*dx, *dy), pipe) {
                            valid_adjacent_locations.push(Location::new((x as isize, y as isize)));
                            // rel_steps.push((*dx, *dy));
                        }
                    }
                }
            }
        }
    }
    valid_adjacent_locations
}

type Map = Vec<Vec<Tile>>;

pub fn create_map(input: &mut dyn BufRead) -> (Map, Location) {
    let mut map: Map = vec![];
    let mut start = None;

    for (y, line) in get_string_from_input(input).lines().enumerate() {
        map.push(vec![]);

        for (x, c) in line.chars().enumerate() {
            let tile = parse_char_to_tile(c);
            if tile == Tile::Start {
                start = Some(Location::new((x as isize, y as isize)));
            }
            map[y].push(tile);
        }
    }

    let start = match start {
        None => panic!("No start tile found"),
        Some(loc) => loc,
    };
    (map, start)
}

pub fn replace_start_tile_with_pipe(
    start: &Location,
    map: &mut Map,
    rel_locs_adj_pipes: &Vec<Location>,
) {
    let start_pipe = Pipe::match_pipe_to_rel_locs(rel_locs_adj_pipes).unwrap();
    map[start.y() as usize][start.x() as usize] = Tile::Pipe(start_pipe);
}

// SOLN: 6,846 (fix initial non-loop step increment; use correct final return val)
// 13,690 high (use much shorter impl, using filtering pipe exits where exit != prev_loc) OH used steps instead of longest_distance value
pub fn first(input: &mut dyn BufRead) -> String {
    // There is no possibility for dead ends because there is no branching
    // Follow loop around and back to start, halve total steps and round up

    let (mut map, start) = create_map(input);

    let mut curr_loc = start;
    let mut prev_loc = curr_loc;
    let next_locs = get_valid_next_locs(curr_loc, None, &map);

    // Replace start tile with appropriate pipe to complete loop
    // - find the pipe that matches the openings that would be situated at next_locs
    let rel_locs_adj_pipes: Vec<Location> = next_locs.iter().map(|&loc| loc - curr_loc).collect();
    replace_start_tile_with_pipe(&start, &mut map, &rel_locs_adj_pipes);

    curr_loc = next_locs[0];

    let mut curr_tile;
    let mut steps = 1;
    loop {
        if curr_loc == start {
            break;
        }

        curr_tile = &map[curr_loc.1 as usize][curr_loc.0 as usize];

        if let Tile::Pipe(pipe) = curr_tile {
            let exits = pipe.d_exits();
            let rel_move = exits
                .iter()
                .find(|&&loc| curr_loc + loc != prev_loc)
                .unwrap();
            prev_loc = curr_loc;

            curr_loc = *rel_move + curr_loc;
        }

        steps += 1;
    }

    let longest_distance = (steps as f64 / 2.).ceil();
    longest_distance.to_string()
}

struct Collector {
    n_locs: i32,
    n_intersects: i32,
}

impl Collector {
    fn new() -> Self {
        Collector {
            n_locs: 0,
            n_intersects: 0,
        }
    }
}

pub fn write_map_to_file(map: &Vec<Vec<Tile>>, file_path: &str) -> std::io::Result<()> {
    use std::io::Write;

    let mut file = std::fs::File::create(file_path)?;

    for row in map {
        for tile in row {
            write!(file, "{}", parse_tile_to_char(tile))?;
        }
        writeln!(file)?;
    }

    Ok(())
}

// SOLN: 325 (replaced S with the fitting pipe!!!)
// 6584 high
// 12,317 high (after doing the collector and only collect upon intersection approach)
// 3,975 high (after fixing bug where n_locs not being cleared for odd intersection)
// 5,562 wrong (after overhaul to raycasting, using collectors and matching pipe entry/exits)
// 2,727 wrong (fix incorrectly overcounting n_intersect == 0 groups)
// 346 wrong (fix was counting even intersect groups (outside) instead of odds (inside))
fn second(input: &mut dyn BufRead) -> String {
    // use raycasting algo:
    // 1. odd number of intersections from the point of analysis to edge of grid indicates inside loop
    // 2. even number.. indicates outside of loop
    // A: continuous intersections aka ray is parallel and intersecting a portion of the loop counts as 1 intersection
    // B: loop "corners" are equivalent to the conditions described in A

    let (mut map, start) = create_map(input);

    // Create the looping path
    let mut path: Vec<Location> = vec![];

    let mut curr_loc = start;
    path.push(curr_loc);

    let mut prev_loc = curr_loc;

    let next_locs = get_valid_next_locs(curr_loc, None, &map);

    let rel_locs_adj_pipes: Vec<Location> = next_locs.iter().map(|&loc| loc - curr_loc).collect();
    replace_start_tile_with_pipe(&start, &mut map, &rel_locs_adj_pipes);

    // write_map_to_file(&map, "part10replacedstart.txt");

    curr_loc = next_locs[0];
    path.push(curr_loc);

    let mut curr_tile;

    loop {
        curr_tile = &map[curr_loc.1 as usize][curr_loc.0 as usize];

        if curr_loc == start {
            break;
        }

        if let Tile::Pipe(pipe) = curr_tile {
            let exits = pipe.d_exits();
            let rel_move = exits
                .iter()
                .find(|&&loc| curr_loc + loc != prev_loc)
                .unwrap();
            prev_loc = curr_loc;
            curr_loc = *rel_move + curr_loc;
            path.push(curr_loc);
        }
    }

    // do horizontal ray casts (easier to traverse within the vec of vecs map)
    let mut n_locs_inside: i32 = 0;

    for y in 0..map.len() {
        let mut entry_pipe: Option<Pipe> = None;
        let mut collectors: Vec<Collector> = vec![Collector::new()];

        for x in 0..map[0].len() {
            // encountered path, determine kind of intersection
            if path.contains(&Location::new((x as isize, y as isize))) {
                let tile = &map[y][x];

                match &tile {
                    &Tile::Pipe(pipe_variant) => {
                        match pipe_variant {
                            // entry, piercing intersection => store locs as group
                            Pipe::NS => {
                                // increment all n_intersects
                                // start new group
                                collectors.iter_mut().for_each(|c| c.n_intersects += 1);
                                collectors.push(Collector::new());
                            }

                            // entry, parallel => save entry type (up/down), await exit
                            // save entry per pipe type,
                            // EN == entry from N
                            // ES == entry from S
                            Pipe::EN | Pipe::ES => {
                                entry_pipe = Some(*pipe_variant);
                            }

                            // exit, piercing => exit opp entry, store locs
                            // EN == exit from N, piercing if entry was WS
                            // ES == exit from S, piercing if entry was WN
                            Pipe::WN | Pipe::WS => {
                                // test piercing and collect as appropo
                                match entry_pipe {
                                    Some(Pipe::EN) => {
                                        if *pipe_variant == Pipe::WS {
                                            collectors.iter_mut().for_each(|c| c.n_intersects += 1);
                                            collectors.push(Collector::new());
                                        }
                                    }
                                    Some(Pipe::ES) => {
                                        if *pipe_variant == Pipe::WN {
                                            collectors.iter_mut().for_each(|c| c.n_intersects += 1);
                                            collectors.push(Collector::new());
                                        }
                                    }
                                    _ => {}
                                }
                                entry_pipe = None;
                            }

                            Pipe::WE => {}
                        }
                    }
                    _ => { /* path only contains pipes */ }
                }
            } else {
                // get last item in collectors and increase n_locs
                if let Some(last_collector) = collectors.last_mut() {
                    last_collector.n_locs += 1;
                }
            }
        }
        // process collected locs
        n_locs_inside += collectors
            .iter()
            .filter_map(|c| {
                if c.n_intersects > 0 && c.n_intersects % 2 == 1 {
                    Some(c.n_locs)
                } else {
                    None
                }
            })
            .sum::<i32>();
    }

    n_locs_inside.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
