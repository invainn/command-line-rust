use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{command, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "head in Rust")]
struct Args {
    #[arg(id = "FILE", help = "Input file(s)", default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "lines",
        value_name = "LINES",
        help = "print NUM of lines",
        conflicts_with = "bytes",
        default_value = "10",
        value_parser = parse_lines,
    )]
    lines: usize,

    #[arg(
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        help = "print NUM of bytes",
        value_parser = parse_bytes,
    )]
    bytes: Option<usize>,
}

fn parse_bytes(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("illegal byte count -- {}", val)),
    }
}

fn parse_lines(val: &str) -> Result<usize, String> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("illegal line count -- {}", val)),
    }
}
#[test]
fn test_parse_lines() {
    let res = parse_lines("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_lines("foo");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!("illegal line count -- {}", "foo")
    );

    let res = parse_lines("0");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!("illegal line count -- {}", "0")
    );
}

#[test]
fn test_parse_bytes() {
    let res = parse_bytes("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_bytes("foo");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!("illegal byte count -- {}", "foo")
    );

    let res = parse_bytes("0");
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!("illegal byte count -- {}", "0")
    );
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();

    for (idx, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if args.files.len() > 1 {
                    println!("{}==> {} <==", if idx > 0 { "\n" } else { "" }, filename);
                }

                if let Some(bytes) = args.bytes {
                    // read bytes
                    let data: Result<Vec<_>, _> = file.bytes().take(bytes).collect();

                    print!("{}", String::from_utf8_lossy(&data?));
                } else {
                    // read lines
                    for line in file.split('\n' as u8).take(args.lines) {
                        println!("{}", String::from_utf8_lossy(&line?));
                    }
                }
            }
        }
    }

    Ok(())
}
