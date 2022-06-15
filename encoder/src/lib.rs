pub fn encode(file: Vec<u8>) {
    println!("Got an array of bytes, they are {:?}", file);
    println!("Converting array of bytes into array of bits...");
    // let bits: Vec<bool> = Vec::with_capacity(file.len() * 8);
    // for
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::encode;
    #[test]
    fn it_works() {
        let file = fs::read_to_string("./examples/hello.txt").unwrap();
        encode(file.into_bytes());
    }
}
