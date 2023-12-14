#![allow(unused)]
use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// SOLN: 23750
// 47536 high
fn count_winners(line: &str) -> usize {
    let culled: Vec<&str> = line.split_once(':').unwrap().1.split('|').collect();
    let winners: Vec<u32> = culled[0]
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    culled[1]
        .trim()
        .split_whitespace()
        .filter_map(|x| {
            let given = x.parse::<u32>().unwrap();
            if winners.contains(&given) {
                Some(given)
            } else {
                None
            }
        })
        .count()
}

fn first(input: &mut dyn BufRead) -> String {
    let mut sum = 0;

    for line in get_string_from_input(input).lines() {
        let count: usize = count_winners(line);

        if count > 0 {
            sum += 2u32.pow((count as u32 - 1));
        }

        // skip "Card N: "
        // read winning numbers
        // read given numbers, during which compare to winners and add to `score_counter`
        // calc card score
        // sum card scores
    }

    sum.to_string()
}

// SOLN: 13_261_850
// 1169 low
fn second(input: &mut dyn BufRead) -> String {
    let mut scratchcard_count = 0;
    let mut winner_counts = vec![];
    // can either process incrementally or batch.. ?
    // - incrementally: process subsequent cards and use a cache
    // - batch: calc win_count for all cards and store, then process (easier to debug?)

    // calc win_count for all cards, store in Vec. Index + 1 == Card No.
    // recursively process and add original + copies to a Hashmap

    for (i, line) in get_string_from_input(input).lines().enumerate() {
        let count = count_winners(line);
        winner_counts.push(count);
    }

    let mut n_copies: Vec<usize> = vec![1; winner_counts.len()];

    for (i, count) in winner_counts.iter().enumerate() {
        for card_idx in (i + 1)..=(i + count) {
            n_copies[card_idx] += n_copies[i];
        }
    }

    n_copies.iter().sum::<usize>().to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
