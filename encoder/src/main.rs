fn parse_args() -> (String, String) {
    let mut args = std::env::args();
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file_path argument");
            std::process::exit(1);
        }
    };

    let output_file_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply output_file_path argument");
            std::process::exit(1);
        }
    };

    (file_path, output_file_path)
}

fn main() {
    let (file_path, output_file_path) = parse_args();

    if let Err(e) = hamming_encoder::run(file_path, output_file_path) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}
