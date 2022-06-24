fn parse_args() -> (String, String) {
    let mut args = std::env::args();
    args.next();

    let file_in = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file_in argument");
            std::process::exit(1);
        }
    };

    let file_out = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file_out argument");
            std::process::exit(1);
        }
    };

    (file_in, file_out)
}

fn main() {
    let (file_in, file_out) = parse_args();

    if let Err(e) = hamming_decoder::run(&file_in, &file_out) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}
