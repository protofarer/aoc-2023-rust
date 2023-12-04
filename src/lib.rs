use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

pub fn _read_lines_from_file_buffered<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
    }
    Ok(lines)
}

pub fn _read_file_as_string(file_path: &str) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(file_path)?)
}

pub fn get_string_from_input(input: &mut dyn BufRead) -> String {
    let mut data = vec![];
    input.read_to_end(&mut data).unwrap();
    String::from_utf8(data).expect("Data isnt UTF8")
}

pub mod day01;
pub mod day02;
pub mod day03;
// pub mod day04;
// pub mod day05;
// pub mod day06;
// pub mod day07;
// pub mod day08;
// pub mod day09;

pub type Solver = fn(&mut dyn BufRead) -> String;
