use crate::{get_string_from_input, Solver};
use std::io::BufRead;

// SOLN: 55172
fn first(input: &mut dyn BufRead) -> String {
    let mut sum: u32 = 0;

    for line in get_string_from_input(input).lines() {
        let a = line.chars().find_map(|c| c.to_digit(10)).unwrap();

        let mut chars = line.chars().collect::<Vec<char>>();
        chars.reverse();
        let b = chars.into_iter().find_map(|c| c.to_digit(10)).unwrap();

        let concat = format!("{}{}", a, b).parse::<u8>().unwrap();

        sum += concat as u32;
    }

    sum.to_string()
}

const NUMBER_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const REV_NUMBER_WORDS: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn find_first_number_word(line: &str, rev: bool) -> Option<(usize, String)> {
    if rev {
        REV_NUMBER_WORDS
            .iter()
            .filter_map(|&word| line.find(word).map(|idx| (idx, word)))
            .min_by_key(|&(idx, _)| idx)
            .map(|(idx, word)| (idx, word.to_string()))
    } else {
        NUMBER_WORDS
            .iter()
            .filter_map(|&word| line.find(word).map(|idx| (idx, word)))
            .min_by_key(|&(idx, _)| idx)
            .map(|(idx, word)| (idx, word.to_string()))
    }
}

fn find_first_numeric_digit(line: &str) -> Option<(usize, usize)> {
    (0..=9)
        .filter_map(|number| line.find(&number.to_string()).map(|idx| (idx, number)))
        .min_by_key(|&(idx, _)| idx)
}

// what about lines with only 1 readable number? :: read as both first and second digit
// SOLN: 55413
fn second(input: &mut dyn BufRead) -> String {
    let mut sum: u32 = 0;

    for line in get_string_from_input(input).lines() {
        // println!("line {}", i);
        // let (idx1, word) = find_first_number_word(line, false);
        // let (idx2, digit) = find_first_numeric_digit(line);
        let result_word = find_first_number_word(line, false);
        let result_number = find_first_numeric_digit(line);

        let first_digit = match (result_word, result_number) {
            (None, None) => {
                println!("No calibration data in line: {}", line);
                return "ERROR".to_string();
            }
            (Some(word_result_tuple), None) => NUMBER_WORDS
                .iter()
                .enumerate()
                .find_map(|(idx, number_word)| {
                    if number_word.eq_ignore_ascii_case(&word_result_tuple.1) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .unwrap(),
            (None, Some(number_result_tuple)) => number_result_tuple.1,
            (Some(word_result_tuple), Some(number_result_tuple)) => {
                let idx_word = word_result_tuple.0;
                let idx_number = number_result_tuple.0;
                if idx_word < idx_number {
                    NUMBER_WORDS
                        .iter()
                        .enumerate()
                        .find_map(|(idx, number_word)| {
                            if number_word.eq_ignore_ascii_case(&word_result_tuple.1) {
                                Some(idx) // the array index corresponds to digit value
                            } else {
                                None
                            }
                        })
                        .unwrap()
                } else {
                    number_result_tuple.1
                }
            }
        };

        // println!("first dig: {}", first_digit);

        let reverse_line_iter = line.chars().rev().collect::<String>();

        // let (idx1, word) = find_first_number_word(&reverse_line_iter, true);
        // let (idx2, digit) = find_first_numeric_digit(&reverse_line_iter);
        let result_word = find_first_number_word(&reverse_line_iter, true);
        let result_number = find_first_numeric_digit(&reverse_line_iter);

        let second_digit = match (result_word, result_number) {
            (None, None) => {
                println!("No calibration data in line: {}", line);
                return "ERROR".to_string();
            }
            (Some(word_result_tuple), None) => REV_NUMBER_WORDS
                .iter()
                .enumerate()
                .find_map(|(idx, rev_number_word)| {
                    // word result tuple has
                    if rev_number_word.eq_ignore_ascii_case(&word_result_tuple.1) {
                        Some(idx)
                    } else {
                        // println!(
                        //     "rev_n_word:{} , word_result_tuple.1:{}",
                        //     rev_number_word, word_result_tuple.1
                        // );
                        None
                    }
                })
                .unwrap(),

            (None, Some(number_result_tuple)) => number_result_tuple.1,

            (Some(word_result_tuple), Some(number_result_tuple)) => {
                let idx_word = word_result_tuple.0;
                let idx_number = number_result_tuple.0;
                if idx_word < idx_number {
                    REV_NUMBER_WORDS
                        .iter()
                        .enumerate()
                        .find_map(|(idx, number_word)| {
                            if number_word.eq_ignore_ascii_case(&word_result_tuple.1) {
                                Some(idx)
                            } else {
                                // println!(
                                //     "n_word: {} , word_result_tuple.1: {}",
                                //     number_word, word_result_tuple.1
                                // );
                                None
                            }
                        })
                        .unwrap()
                } else {
                    number_result_tuple.1
                }
            }
        };

        // println!("second dig: {}", second_digit);

        let concat = format!("{}{}", first_digit, second_digit)
            .parse::<u8>()
            .unwrap();

        sum += concat as u32;
    }

    sum.to_string()
}

pub const SOLVERS: &[Solver] = &[first, second];
