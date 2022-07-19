use bitvec::field::BitField;
use bitvec::prelude::AsBits;
use bitvec::prelude::BitVec;
use bitvec::prelude::Lsb0;
use shared::parity_check;

/// Flip the appropriate bits in the bitvec to match the given parity
///
/// # Arguments
/// `bits` - A bitvec\
/// `parity` - A parity index to fix the bitvec with
///
/// # Examples
/// When given a parity of 13 (`1101`) - the function will flip the bits
/// in index 1, 4, and 8 (`0001`, `0100`, and `1000`)
///
/// # Panics
/// The function will panic if `parity` is equal to or larger than the length of `bits`
///
fn fix_parity(bits: &mut BitVec<u8>, parity: usize) {
    let mut index = 1;
    while index <= parity {
        if index & parity != 0 {
            bits.set(index, true);
        }

        index *= 2;
    }
}

/// Transform original chunk index to hamming chunk index
///
/// Maximum value of index is 11, as hamming codes make very little sense for original chunks
/// with a size bigger than 11
fn og_index_to_new_index(index: usize) -> usize {
    let index_list = [3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15];
    index_list[index]
}

/// Encodes a file using hamming codes
///
/// /// # Arguments
/// `file` - A u8 vector slice representing the file in bytes\
/// `final_chunk_size` - The **final** chunk size for each chunk in the encoded file.
/// Note that this must be a power of 2, and cannot be larger than 16, so realistically speaking this has to be 8 or 16
fn encode(file: &[u8], final_chunk_size: usize) -> Vec<u8> {
    println!("=============== ENCODING ===============");
    if (final_chunk_size as f64).log2().fract() != 0.0 {
        panic!("final_chunk_size must be a power of 2");
    }

    let chunk_size = ((final_chunk_size as f64) - (final_chunk_size as f64).log2() - 1.0) as usize;
    let chunks = file.as_bits::<Lsb0>().chunks(chunk_size);
    let chunks_count = chunks.clone().count();

    let mut encoded_bitvec: BitVec<u8, Lsb0> =
        BitVec::with_capacity(chunks_count * final_chunk_size / 8);

    let mut last_chunk_size = chunk_size;

    for (_, chunk) in chunks.enumerate() {
        let mut new_chunk: BitVec<u8, Lsb0> = BitVec::with_capacity(final_chunk_size);
        unsafe { new_chunk.set_len(final_chunk_size) }
        new_chunk.fill(false);

        for (bit_index, bit) in chunk.iter().enumerate() {
            new_chunk.set(og_index_to_new_index(bit_index), *bit);
        }

        let parity = parity_check(&new_chunk);
        fix_parity(&mut new_chunk, parity);

        for bit in new_chunk {
            encoded_bitvec.push(bit);
        }

        if chunk.len() != chunk_size {
            last_chunk_size = chunk.len();
        }
    }

    let mut encoded_file: Vec<u8> = Vec::with_capacity(encoded_bitvec.len() + 9);
    encoded_file.push(b'h');
    encoded_file.push(b'a');
    encoded_file.push(b'm');
    encoded_file.push(b'm');
    encoded_file.push(b'i');
    encoded_file.push(b'n');
    encoded_file.push(b'g');
    encoded_file.push(last_chunk_size as u8);
    encoded_file.push(final_chunk_size as u8);

    for byte in encoded_bitvec.chunks(8) {
        encoded_file.push(byte.load());
    }

    encoded_file
}

pub fn encode_4_1(file: &[u8]) -> Vec<u8> {
    encode(&file, 4)
}

pub fn encode_8_4(file: &[u8]) -> Vec<u8> {
    encode(&file, 8)
}

pub fn encode_16_11(file: &[u8]) -> Vec<u8> {
    encode(&file, 16)
}

/// The higher the level (1 being the highest),
/// the more resilient the encoded file will be to corruptions,
/// but it will also take more space
pub enum HammingLevel {
    /// Resilient for up to 100% original data corruption, takes 4 times more space
    L1,
    /// Resilient for up to 25% original data corruption, takes 2 times more space
    L2,
    /// Resilient for up to 10% original data corruption, takes 1.4 times more space
    L3,
}

pub fn run(
    file_in: &str,
    file_out: &str,
    level: HammingLevel,
) -> Result<(), Box<dyn std::error::Error>> {
    let og_file = std::fs::read_to_string(&file_in)?;

    let encoded_file = match level {
        HammingLevel::L1 => encode_4_1(og_file.as_bytes()),
        HammingLevel::L2 => encode_8_4(og_file.as_bytes()),
        HammingLevel::L3 => encode_16_11(og_file.as_bytes()),
    };

    std::fs::write(file_out, encoded_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::og_index_to_new_index;

    #[test]
    fn test_og_index_to_new_index() {
        assert_eq!(og_index_to_new_index(0), 3);
        assert_eq!(og_index_to_new_index(1), 5);
        assert_eq!(og_index_to_new_index(2), 6);
        assert_eq!(og_index_to_new_index(3), 7);
        assert_eq!(og_index_to_new_index(4), 9);
        assert_eq!(og_index_to_new_index(5), 10);
    }
}
