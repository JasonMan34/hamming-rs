use hamming_corruptor;
use hamming_decoder;
use hamming_encoder;

fn parse_args() -> (String, String, String) {
    let mut args = std::env::args();
    args.next();

    let file_to_encode = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file_in argument");
            std::process::exit(1);
        }
    };

    let encoded_file = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply file_out argument");
            std::process::exit(1);
        }
    };

    let decoded_file = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Did not supply decoded_file argument");
            std::process::exit(1);
        }
    };

    (file_to_encode, encoded_file, decoded_file)
}

fn encode(file_to_encode: &str, encoded_file: &str) {
    if let Err(e) = hamming_encoder::run(file_to_encode, encoded_file) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}

fn corrupt(file_to_corrupt: &str) {
    if let Err(e) = hamming_corruptor::run(file_to_corrupt) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}

fn decode(file_to_decode: &str, decoded_file: &str) {
    if let Err(e) = hamming_decoder::run(file_to_decode, decoded_file) {
        eprintln!("Application error: {}", e);

        let paths = std::fs::read_dir("./").unwrap();

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        std::process::exit(1);
    }
}

fn main() {
    let (file_to_encode, encoded_file, decoded_file) = parse_args();

    encode(&file_to_encode, &encoded_file);
    corrupt(&encoded_file);
    decode(&encoded_file, &decoded_file);
}
