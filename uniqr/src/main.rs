fn main() {
    if let Err(e) = uniqr::run() {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
