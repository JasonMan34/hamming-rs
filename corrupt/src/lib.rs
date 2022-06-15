use bitvec::field::BitField;
use bitvec::prelude::AsBits;
use bitvec::prelude::Lsb0;
use rand::Rng;

pub fn corrupt(file: &[u8]) -> Vec<u8> {
    let chunk_size = file[8] as usize;

    let mut bits = file.as_bits::<Lsb0>().to_owned();
    let chunks = &mut bits[72..].chunks_mut(chunk_size);
    let mut chunks_count = 0;

    for chunk in chunks {
        chunks_count = chunks_count + 1;
        let bit_index = rand::thread_rng().gen_range(0..chunk_size);

        let value = chunk[bit_index];
        chunk.set(bit_index, !value);
    }

    let mut corrupted_file: Vec<u8> = Vec::with_capacity(chunks_count * chunk_size / 8);

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
