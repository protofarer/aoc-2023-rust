use clap::Parser;
use prettytable::row;
use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
    time::Instant,
};

use aoc_2023_rust::{
    day01,
    day02,
    day03,
    // day04,
    // day05,
    // day06,
    Solver,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    suffix: Option<String>,

    #[arg(short, long)]
    tries: Option<u128>,

    #[arg()]
    days: Option<Vec<u8>>,
}

const BENCH_TRIES_DEFAULT: u128 = 500;
const DAY_MAX: u8 = 25;

fn main() {
    // Usage: `cargo run -- <2_digit_day> .. --suffix <suffix> --tries <n>
    let args = Args::parse();

    let mut total_runtime = 0f64;

    let all_days = match args.days {
        None => vec![],
        Some(v) => v,
    };

    let days = if all_days.len() == 0 {
        (1..=DAY_MAX).collect()
    } else {
        all_days
    };

    let tries = if let Some(tries) = args.tries {
        tries
    } else if days.len() == 0 {
        BENCH_TRIES_DEFAULT
    } else {
        1
    };

    let mut table = prettytable::Table::new();
    table.add_row(row!["Day", "Part 1", "elapsed", "Part 2", "elapsed"]);

    for day in days {
        let suffix = if let Some(str) = &args.suffix {
            format!("-{}", str)
        } else {
            String::from("")
        };
        let input_filename = format!("input/{:02}{}.txt", day, suffix);
        let mut input = BufReader::new(match File::open(input_filename) {
            Ok(v) => v,
            Err(_) => {
                continue;
            }
        });
        input.fill_buf().expect("fail to fill buffer");

        let mut row_vec = vec![day.to_string()];
        let solvers = get_day(day);
        for func in solvers {
            let (result, elapsed) = run_solver(*func, &mut input, tries);
            row_vec.push(result.to_string());
            row_vec.push(format!("{:.02} ns", elapsed / 1000000f64));
            total_runtime += elapsed;
        }
        table.add_row(row_vec.into());
    }
    table.printstd();
    println!();
    println!("Total runtime: {:.02} ns", total_runtime / 1000000f64);
}

fn run_solver(solver: Solver, input: &mut (impl BufRead + Seek), tries: u128) -> (String, f64) {
    let start = Instant::now();
    let mut result = None;
    for _ in 0..tries {
        input.seek(std::io::SeekFrom::Start(0)).unwrap();
        match result.as_ref() {
            Some(result) => {
                if *result != solver(input) {
                    panic!("Solver returned different results");
                }
            }
            None => result = Some(solver(input)),
        }
    }
    let elapsed = start.elapsed().as_nanos() as f64 / tries as f64;
    (result.unwrap(), elapsed)
}

fn get_day(day: u8) -> &'static [Solver] {
    match day {
        1 => day01::SOLVERS,
        2 => day02::SOLVERS,
        3 => day03::SOLVERS,
        // 4 => day04::SOLVERS,
        // 5 => day05::SOLVERS,
        // 6 => day06::SOLVERS,
        // 7 => day07::SOLVERS,
        // 8 => day08::SOLVERS,
        // 9 => day09::SOLVERS,
        // 10 => day10::SOLVERS,
        // 11 => day11::SOLVERS,
        // 12 => day12::SOLVERS,
        // 13 => day13::SOLVERS,
        // 14 => day14::SOLVERS,
        // 15 => day15::SOLVERS,
        // 16 => day16::SOLVERS,
        // 17 => day17::SOLVERS,
        // 18 => day18::SOLVERS,
        // 19 => day19::SOLVERS,
        // 20 => day20::SOLVERS,
        // 21 => day21::SOLVERS,
        // 22 => day22::SOLVERS,
        // 23 => day23::SOLVERS,
        // 24 => day24::SOLVERS,
        // 25 => day25::SOLVERS,
        _ => panic!("Day {} not implemented", day),
    }
}
// fn main() {
//     run();
// }
