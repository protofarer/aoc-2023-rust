use aoc_2023_rust::special::parse_args_special;

fn process_file(input_str: String) {
    println!("processing file contents: {}", input_str);
}

fn run() {
    println!("RUN",);
}

fn main() {
    parse_args_special(run, process_file);
}
