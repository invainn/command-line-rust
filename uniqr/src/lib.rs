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

    let mut last_line = String::new();
    let mut current_count = 0;

    let mut results: Vec<String> = vec![];

    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim() != last_line.trim() {
            dbg!(&line, &last_line);
            if current_count != 0 {
                results.push(format!(
                    "{}{}{}",
                    if args.count {
                        format!("{:>4}", current_count)
                    } else {
                        "".to_string()
                    },
                    if args.count { " " } else { "" },
                    last_line
                ));
                current_count = 0;
            }

            last_line = line.clone();
        }

        current_count += 1;
        line.clear();
    }

    if current_count != 0 {
        results.push(format!(
            "{}{}{}",
            if args.count {
                format!("{:>4}", current_count)
            } else {
                "".to_string()
            },
            if args.count { " " } else { "" },
            last_line
        ));
    }

    if results.len() > 0 {
        let mut buffer: Box<dyn Write> = match args.out_file {
            Some(filename) => Box::new(File::create(filename)?),
            None => Box::new(io::stdout()),
        };

        for result in results {
            buffer.write(result.as_bytes())?;
        }

        buffer.flush()?;
    }

    Ok(())
}
