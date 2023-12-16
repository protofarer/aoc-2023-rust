use crate::read_file_as_string;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    input: Option<String>,
}

pub fn parse_args_special<Fa, Fb>(fn_run: Fa, fn_special: Fb)
where
    Fa: Fn(),
    Fb: Fn(String),
{
    let args = Args::parse();

    let input_str: Option<String> = match args.input {
        Some(file_path) => match read_file_as_string(&file_path) {
            Ok(s) => {
                println!("Run w/ input file {}", file_path);
                Some(s)
            }
            Err(e) => {
                panic!("Error reading file: {}", e);
            }
        },
        None => {
            println!("Run w/o input file");
            None
        }
    };

    match input_str {
        Some(s) => fn_special(s),
        None => fn_run(),
    }
}
