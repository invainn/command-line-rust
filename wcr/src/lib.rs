use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{command, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short, long, default_value = "false")]
    words: bool,

    #[arg(short = 'c', long, conflicts_with = "chars", default_value = "false")]
    bytes: bool,

    #[arg(short = 'm', long, default_value = "false")]
    chars: bool,

    #[arg(short, long, default_value = "false")]
    lines: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let lines: Vec<_> = file
        .split('\n' as u8)
        .map(|v| String::from_utf8(v.unwrap()))
        .collect();

    for line in lines.into_iter() {
        let line = line.unwrap();

        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line.bytes().count() + 1; // counts end of line
        num_chars += line.chars().count() + 1; // counts end of line
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_output(args: &Args, counts: &FileInfo, descriptor: &str) -> String {
    format!(
        "{}{}{}{}",
        if args.lines == true {
            format!("{:>8}", counts.num_lines)
        } else {
            "".to_string()
        },
        if args.words == true {
            format!("{:>8}", counts.num_words)
        } else {
            "".to_string()
        },
        if args.bytes == true {
            format!("{:>8}", counts.num_bytes)
        } else if args.chars == true {
            format!("{:>8}", counts.num_chars)
        } else {
            "".to_string()
        },
        if descriptor == "-" {
            "".to_string()
        } else {
            format!(" {}", descriptor)
        },
    )
}

pub fn run() -> MyResult<()> {
    let mut args = Args::parse();

    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|v| v == &false)
    {
        args.words = true;
        args.bytes = true;
        args.lines = true;
    }

    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let counts = count(file)?;

                println!("{}", format_output(&args, &counts, filename));

                total.num_chars += counts.num_chars;
                total.num_words += counts.num_words;
                total.num_bytes += counts.num_bytes;
                total.num_lines += counts.num_lines;
            }
        }
    }

    if args.files.len() > 1 {
        println!("{}", format_output(&args, &total, "total"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::{count, FileInfo};

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));

        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}
