fn parse_args() -> String {
    let mut args = std::env::args();
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file_path argument");
            std::process::exit(1);
        }
    };

    file_path
}

fn main() {
    let file_path = parse_args();

    if let Err(e) = hamming_encoder::run(file_path) {
        eprintln!("Application error: {}", e);

        std::process::exit(1);
    }
}
