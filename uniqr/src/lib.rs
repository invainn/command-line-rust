use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use clap::{command, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input file", value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    #[arg(help = "Output file", value_name = "OUT_FILE")]
    out_file: Option<String>,

    #[arg(help = "Show counts", short, long, default_value = "false")]
    count: bool,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();

    let mut file = open(&args.in_file).map_err(|e| format!("{}: {}", args.in_file, e))?;
    let mut out_file: Box<dyn Write> = match args.out_file {
        Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count != 0 {
            write!(
                out_file,
                "{}{}{}",
                if args.count {
                    format!("{:>4}", count)
                } else {
                    "".to_string()
                },
                if args.count { " " } else { "" },
                text
            )?;
        }

        Ok(())
    };

    let mut line = String::new();
    let mut last_line = String::new();
    let mut current_count = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim() != last_line.trim() {
            print(current_count, &last_line)?;

            last_line = line.clone();
            current_count = 0;
        }

        current_count += 1;
        line.clear();
    }

    print(current_count, &last_line)?;

    Ok(())
}
