fn parse_args() -> String {
    let mut args = std::env::args();
    args.next();

    let file = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file argument");
            std::process::exit(1);
        }
    };

    file
}

fn main() {
    let file = parse_args();

    if let Err(e) = hamming_corruptor::run(&file) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}
