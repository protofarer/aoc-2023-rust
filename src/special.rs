use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    input: Option<String>,
}

pub fn parse_args_special<Fa, Fb>(fn_run: Fa, fn_special: Fb)
where
    Fa: Fn(),
    Fb: Fn(&mut dyn BufRead),
{
    let args = Args::parse();

    match args.input {
        Some(input_filename) => {
            let mut input = BufReader::new(match File::open(input_filename) {
                Ok(v) => v,
                Err(e) => {
                    panic!("File open or BufReader error: {}", e);
                }
            });
            input.fill_buf().expect("fail to fill buffer");
            fn_special(&mut input)
        }
        None => fn_run(),
    }
}
