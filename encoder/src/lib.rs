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
///
/// # Examples
/// When given a parity of 13 (`1101`) - the function will flip the bits
/// in index 1, 4, and 8 (`0001`, `0100`, and `1000`)
///
/// # Panics
/// The function will panic if `parity` is equal to or larger than the length of `bits`
///
pub fn encode(file: &[u8], final_chunk_size: usize) -> Vec<u8> {
    println!("=============== ENCODING ===============");
    // println!("File is made of {} bytes", file.len());
    if (final_chunk_size as f64).log2().fract() != 0.0 {
        panic!("final_chunk_size must be a power of 2");
    }

    let chunk_size = ((final_chunk_size as f64) - (final_chunk_size as f64).log2() - 1.0) as usize;

    println!("Chunk size is {}", chunk_size);
    println!("Final chunk size is {}", final_chunk_size);

    let chunks = file.as_bits::<Lsb0>().chunks(chunk_size);
    let chunks_count = chunks.clone().count();

    println!("Chunks count is {}", chunks_count);

    let mut encoded_bitvec: BitVec<u8, Lsb0> =
        BitVec::with_capacity(chunks_count * final_chunk_size / 8);

    let mut last_chunk_size = chunk_size;

    for (_, chunk) in chunks.enumerate() {
        // println!("Chunk #{} is: {}", chunk_index + 1, chunk);
        let mut new_chunk: BitVec<u8, Lsb0> = BitVec::with_capacity(final_chunk_size);
        unsafe { new_chunk.set_len(final_chunk_size) }
        new_chunk.fill(false);

        // let mut debug_bitvec: BitVec<u8, Lsb0> = BitVec::with_capacity(chunk_size);
        for (bit_index, bit) in chunk.iter().enumerate() {
            // debug_bitvec.push(*bit);
            new_chunk.set(og_index_to_new_index(bit_index), *bit);
        }
        // println!("Encoded chunk #{}: {}", chunk_index + 1, debug_bitvec);

        let parity = parity_check(&new_chunk);
        // println!("NEW chunk #{} is: {}", chunk_index + 1, new_chunk);
        fix_parity(&mut new_chunk, parity);
        // println!(
        //     "NEW chunk #{} AFTER PARITY FIXING is: {}",
        //     chunk_index + 1,
        //     new_chunk
        // );

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

pub fn encode_7_4(file: &[u8]) -> Vec<u8> {
    encode(&file, 4)
}

pub fn encode_15_11(file: &[u8]) -> Vec<u8> {
    encode(&file, 16)
}

pub fn run(file_in: &str, file_out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let og_file = std::fs::read_to_string(&file_in)?;
    let encoded_file = encode_7_4(og_file.as_bytes());
    // let encoded_file = encode_15_11(og_file.as_bytes());

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
