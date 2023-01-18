use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "anthony",
    version = "0.1.0",
    about = "echo in Rust",
    long_about = None,
)]
struct Args {
    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,

    #[arg(help = "Input text", required = true, num_args = 1..=10)]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let ending = if args.omit_newline { "" } else { "\n" };
    print!("{}{}", args.text.join(" "), ending);
}
