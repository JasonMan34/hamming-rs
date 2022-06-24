use bitvec::field::BitField;
use bitvec::prelude::AsBits;
use bitvec::prelude::BitVec;
use bitvec::prelude::Lsb0;
use shared::parity_check;

/// Flip the bit with the parity index in the bitvec
fn fix_parity(bits: &mut BitVec<u8>, parity: usize) {
    let value = bits[parity];
    bits.set(parity, !value);
}

/// Decodes a hamming-encoded file
///
/// /// # Arguments
/// `file` - A u8 vector slice representing a hamming encoded file in bytes\
/// `should_fix` - Should corrupted data be fixed
pub fn decode(file: &[u8], should_fix: bool) -> Vec<u8> {
    let last_chunk_bitcount = file[7];
    let chunk_size = file[8];
    let file = &file[9..];

    let decoded_chunk_size = ((chunk_size as f64) - (chunk_size as f64).log2() - 1.0) as usize;
    let chunks = file.as_bits::<Lsb0>().chunks(chunk_size as usize);
    let chunks_count = chunks.clone().count();
    let decoded_file_bit_count =
        (chunks_count - 1) * decoded_chunk_size + (last_chunk_bitcount as usize);

    let mut decoded_bitvec: BitVec<u8, Lsb0> = BitVec::with_capacity(decoded_file_bit_count);

    // Iterate over hamming chunks
    for chunk in chunks {
        let mut chunk = BitVec::from(chunk);
        if should_fix {
            let parity = parity_check(&chunk);
            fix_parity(&mut chunk, parity);
        }

        // The first 3 bits never hold meaningful data so we might as well skip them when decoding
        for (bit_index, bit) in chunk.iter().enumerate().skip(3) {
            if (bit_index as f64).log2().fract() != 0.0 {
                decoded_bitvec.push(*bit);
            }
        }
    }

    // Remove padded bits
    for _ in 0..(decoded_chunk_size as u8 - last_chunk_bitcount) {
        decoded_bitvec.pop();
    }

    // Load the decoded bitvec into a file as bytes
    let mut decoded_file: Vec<u8> = Vec::with_capacity(decoded_bitvec.capacity());
    for byte in decoded_bitvec.chunks(8) {
        decoded_file.push(byte.load());
    }

    decoded_file
}

pub fn run(file_in: &str, file_out: &str) -> Result<(), Box<dyn std::error::Error>> {
    let og_file = std::fs::read(file_in)?;
    let decoded_file = decode(&og_file, true);
    let corrupted_file = decode(&og_file, false);
    let cor_t = &file_out[..file_out.len() - 4];
    let mut cor_t = String::from(cor_t);
    cor_t.push_str("_corrupted.txt");

    std::fs::write(cor_t, corrupted_file)?;
    std::fs::write(file_out, decoded_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
