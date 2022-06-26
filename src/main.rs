fn main() {
    if let Err(e) = ego::get_args().and_then(ego::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}