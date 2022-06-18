use bitvec::field::BitField;
use bitvec::prelude::AsBits;
use bitvec::prelude::Lsb0;
use rand::Rng;

pub fn corrupt(file: &[u8]) -> Vec<u8> {
    let chunk_size = file[8] as usize;
    let mut bits = (&file[9..]).as_bits::<Lsb0>().to_owned();

    for chunk in bits.chunks_mut(chunk_size) {
        let bit_index = rand::thread_rng().gen_range(0..chunk_size);

        let value = chunk[bit_index];
        chunk.set(bit_index, !value);
    }

    let mut corrupted_file: Vec<u8> = Vec::with_capacity(file.len() * 8);

    for i in 0..9 {
        corrupted_file.push(file[i]);
    }

    for byte in bits.chunks(8) {
        corrupted_file.push(byte.load());
    }

    corrupted_file
}

pub fn run(file: String) -> Result<(), Box<dyn std::error::Error>> {
    let og_file = std::fs::read(&file)?;
    let corrupted_file = corrupt(&og_file);

    std::fs::write(file, corrupted_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::corrupt;

    #[test]
    fn it_corrupts() {
        let file = format!("hamming{}{}Test", 0u8 as char, 16u8 as char).into_bytes();
        let corrupted_file = corrupt(&file);

        assert_ne!(file, corrupted_file);
    }
}
