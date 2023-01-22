fn main() {
    if let Err(e) = wcr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
