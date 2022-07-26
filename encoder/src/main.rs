use hamming_encoder::HammingLevel;

fn parse_args() -> (String, String, HammingLevel) {
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

    let level = match args.next().as_deref() {
        Some("1") => HammingLevel::L1,
        Some("2") => HammingLevel::L2,
        Some("3") => HammingLevel::L3,
        Some(_) => {
            eprintln!("Invalid hamming level specified. Valid values are 1/2/3");
            std::process::exit(1);
        }
        None => {
            eprintln!("Did not supply hamming_level argument. Valid values are 1/2/3");
            std::process::exit(1);
        }
    };

    (file_in, file_out, level)
}

fn main() {
    let (file_in, file_out, level) = parse_args();

    if let Err(e) = hamming_encoder::run(&file_in, &file_out, level) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}
