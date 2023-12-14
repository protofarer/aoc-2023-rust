#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// SOLN: 440,000
fn first(input: &mut dyn BufRead) -> String {
    let lines_string = get_string_from_input(input);
    let mut lines = lines_string.lines();
    let first_line = lines.next().unwrap();

    const SPEED_INCREMENT: u32 = 1; // 1 millimeter per millisecond

    let race_durations: Vec<u32> = first_line
        .split_once(':')
        .unwrap_or(("", ""))
        .1
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let second_line = lines.next().unwrap();

    let distance_records: Vec<u32> = second_line
        .split_once(':')
        .unwrap_or(("", ""))
        .1
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let mut ways_per_race = vec![0; race_durations.len()];
    for (i, duration) in race_durations.iter().enumerate() {
        for t_held in 0..*duration {
            let t_move = duration - t_held;
            let speed = t_held * SPEED_INCREMENT;
            let distance = speed * t_move;
            if distance > distance_records[i] {
                ways_per_race[i] += 1;
            }
        }
    }
    let result: u32 = ways_per_race.iter().product();

    result.to_string()
}

// SOLN: 26187338
// 42686982 high (confused distance_record with duration)
fn second(input: &mut dyn BufRead) -> String {
    const SPEED_INCREMENT: u64 = 1; // 1 millimeter per millisecond

    let lines_string = get_string_from_input(input);
    let mut lines = lines_string.lines();
    let first_line = lines.next().unwrap();

    let race_duration = first_line
        .split_once(':')
        .unwrap_or(("", ""))
        .1
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    println!("time: {}", race_duration);

    let second_line = lines.next().unwrap();

    let distance_record = second_line
        .split_once(':')
        .unwrap_or(("", ""))
        .1
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    println!("distance: {}", distance_record);

    let mut ways = 0;
    for t_held in 0..race_duration {
        let t_move = race_duration - t_held;
        let speed = t_held * SPEED_INCREMENT;
        let distance = speed * t_move;
        if distance > distance_record {
            ways += 1;
        }
    }
    // let result: u32 = ways_per_race.iter().product();

    // result.to_string()
    ways.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
