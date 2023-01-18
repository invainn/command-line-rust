use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{command, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "cat in Rust")]
struct Args {
    #[arg(id = "FILE", help = "Input file(s)")]
    files: Vec<String>,

    #[arg(
        short,
        long = "number",
        help = "Number lines",
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "Number nonblank lines")]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut count = 1;
                for line in file.lines().map(|line| line.unwrap()) {
                    if args.number_lines {
                        println!("{:>6}\t{}", count, &line);
                        count += 1;
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            println!("{:>6}\t{}", count, &line);
                            count += 1;
                        }
                    } else {
                        println!("{}", &line);
                    }
                }
            }
        }
    }

    Ok(())
}
